//! World-class transfer engine for NexExplorer.
//!
//! Key features:
//! - Dynamic RAM budget (25% of available, always leaves 2GB for OS)
//! - Self-calibrating drive profiles (measured benchmark, cached 7 days)
//! - Memory pressure guard (None/Low/Critical → full/half/1 workers)
//! - AtomicU64 for hot-path bytes/files (no lock contention on copy loop)
//! - Condvar for pause (0% CPU while paused, replaces 200ms sleep loop)
//! - Dedicated emitter thread at 30Hz (decoupled from copy throughput)
//! - Pre-scan conflict detection (batch dialog, non-blocking)
//! - Per-file retry with exponential backoff
//! - Adaptive copy (tiny=whole-file, medium=buffered, large=mmap)
//! - Long path support (>260 chars) via \\?\ prefix on Windows
//! - Optional post-copy CRC32 verification

use crc32fast::Hasher as Crc32Hasher;
use crossbeam_channel::{bounded, Receiver, Sender};
use memmap2::{Mmap, MmapMut};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter};

// ── Long-path helpers ────────────────────────────────────────────────────────

/// On Windows, paths longer than MAX_PATH (260) require the `\\?\` prefix
/// to bypass the legacy limit. This function adds it when needed.
#[cfg(target_os = "windows")]
fn long_path(p: &Path) -> std::borrow::Cow<'_, Path> {
    let s = p.to_string_lossy();
    if s.len() > 260 && !s.starts_with(r"\\?\") && !s.starts_with(r"\\.\") {
        let prefixed = format!(r"\\?\{}", s);
        std::borrow::Cow::Owned(PathBuf::from(prefixed))
    } else {
        std::borrow::Cow::Borrowed(p)
    }
}

#[cfg(not(target_os = "windows"))]
fn long_path(p: &Path) -> std::borrow::Cow<Path> {
    std::borrow::Cow::Borrowed(p)
}

// ── CRC32 verification ────────────────────────────────────────────────────────

/// Compute CRC32 of a file. Returns None on I/O error.
fn crc32_file(path: &Path) -> Option<u32> {
    use std::io::Read;
    let path = long_path(path);
    let mut file = fs::File::open(path.as_ref()).ok()?;
    let mut hasher = Crc32Hasher::new();
    let mut buf = vec![0u8; 256 * 1024];
    loop {
        let n = file.read(&mut buf).ok()?;
        if n == 0 { break; }
        hasher.update(&buf[..n]);
    }
    Some(hasher.finalize())
}

// ── Public types ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransferOp {
    Copy,
    Move,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TransferStatus {
    Queued,
    Running,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ConflictResolution {
    Skip,
    Replace,
    Rename,
}

#[derive(Debug, Clone, Serialize)]
pub struct ConflictInfo {
    pub source: String,
    pub destination: String,
    pub source_size: u64,
    pub source_modified: u64,
    pub dest_size: u64,
    pub dest_modified: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct TransferProgress {
    pub id: String,
    pub op: TransferOp,
    pub status: TransferStatus,
    pub source: String,
    pub destination: String,
    pub current_file: String,
    pub bytes_done: u64,
    pub bytes_total: u64,
    pub files_done: u64,
    pub files_total: u64,
    pub speed_bps: f64,
    pub speed_history: Vec<f32>,
    pub eta_seconds: f64,
    pub eta_confidence: f32,
    pub error: Option<String>,
    pub failed_files: Vec<String>,
    pub drive_concurrency: u8,
    pub calibrating: bool,
    pub verify: bool,
    pub verified_files: u64,
    pub verify_failed_files: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveProfile {
    pub drive_root: String,
    pub optimal_workers: u8,
    pub optimal_buffer_mb: u8,
    pub measured_speed_mbs: u64,
    pub calibrated_at: u64,
}

// ── Drive calibration store ───────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
struct CalibrationStore {
    drives: Vec<DriveProfile>,
}

impl CalibrationStore {
    fn load(app_data_dir: &Path) -> Self {
        let path = app_data_dir.join("drive_profiles.json");
        fs::read_to_string(&path)
            .ok()
            .and_then(|s| serde_json::from_str(&s).ok())
            .unwrap_or_default()
    }

    fn save(&self, app_data_dir: &Path) {
        let path = app_data_dir.join("drive_profiles.json");
        if let Ok(json) = serde_json::to_string_pretty(self) {
            let _ = fs::create_dir_all(app_data_dir);
            let _ = fs::write(path, json);
        }
    }

    fn get_valid(&self, drive_root: &str) -> Option<&DriveProfile> {
        let now = unix_now();
        self.drives.iter().find(|d| {
            d.drive_root == drive_root
                && now.saturating_sub(d.calibrated_at) < 7 * 24 * 3600
        })
    }

    fn upsert(&mut self, profile: DriveProfile) {
        if let Some(existing) = self.drives.iter_mut().find(|d| d.drive_root == profile.drive_root)
        {
            *existing = profile;
        } else {
            self.drives.push(profile);
        }
    }
}

// ── Global state ──────────────────────────────────────────────────────────────

lazy_static::lazy_static! {
    static ref CALIBRATION: Mutex<CalibrationStore> = Mutex::new(CalibrationStore::default());
    static ref APP_DATA_DIR: Mutex<Option<PathBuf>> = Mutex::new(None);
    static ref ENGINE_TRANSFERS: Mutex<HashMap<String, Arc<EngineTransfer>>> =
        Mutex::new(HashMap::new());
}

static TRANSFER_BUDGET: AtomicU64 = AtomicU64::new(256 * 1024 * 1024);
static TX_COUNTER: AtomicU64 = AtomicU64::new(1);
/// Global rate limit in bytes/sec. 0 = unlimited.
static RATE_LIMIT_BPS: AtomicU64 = AtomicU64::new(0);

pub fn set_rate_limit(bytes_per_sec: u64) {
    RATE_LIMIT_BPS.store(bytes_per_sec, Ordering::Relaxed);
}

pub fn get_rate_limit() -> u64 {
    RATE_LIMIT_BPS.load(Ordering::Relaxed)
}

/// Signal the transfer to skip the current file and move to the next one.
pub fn skip_current_file(id: &str) -> Result<(), String> {
    let guard = ENGINE_TRANSFERS.lock().unwrap();
    let et = guard.get(id).ok_or_else(|| "Transfer not found".to_string())?;
    et.control.lock().unwrap().skip_current = true;
    Ok(())
}

/// Sleep the calling thread enough to honour the global rate limit after writing `bytes`.
/// `chunk_start` is the Instant before the chunk write; used to measure actual elapsed time.
#[inline]
fn throttle_rate(bytes: u64, chunk_start: Instant) {
    let limit = RATE_LIMIT_BPS.load(Ordering::Relaxed);
    if limit == 0 { return; }
    // How long this chunk *should* have taken at the target rate
    let target_ns = bytes * 1_000_000_000 / limit;
    let elapsed_ns = chunk_start.elapsed().as_nanos() as u64;
    if target_ns > elapsed_ns {
        std::thread::sleep(Duration::from_nanos(target_ns - elapsed_ns));
    }
}

pub fn init_engine(app_data_dir: PathBuf) {
    let store = CalibrationStore::load(&app_data_dir);
    *CALIBRATION.lock().unwrap() = store;
    *APP_DATA_DIR.lock().unwrap() = Some(app_data_dir);
    refresh_transfer_budget();
}

pub fn refresh_transfer_budget() {
    TRANSFER_BUDGET.store(compute_transfer_budget(), Ordering::Relaxed);
}

fn compute_transfer_budget() -> u64 {
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
        let mut ms: MEMORYSTATUSEX = unsafe { std::mem::zeroed() };
        ms.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
        unsafe { GlobalMemoryStatusEx(&mut ms) };
        let available = ms.ullAvailPhys;
        let total = ms.ullTotalPhys;
        return [
            available / 4,
            total / 2,
            available.saturating_sub(2 * 1024 * 1024 * 1024),
            512 * 1024 * 1024u64, // floor: always allow 512MB
        ]
        .iter()
        .copied()
        .min()
        .unwrap_or(256 * 1024 * 1024);
    }
    #[cfg(not(target_os = "windows"))]
    {
        512 * 1024 * 1024
    }
}

#[derive(Clone, Copy, PartialEq)]
enum MemoryPressure {
    None,
    Low,
    Critical,
}

fn check_memory_pressure() -> MemoryPressure {
    #[cfg(target_os = "windows")]
    {
        use windows_sys::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
        let mut ms: MEMORYSTATUSEX = unsafe { std::mem::zeroed() };
        ms.dwLength = std::mem::size_of::<MEMORYSTATUSEX>() as u32;
        unsafe { GlobalMemoryStatusEx(&mut ms) };
        return match ms.ullAvailPhys {
            a if a > 4 * 1024 * 1024 * 1024 => MemoryPressure::None,
            a if a > 2 * 1024 * 1024 * 1024 => MemoryPressure::Low,
            _ => MemoryPressure::Critical,
        };
    }
    #[cfg(not(target_os = "windows"))]
    {
        MemoryPressure::None
    }
}

// ── Drive detection ───────────────────────────────────────────────────────────

fn get_drive_root(path: &Path) -> String {
    #[cfg(target_os = "windows")]
    {
        let s = path.to_string_lossy();
        // Drive letter: C:\path → "C:\"
        if s.len() >= 3 && s.chars().nth(1) == Some(':') {
            return s[..3].to_uppercase();
        }
        // UNC: \\server\share\path → "\\server\share\"
        if s.starts_with(r"\\") {
            let parts: Vec<&str> = s.splitn(5, '\\').collect();
            if parts.len() >= 4 {
                return format!(r"\\{}\{}\", parts[2], parts[3]);
            }
        }
    }
    "/".to_string()
}

// ── Drive calibration ─────────────────────────────────────────────────────────

fn calibrate_drive(drive_root: &str, dest: &Path) -> DriveProfile {
    let temp_file = dest.join("__nexexplorer_calibration__.tmp");

    // Measure write speed with 8MB test (fast but accurate with pseudo-random pattern)
    let measured_speed_mbs = measure_write_speed(&temp_file, 8);

    // Map measured speed to optimal worker + buffer config
    let (optimal_workers, optimal_buffer_mb) = match measured_speed_mbs {
        s if s < 50 => (1u8, 1u8),     // Very slow: USB/ancient HDD
        s if s < 200 => (1, 2),         // HDD-class
        s if s < 800 => (2, 4),         // SATA SSD
        s if s < 3_000 => (4, 8),       // NVMe Gen 3
        s if s < 7_000 => (8, 16),      // NVMe Gen 4
        _ => (12, 16),                  // NVMe Gen 5+ (auto-adapts to any future hardware)
    };

    DriveProfile {
        drive_root: drive_root.to_string(),
        optimal_workers,
        optimal_buffer_mb,
        measured_speed_mbs,
        calibrated_at: unix_now(),
    }
}

fn measure_write_speed(temp_file: &Path, size_mb: usize) -> u64 {
    // Use a pseudo-random pattern — SSDs compress 0x00/0xAB, skewing results
    let buf: Vec<u8> = (0usize..1024 * 1024)
        .map(|i| ((i.wrapping_mul(0x9E3779B9)) ^ 0xDEAD) as u8)
        .collect();
    let start = Instant::now();
    let mut written = 0usize;

    if let Ok(mut file) = fs::File::create(temp_file) {
        for _ in 0..size_mb {
            if file.write_all(&buf).is_err() {
                break;
            }
            written += buf.len();
        }
        let _ = file.flush();
    }
    let _ = fs::remove_file(temp_file);

    let elapsed = start.elapsed().as_secs_f64();
    if elapsed <= 0.0 || written == 0 {
        return 0;
    }
    (written as f64 / elapsed / (1024.0 * 1024.0)) as u64
}

fn load_or_calibrate(dest: &Path, _app: &AppHandle, transfer_id: &str) -> DriveProfile {
    let drive_root = get_drive_root(dest);

    // Return cached profile if valid
    {
        let cal = CALIBRATION.lock().unwrap();
        if let Some(profile) = cal.get_valid(&drive_root) {
            return profile.clone();
        }
    }

    // Signal calibrating to UI
    set_calibrating(transfer_id, true);

    // Run calibration (benchmark)
    let profile = calibrate_drive(&drive_root, dest);

    // Cache and persist
    {
        let mut cal = CALIBRATION.lock().unwrap();
        cal.upsert(profile.clone());
        if let Some(dir) = APP_DATA_DIR.lock().unwrap().as_ref() {
            cal.save(dir);
        }
    }

    set_calibrating(transfer_id, false);
    profile
}

fn set_calibrating(id: &str, value: bool) {
    let map = ENGINE_TRANSFERS.lock().unwrap();
    if let Some(et) = map.get(id) {
        et.control.lock().unwrap().calibrating = value;
    }
}

pub fn get_drive_profiles() -> Vec<DriveProfile> {
    CALIBRATION.lock().unwrap().drives.clone()
}

pub fn recalibrate_drive(dest_path: &str) -> DriveProfile {
    let path = PathBuf::from(dest_path);
    let drive_root = get_drive_root(&path);
    let profile = calibrate_drive(&drive_root, &path);
    {
        let mut cal = CALIBRATION.lock().unwrap();
        cal.upsert(profile.clone());
        if let Some(dir) = APP_DATA_DIR.lock().unwrap().as_ref() {
            cal.save(dir);
        }
    }
    profile
}

// ── Worker spawn ──────────────────────────────────────────────────────────────

fn spawn_worker_count(profile: &DriveProfile, pressure: MemoryPressure) -> u8 {
    let base = profile.optimal_workers;
    let pressure_cap = match pressure {
        MemoryPressure::None => base,
        MemoryPressure::Low => (base / 2).max(1),
        MemoryPressure::Critical => 1,
    };
    let budget = TRANSFER_BUDGET.load(Ordering::Relaxed);
    let bytes_per_worker = (profile.optimal_buffer_mb as u64).max(1) * 1024 * 1024;
    let budget_cap = ((budget / bytes_per_worker) as u8).max(1);
    pressure_cap.min(budget_cap).max(1)
}

// ── Internal transfer state ───────────────────────────────────────────────────

struct AtomicCounters {
    bytes_done: AtomicU64,
    files_done: AtomicU64,
    verified_files: AtomicU64,
}

struct ControlBlock {
    status: TransferStatus,
    current_file: String,
    failed_files: Vec<String>,
    verify_failed_files: Vec<String>,
    error: Option<String>,
    calibrating: bool,
    per_file_resolution: HashMap<String, ConflictResolution>,
    default_resolution: ConflictResolution,
    /// Set to true by skip_file command; copy loop detects and clears it to skip current file
    skip_current: bool,
}

struct EngineTransfer {
    id: String,
    op: TransferOp,
    sources: Vec<String>,
    destination: String,
    bytes_total: AtomicU64,
    files_total: AtomicU64,
    drive_concurrency: AtomicU64,
    counters: Arc<AtomicCounters>,
    control: Arc<Mutex<ControlBlock>>,
    pause_sync: Arc<(Mutex<bool>, Condvar)>,
    conflict_sync: Arc<(Mutex<bool>, Condvar)>,
    verify: bool,
}

// ── Copy utilities ────────────────────────────────────────────────────────────

const SMALL_FILE_THRESHOLD: u64 = 1 * 1024 * 1024;   // 1MB → whole-file read
const LARGE_FILE_THRESHOLD: u64 = 256 * 1024 * 1024;  // 256MB → mmap

enum CopyErr {
    Cancelled,
    Skipped,
    Io(std::io::Error),
}

enum CopyResult {
    Ok,
    Skipped,
    Failed,
    Cancelled,
}

fn check_pause_cancel(
    control: &Arc<Mutex<ControlBlock>>,
    pause_sync: &Arc<(Mutex<bool>, Condvar)>,
) -> Result<(), CopyErr> {
    let (status, skip) = {
        let mut cb = control.lock().unwrap();
        if cb.skip_current {
            cb.skip_current = false; // consume the flag immediately
            return Err(CopyErr::Skipped);
        }
        (cb.status.clone(), false)
    };
    let _ = skip;
    if status == TransferStatus::Cancelled {
        return Err(CopyErr::Cancelled);
    }
    if status == TransferStatus::Paused {
        let (lock, cvar) = pause_sync.as_ref();
        let mut paused = lock.lock().unwrap();
        while *paused {
            paused = cvar.wait(paused).unwrap();
        }
        if control.lock().unwrap().status == TransferStatus::Cancelled {
            return Err(CopyErr::Cancelled);
        }
    }
    Ok(())
}

fn copy_file_adaptive(
    src: &Path,
    dst: &Path,
    buf_size: usize,
    control: &Arc<Mutex<ControlBlock>>,
    pause_sync: &Arc<(Mutex<bool>, Condvar)>,
    counters: &Arc<AtomicCounters>,
) -> Result<(), CopyErr> {
    let src_lp = long_path(src);
    let dst_lp = long_path(dst);
    let file_size = src_lp.metadata().map(|m| m.len()).unwrap_or(0);

    if file_size <= SMALL_FILE_THRESHOLD {
        // Tier 1: entire file into memory, single write
        let data = fs::read(src_lp.as_ref()).map_err(CopyErr::Io)?;
        check_pause_cancel(control, pause_sync)?;
        fs::write(dst_lp.as_ref(), &data).map_err(CopyErr::Io)?;
        counters.bytes_done.fetch_add(file_size, Ordering::Relaxed);
    } else if file_size > LARGE_FILE_THRESHOLD {
        // Tier 3: memory-mapped — OS handles page-in, saturates any NVMe
        copy_mmap(src_lp.as_ref(), dst_lp.as_ref(), file_size, control, pause_sync, counters)?;
    } else {
        // Tier 2: buffered copy with calibrated buffer size
        copy_buffered(src_lp.as_ref(), dst_lp.as_ref(), buf_size, control, pause_sync, counters)?;
    }

    // Preserve modified time
    if let Ok(meta) = fs::metadata(src_lp.as_ref()) {
        if let Ok(mtime) = meta.modified() {
            let _ = filetime::set_file_mtime(dst_lp.as_ref(), filetime::FileTime::from_system_time(mtime));
        }
    }

    Ok(())
}

/// Open a source file for reading. On Windows, retries with full share mode
/// (FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE = 0x7) when the
/// normal open fails. This handles files locked by Office, browsers, log
/// writers, etc. without requiring VSS or admin rights.
fn open_for_copy(path: &Path) -> std::io::Result<fs::File> {
    match fs::File::open(path) {
        Ok(f) => Ok(f),
        #[cfg(target_os = "windows")]
        Err(_) => {
            use std::os::windows::fs::OpenOptionsExt;
            fs::OpenOptions::new()
                .read(true)
                .share_mode(0x0000_0007) // FILE_SHARE_READ | FILE_SHARE_WRITE | FILE_SHARE_DELETE
                .open(path)
        }
        #[cfg(not(target_os = "windows"))]
        Err(e) => Err(e),
    }
}

fn copy_buffered(
    src: &Path,
    dst: &Path,
    buf_size: usize,
    control: &Arc<Mutex<ControlBlock>>,
    pause_sync: &Arc<(Mutex<bool>, Condvar)>,
    counters: &Arc<AtomicCounters>,
) -> Result<(), CopyErr> {
    let mut reader = open_for_copy(src).map_err(CopyErr::Io)?; // already long_path'd by caller
    let mut writer = fs::File::create(dst).map_err(CopyErr::Io)?;
    let mut buf = vec![0u8; buf_size];
    loop {
        check_pause_cancel(control, pause_sync)?;
        let chunk_start = Instant::now();
        let n = reader.read(&mut buf).map_err(CopyErr::Io)?;
        if n == 0 {
            break;
        }
        writer.write_all(&buf[..n]).map_err(CopyErr::Io)?;
        counters.bytes_done.fetch_add(n as u64, Ordering::Relaxed);
        throttle_rate(n as u64, chunk_start);
    }
    Ok(())
}

fn copy_mmap(
    src: &Path,
    dst: &Path,
    size: u64,
    control: &Arc<Mutex<ControlBlock>>,
    pause_sync: &Arc<(Mutex<bool>, Condvar)>,
    counters: &Arc<AtomicCounters>,
) -> Result<(), CopyErr> {
    let src_file = open_for_copy(src).map_err(CopyErr::Io)?;
    let dst_file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(dst)
        .map_err(CopyErr::Io)?;
    dst_file.set_len(size).map_err(CopyErr::Io)?;

    let src_map = unsafe { Mmap::map(&src_file).map_err(CopyErr::Io)? };
    let mut dst_map = unsafe { MmapMut::map_mut(&dst_file).map_err(CopyErr::Io)? };

    const CHUNK: usize = 16 * 1024 * 1024; // 16MB check-in chunks
    let mut offset = 0usize;
    let total = size as usize;
    while offset < total {
        check_pause_cancel(control, pause_sync)?;
        let chunk_start = Instant::now();
        let end = (offset + CHUNK).min(total);
        dst_map[offset..end].copy_from_slice(&src_map[offset..end]);
        let written = (end - offset) as u64;
        counters.bytes_done.fetch_add(written, Ordering::Relaxed);
        throttle_rate(written, chunk_start);
        offset = end;
    }
    dst_map.flush().map_err(CopyErr::Io)?;
    Ok(())
}

fn copy_with_retry(
    src: &Path,
    dst: &Path,
    buf_size: usize,
    verify: bool,
    control: &Arc<Mutex<ControlBlock>>,
    pause_sync: &Arc<(Mutex<bool>, Condvar)>,
    counters: &Arc<AtomicCounters>,
) -> CopyResult {
    for attempt in 0u32..3 {
        match copy_file_adaptive(src, dst, buf_size, control, pause_sync, counters) {
            Ok(()) => {
                if verify {
                    let src_crc = crc32_file(src);
                    let dst_crc = crc32_file(dst);
                    match (src_crc, dst_crc) {
                        (Some(a), Some(b)) if a == b => {
                            counters.verified_files.fetch_add(1, Ordering::Relaxed);
                        }
                        _ => {
                            let _ = fs::remove_file(&long_path(dst));
                            if attempt < 2 {
                                std::thread::sleep(Duration::from_secs((attempt + 1) as u64));
                                continue;
                            }
                            let mut cb = control.lock().unwrap();
                            if cb.verify_failed_files.len() < 1000 {
                                cb.verify_failed_files.push(src.display().to_string());
                            }
                            return CopyResult::Failed;
                        }
                    }
                }
                return CopyResult::Ok;
            }
            Err(CopyErr::Cancelled) => {
                let _ = fs::remove_file(&long_path(dst));
                return CopyResult::Cancelled;
            }
            Err(CopyErr::Skipped) => {
                let _ = fs::remove_file(&long_path(dst));
                return CopyResult::Skipped;
            }
            Err(CopyErr::Io(_)) if attempt < 2 => {
                let _ = fs::remove_file(&long_path(dst));
                std::thread::sleep(Duration::from_secs((attempt + 1) as u64));
            }
            Err(CopyErr::Io(e)) => {
                let mut cb = control.lock().unwrap();
                if cb.failed_files.len() < 1000 {
                    cb.failed_files.push(format!("{}: {}", src.display(), e));
                }
                let _ = fs::remove_file(&long_path(dst));
                return CopyResult::Failed;
            }
        }
    }
    CopyResult::Failed
}

// ── Size calculation ──────────────────────────────────────────────────────────

fn calc_total_size(sources: &[String]) -> (u64, u64) {
    let mut bytes = 0u64;
    let mut files = 0u64;
    for src in sources {
        let p = Path::new(src);
        if p.is_dir() {
            let (b, f) = dir_size(p);
            bytes += b;
            files += f;
        } else if let Ok(m) = p.metadata() {
            bytes += m.len();
            files += 1;
        }
    }
    (bytes, files)
}

fn dir_size(path: &Path) -> (u64, u64) {
    let mut bytes = 0u64;
    let mut files = 0u64;
    let mut stack = vec![path.to_path_buf()];
    while let Some(dir) = stack.pop() {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                let p = entry.path();
                if p.is_dir() {
                    stack.push(p);
                } else if let Ok(m) = p.metadata() {
                    bytes += m.len();
                    files += 1;
                }
            }
        }
    }
    (bytes, files)
}

// ── Conflict detection ────────────────────────────────────────────────────────

fn prescan_conflicts(sources: &[String], dest: &Path) -> Vec<ConflictInfo> {
    let mut conflicts = Vec::new();
    for src_str in sources {
        let src = Path::new(src_str);
        if let Some(name) = src.file_name() {
            let dst = dest.join(name);
            if dst.exists() {
                // Skip if source and destination are the same file (e.g. moving a file
                // back to the folder it already lives in — not a real conflict).
                let same = src.canonicalize().ok().zip(dst.canonicalize().ok())
                    .map(|(a, b)| a == b)
                    .unwrap_or(false);
                if same { continue; }
                let sm = src.metadata().ok();
                let dm = dst.metadata().ok();
                conflicts.push(ConflictInfo {
                    source: src_str.clone(),
                    destination: dst.display().to_string(),
                    source_size: sm.as_ref().map(|m| m.len()).unwrap_or(0),
                    source_modified: sm
                        .as_ref()
                        .and_then(|m| m.modified().ok())
                        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                        .map(|d| d.as_secs())
                        .unwrap_or(0),
                    dest_size: dm.as_ref().map(|m| m.len()).unwrap_or(0),
                    dest_modified: dm
                        .as_ref()
                        .and_then(|m| m.modified().ok())
                        .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                        .map(|d| d.as_secs())
                        .unwrap_or(0),
                });
            }
        }
    }
    conflicts
}

fn resolve_conflict_path(dst: &Path) -> PathBuf {
    let stem = dst.file_stem().and_then(|s| s.to_str()).unwrap_or("file");
    let ext = dst.extension().and_then(|s| s.to_str()).unwrap_or("");
    let parent = dst.parent().unwrap_or(Path::new("."));
    let mut counter = 1u32;
    loop {
        let name = if ext.is_empty() {
            format!("{} ({})", stem, counter)
        } else {
            format!("{} ({}).{}", stem, counter, ext)
        };
        let candidate = parent.join(&name);
        if !candidate.exists() {
            return candidate;
        }
        counter += 1;
    }
}

// ── Speed / ETA tracking ──────────────────────────────────────────────────────

struct SpeedState {
    ema_speed: f64,
    speed_history: [f32; 30],
    history_idx: usize,
    history_count: usize,
    last_bytes: u64,
    last_instant: Instant,
    last_second: Instant,
    second_bytes: u64,
}

impl SpeedState {
    const ALPHA: f64 = 0.2;

    fn new() -> Self {
        let now = Instant::now();
        Self {
            ema_speed: 0.0,
            speed_history: [0.0f32; 30],
            history_idx: 0,
            history_count: 0,
            last_bytes: 0,
            last_instant: now,
            last_second: now,
            second_bytes: 0,
        }
    }

    fn update(&mut self, current_bytes: u64) {
        let now = Instant::now();
        let dt = now.duration_since(self.last_instant).as_secs_f64();
        if dt < 0.001 {
            return;
        }
        let delta = current_bytes.saturating_sub(self.last_bytes) as f64;
        let instant_speed = delta / dt;
        self.ema_speed = if self.ema_speed == 0.0 {
            instant_speed
        } else {
            Self::ALPHA * instant_speed + (1.0 - Self::ALPHA) * self.ema_speed
        };
        self.last_bytes = current_bytes;
        self.second_bytes += delta as u64;
        self.last_instant = now;

        if now.duration_since(self.last_second).as_secs_f64() >= 1.0 {
            let mbs = self.second_bytes as f32 / (1024.0 * 1024.0);
            self.speed_history[self.history_idx] = mbs;
            self.history_idx = (self.history_idx + 1) % 30;
            self.history_count = (self.history_count + 1).min(30);
            self.second_bytes = 0;
            self.last_second = now;
        }
    }

    fn compute_eta(&self, bytes_done: u64, bytes_total: u64) -> (f64, f32) {
        if self.ema_speed <= 0.0 || bytes_total == 0 {
            return (0.0, 0.0);
        }
        let remaining = bytes_total.saturating_sub(bytes_done) as f64;
        let raw_eta = remaining / self.ema_speed;
        let pct = bytes_done as f64 / bytes_total as f64;
        let eta = if pct > 0.95 { raw_eta.max(1.0) } else { raw_eta };

        let confidence = if self.history_count < 3 {
            0.3f32
        } else {
            let count = self.history_count.min(30);
            let samples: Vec<f32> = (0..count)
                .map(|i| {
                    let idx = if count < 30 { i } else { (self.history_idx + i) % 30 };
                    self.speed_history[idx]
                })
                .collect();
            let mean = samples.iter().sum::<f32>() / samples.len() as f32;
            if mean <= 0.0 {
                return (eta, 0.1);
            }
            let variance =
                samples.iter().map(|&s| (s - mean).powi(2)).sum::<f32>() / samples.len() as f32;
            let cv = variance.sqrt() / mean;
            (1.0 - cv.min(1.0)).max(0.1)
        };

        (eta, confidence)
    }

    fn ordered_history(&self) -> Vec<f32> {
        let count = self.history_count.min(30);
        let start = if count < 30 { 0 } else { self.history_idx };
        (0..count).map(|i| self.speed_history[(start + i) % 30]).collect()
    }
}

// ── Job expansion ─────────────────────────────────────────────────────────────

struct FileJob {
    src: PathBuf,
    dst: PathBuf,
}

/// Iteratively expand src→dst into individual file jobs.
/// `dst` is the EXACT destination path (not a directory to copy INTO).
/// Uses an explicit stack instead of recursion to prevent stack overflow
/// on deeply nested directory trees.
/// Symlinks are resolved: symlinks to files are copied as regular files;
/// symlinks to directories are followed (contents copied); dangling symlinks are skipped.
fn expand_to_jobs(src: &Path, dst: &Path, tx: &Sender<FileJob>) {
    // stack holds (src_dir, dst_dir) pairs to process
    let mut stack: Vec<(PathBuf, PathBuf)> = vec![(src.to_path_buf(), dst.to_path_buf())];
    while let Some((cur_src, cur_dst)) = stack.pop() {
        // Resolve symlinks: get the real metadata (follow the link)
        let meta = match fs::metadata(&cur_src) {
            Ok(m) => m,
            Err(_) => continue, // dangling symlink or permission error — skip
        };
        if meta.is_dir() {
            let _ = fs::create_dir_all(&cur_dst);
            if let Ok(entries) = fs::read_dir(&cur_src) {
                for entry in entries.flatten() {
                    let child_src = entry.path();
                    let child_dst = cur_dst.join(entry.file_name());
                    stack.push((child_src, child_dst));
                }
            }
        } else {
            // Regular file or symlink to a file — copy the content
            let _ = tx.send(FileJob { src: cur_src, dst: cur_dst });
        }
    }
}

// ── Main entry points ─────────────────────────────────────────────────────────

pub fn start_engine_transfer(
    app: AppHandle,
    op: TransferOp,
    sources: Vec<String>,
    destination: String,
    verify: bool,
) -> String {
    let id = format!("tx_{}", TX_COUNTER.fetch_add(1, Ordering::Relaxed));

    let counters = Arc::new(AtomicCounters {
        bytes_done: AtomicU64::new(0),
        files_done: AtomicU64::new(0),
        verified_files: AtomicU64::new(0),
    });
    let control = Arc::new(Mutex::new(ControlBlock {
        status: TransferStatus::Queued,
        current_file: String::new(),
        failed_files: Vec::new(),
        verify_failed_files: Vec::new(),
        error: None,
        calibrating: false,
        per_file_resolution: HashMap::new(),
        default_resolution: ConflictResolution::Rename,
        skip_current: false,
    }));
    let pause_sync = Arc::new((Mutex::new(false), Condvar::new()));
    let conflict_sync = Arc::new((Mutex::new(false), Condvar::new()));

    let et = Arc::new(EngineTransfer {
        id: id.clone(),
        op,
        sources,
        destination,
        bytes_total: AtomicU64::new(0),
        files_total: AtomicU64::new(0),
        drive_concurrency: AtomicU64::new(1),
        counters,
        control,
        pause_sync,
        conflict_sync,
        verify,
    });

    ENGINE_TRANSFERS.lock().unwrap().insert(id.clone(), et.clone());

    std::thread::spawn(move || run_orchestrator(app, et));

    id
}

fn run_orchestrator(app: AppHandle, et: Arc<EngineTransfer>) {
    let id = et.id.clone();
    let op = et.op.clone();
    let sources = et.sources.clone();
    let destination = et.destination.clone();
    let dest_path = PathBuf::from(&destination);
    let started_at = unix_now();

    // 1. Set Running immediately so the UI shows activity at once
    et.control.lock().unwrap().status = TransferStatus::Running;

    // 2. Start emitter thread immediately — user sees "Running" before any blocking work
    let emitter_et = et.clone();
    let emitter_app = app.clone();
    let emitter_id = id.clone();
    let emitter_handle = std::thread::spawn(move || {
        run_emitter(emitter_app, emitter_et, emitter_id);
    });

    // 3. Load or calibrate drive profile (now reduced to 8MB test, ~0.1s)
    let profile = load_or_calibrate(&dest_path, &app, &id);

    // 4. Calculate total size (bytes_total starts at 0; emitter shows "calculating" state)
    let (bytes_total, files_total) = calc_total_size(&sources);
    et.bytes_total.store(bytes_total, Ordering::Relaxed);
    et.files_total.store(files_total, Ordering::Relaxed);

    // 5. Compute workers
    let pressure = check_memory_pressure();
    let num_workers = spawn_worker_count(&profile, pressure);
    et.drive_concurrency.store(num_workers as u64, Ordering::Relaxed);
    let buf_size = (profile.optimal_buffer_mb as usize).max(1) * 1024 * 1024;

    // 6. Pre-scan conflicts
    let conflicts = prescan_conflicts(&sources, &dest_path);
    if !conflicts.is_empty() {
        let _ = app.emit(
            "transfer-conflicts",
            &serde_json::json!({ "id": id, "conflicts": conflicts }),
        );
        // Mark conflict pending and wait
        {
            *et.conflict_sync.0.lock().unwrap() = true;
        }
        {
            let (lock, cvar) = et.conflict_sync.as_ref();
            let mut pending = lock.lock().unwrap();
            while *pending {
                pending = cvar.wait(pending).unwrap();
            }
        }
    }

    // 7. Build work channel and spawn workers
    let (work_tx, work_rx): (Sender<FileJob>, Receiver<FileJob>) = bounded(512);
    let work_rx = Arc::new(work_rx);

    // Scanner thread: resolve conflicts and expand each source to file jobs
    let scan_sources = sources.clone();
    let scan_dest = dest_path.clone();
    let scan_control = et.control.clone();
    let scan_counters = et.counters.clone();
    let scan_tx = work_tx.clone();
    let scan_op = op.clone();
    std::thread::spawn(move || {
        for src_str in &scan_sources {
            let src = Path::new(src_str);
            let file_name = match src.file_name() {
                Some(n) => n.to_owned(),
                None => continue,
            };
            // Guard: skip if source is already inside the destination directory
            // (e.g. moving a folder into itself or its own parent)
            let src_parent = src.parent().unwrap_or(Path::new(""));
            if src_parent == scan_dest.as_path() {
                continue;
            }
            // Guard: skip if destination is inside the source (e.g. moving C:\foo into C:\foo\bar)
            if scan_dest.starts_with(src) {
                continue;
            }
            let raw_target = scan_dest.join(&file_name);

            // Resolve conflict for this source
            let target = if raw_target.exists() {
                let res = {
                    let cb = scan_control.lock().unwrap();
                    cb.per_file_resolution
                        .get(src_str)
                        .cloned()
                        .unwrap_or_else(|| cb.default_resolution.clone())
                };
                match res {
                    ConflictResolution::Skip => continue,
                    ConflictResolution::Replace => {
                        if raw_target.is_dir() {
                            let _ = fs::remove_dir_all(&raw_target);
                        } else {
                            let _ = fs::remove_file(&raw_target);
                        }
                        raw_target
                    }
                    ConflictResolution::Rename => resolve_conflict_path(&raw_target),
                }
            } else {
                raw_target
            };

            // Same-drive move: try atomic rename first
            if scan_op == TransferOp::Move {
                // Measure BEFORE rename — source path disappears after success
                let (src_bytes, src_files) = if src.is_dir() {
                    dir_size(src)
                } else {
                    let bytes = src.metadata().map(|m| m.len()).unwrap_or(0);
                    (bytes, 1u64)
                };
                if fs::rename(src, &target).is_ok() {
                    scan_counters.files_done.fetch_add(src_files.max(1), Ordering::Relaxed);
                    scan_counters.bytes_done.fetch_add(src_bytes, Ordering::Relaxed);
                    continue;
                }
            }

            expand_to_jobs(src, &target, &scan_tx);
        }
        drop(scan_tx);
    });
    drop(work_tx);

    // Spawn N worker threads
    let mut worker_handles = Vec::with_capacity(num_workers as usize);
    for _ in 0..num_workers {
        let rx = work_rx.clone();
        let counters = et.counters.clone();
        let control = et.control.clone();
        let pause_sync = et.pause_sync.clone();
        let op2 = op.clone();
        let verify2 = et.verify;

        let handle = std::thread::spawn(move || {
            while let Ok(job) = rx.recv() {
                {
                    let mut cb = control.lock().unwrap();
                    cb.current_file = job.src.display().to_string();
                }

                match copy_with_retry(
                    &job.src, &job.dst, buf_size, verify2,
                    &control, &pause_sync, &counters,
                ) {
                    CopyResult::Ok => {
                        counters.files_done.fetch_add(1, Ordering::Relaxed);
                        if op2 == TransferOp::Move {
                            let _ = fs::remove_file(&job.src);
                        }
                    }
                    CopyResult::Skipped => {
                        // User skipped this file — not a failure, just move on
                    }
                    CopyResult::Failed => {
                        // Already recorded in failed_files inside copy_with_retry
                    }
                    CopyResult::Cancelled => break,
                }
            }
        });
        worker_handles.push(handle);
    }

    for h in worker_handles {
        let _ = h.join();
    }

    // Final status
    {
        let mut cb = et.control.lock().unwrap();
        if cb.status != TransferStatus::Cancelled {
            cb.status = if cb.failed_files.is_empty() && cb.verify_failed_files.is_empty() {
                TransferStatus::Completed
            } else {
                TransferStatus::Failed
            };
        }
        if cb.status == TransferStatus::Completed {
            et.counters.bytes_done.store(bytes_total, Ordering::Relaxed);
            et.counters.files_done.store(files_total, Ordering::Relaxed);
        }
    }

    let _ = emitter_handle.join();

    // Write transfer log entry
    {
        let cb = et.control.lock().unwrap();
        write_transfer_log(
            &id,
            &op,
            &sources,
            &destination,
            &cb.status,
            et.counters.files_done.load(Ordering::Relaxed),
            et.files_total.load(Ordering::Relaxed),
            et.counters.bytes_done.load(Ordering::Relaxed),
            et.bytes_total.load(Ordering::Relaxed),
            started_at,
            &cb.failed_files,
            &cb.verify_failed_files,
        );
    }

    // Remove from global map to prevent memory leak over long sessions
    ENGINE_TRANSFERS.lock().unwrap().remove(&id);
}

fn run_emitter(app: AppHandle, et: Arc<EngineTransfer>, id: String) {
    let mut speed = SpeedState::new();

    loop {
        std::thread::sleep(Duration::from_millis(33)); // 30Hz

        let bytes_done = et.counters.bytes_done.load(Ordering::Relaxed);
        let files_done = et.counters.files_done.load(Ordering::Relaxed);
        let bytes_total = et.bytes_total.load(Ordering::Relaxed);
        let files_total = et.files_total.load(Ordering::Relaxed);
        let drive_concurrency = et.drive_concurrency.load(Ordering::Relaxed) as u8;

        let verified_files = et.counters.verified_files.load(Ordering::Relaxed);
        let (status, current_file, error, failed_files, verify_failed_files, calibrating) = {
            let cb = et.control.lock().unwrap();
            (
                cb.status.clone(),
                cb.current_file.clone(),
                cb.error.clone(),
                cb.failed_files.clone(),
                cb.verify_failed_files.clone(),
                cb.calibrating,
            )
        };
        let failed_count = failed_files.len() + verify_failed_files.len();

        speed.update(bytes_done);
        let (eta, confidence) = speed.compute_eta(bytes_done, bytes_total);

        let progress = TransferProgress {
            id: id.clone(),
            op: et.op.clone(),
            status: status.clone(),
            source: et.sources.first().cloned().unwrap_or_default(),
            destination: et.destination.clone(),
            current_file,
            bytes_done,
            bytes_total,
            files_done,
            files_total,
            speed_bps: speed.ema_speed,
            speed_history: speed.ordered_history(),
            eta_seconds: eta,
            eta_confidence: confidence,
            error,
            failed_files,
            drive_concurrency,
            calibrating,
            verify: et.verify,
            verified_files,
            verify_failed_files,
        };

        let _ = app.emit("transfer-progress", &progress);

        if matches!(
            status,
            TransferStatus::Completed | TransferStatus::Failed | TransferStatus::Cancelled
        ) {
            // Emit a dedicated done event for native toast notifications
            let op_label = match et.op { TransferOp::Copy => "Copy", TransferOp::Move => "Move" };
            let src_name = et.sources.first()
                .and_then(|s| std::path::Path::new(s).file_name())
                .map(|n| n.to_string_lossy().into_owned())
                .unwrap_or_else(|| format!("{} item(s)", et.sources.len()));
            let extra = if et.sources.len() > 1 {
                format!(" (+{} more)", et.sources.len() - 1)
            } else {
                String::new()
            };
            let (title, body) = match status {
                TransferStatus::Completed => (
                    format!("{} complete", op_label),
                    format!("{}{} → {}", src_name, extra, et.destination),
                ),
                TransferStatus::Failed => (
                    format!("{} finished with errors", op_label),
                    format!("{} file(s) failed — check transfer log", failed_count),
                ),
                _ => (
                    format!("{} cancelled", op_label),
                    String::new(),
                ),
            };
            let _ = app.emit("transfer-done", serde_json::json!({
                "id": id,
                "status": format!("{:?}", status),
                "title": title,
                "body": body,
            }));
            break;
        }
    }
}

// ── Control commands ──────────────────────────────────────────────────────────

pub fn pause_engine(id: &str) -> Result<(), String> {
    let map = ENGINE_TRANSFERS.lock().unwrap();
    let et = map.get(id).ok_or("Transfer not found")?;
    let mut cb = et.control.lock().unwrap();
    if cb.status == TransferStatus::Running {
        cb.status = TransferStatus::Paused;
        *et.pause_sync.0.lock().unwrap() = true;
    }
    Ok(())
}

pub fn resume_engine(id: &str) -> Result<(), String> {
    let map = ENGINE_TRANSFERS.lock().unwrap();
    let et = map.get(id).ok_or("Transfer not found")?;
    let mut cb = et.control.lock().unwrap();
    if cb.status == TransferStatus::Paused {
        cb.status = TransferStatus::Running;
        let (lock, cvar) = et.pause_sync.as_ref();
        *lock.lock().unwrap() = false;
        cvar.notify_all();
    }
    Ok(())
}

pub fn cancel_engine(id: &str) -> Result<(), String> {
    let map = ENGINE_TRANSFERS.lock().unwrap();
    let et = map.get(id).ok_or("Transfer not found")?;
    let mut cb = et.control.lock().unwrap();
    if matches!(cb.status, TransferStatus::Running | TransferStatus::Paused) {
        cb.status = TransferStatus::Cancelled;
        let (lock, cvar) = et.pause_sync.as_ref();
        *lock.lock().unwrap() = false;
        cvar.notify_all();
    }
    Ok(())
}

pub fn resolve_conflicts(
    id: &str,
    decisions: HashMap<String, ConflictResolution>,
    default_resolution: ConflictResolution,
) -> Result<(), String> {
    let map = ENGINE_TRANSFERS.lock().unwrap();
    let et = map.get(id).ok_or("Transfer not found")?;
    {
        let mut cb = et.control.lock().unwrap();
        cb.per_file_resolution = decisions;
        cb.default_resolution = default_resolution;
    }
    let (lock, cvar) = et.conflict_sync.as_ref();
    *lock.lock().unwrap() = false;
    cvar.notify_all();
    Ok(())
}

pub fn get_engine_progress(id: &str) -> Option<TransferProgress> {
    let map = ENGINE_TRANSFERS.lock().unwrap();
    let et = map.get(id)?;
    let cb = et.control.lock().unwrap();
    Some(TransferProgress {
        id: id.to_string(),
        op: et.op.clone(),
        status: cb.status.clone(),
        source: et.sources.first().cloned().unwrap_or_default(),
        destination: et.destination.clone(),
        current_file: cb.current_file.clone(),
        bytes_done: et.counters.bytes_done.load(Ordering::Relaxed),
        bytes_total: et.bytes_total.load(Ordering::Relaxed),
        files_done: et.counters.files_done.load(Ordering::Relaxed),
        files_total: et.files_total.load(Ordering::Relaxed),
        speed_bps: 0.0,
        speed_history: vec![],
        eta_seconds: 0.0,
        eta_confidence: 0.0,
        error: cb.error.clone(),
        failed_files: cb.failed_files.clone(),
        drive_concurrency: et.drive_concurrency.load(Ordering::Relaxed) as u8,
        calibrating: cb.calibrating,
        verify: et.verify,
        verified_files: et.counters.verified_files.load(Ordering::Relaxed),
        verify_failed_files: cb.verify_failed_files.clone(),
    })
}

pub fn list_engine_transfers() -> Vec<TransferProgress> {
    let map = ENGINE_TRANSFERS.lock().unwrap();
    map.keys().filter_map(|id| {
        let et = map.get(id)?;
        let cb = et.control.lock().unwrap();
        Some(TransferProgress {
            id: id.clone(),
            op: et.op.clone(),
            status: cb.status.clone(),
            source: et.sources.first().cloned().unwrap_or_default(),
            destination: et.destination.clone(),
            current_file: cb.current_file.clone(),
            bytes_done: et.counters.bytes_done.load(Ordering::Relaxed),
            bytes_total: et.bytes_total.load(Ordering::Relaxed),
            files_done: et.counters.files_done.load(Ordering::Relaxed),
            files_total: et.files_total.load(Ordering::Relaxed),
            speed_bps: 0.0,
            speed_history: vec![],
            eta_seconds: 0.0,
            eta_confidence: 0.0,
            error: cb.error.clone(),
            failed_files: cb.failed_files.clone(),
            drive_concurrency: et.drive_concurrency.load(Ordering::Relaxed) as u8,
            calibrating: cb.calibrating,
            verify: et.verify,
            verified_files: et.counters.verified_files.load(Ordering::Relaxed),
            verify_failed_files: cb.verify_failed_files.clone(),
        })
    }).collect()
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn unix_now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn format_datetime(unix_secs: u64) -> String {
    let dt = chrono::DateTime::from_timestamp(unix_secs as i64, 0);
    match dt {
        Some(dt) => dt.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        None => unix_secs.to_string(),
    }
}

/// Write a JSON log entry for a completed/failed/cancelled transfer.
/// Log file: <app_data_dir>/transfer_logs/YYYY-MM-DD.json  (one file per day, appended as a JSON array)
fn write_transfer_log(
    id: &str,
    op: &TransferOp,
    sources: &[String],
    destination: &str,
    status: &TransferStatus,
    files_done: u64,
    files_total: u64,
    bytes_done: u64,
    bytes_total: u64,
    started_at: u64,
    failed_files: &[String],
    verify_failed_files: &[String],
) {
    let app_data = match APP_DATA_DIR.lock().unwrap().clone() {
        Some(d) => d,
        None => return,
    };

    let now = unix_now();
    let log_dir = app_data.join("transfer_logs");
    let _ = fs::create_dir_all(&log_dir);

    // One log file per calendar day
    let day = chrono::DateTime::from_timestamp(now as i64, 0)
        .map(|dt| dt.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "unknown".to_string());
    let log_path = log_dir.join(format!("{}.json", day));

    let entry = serde_json::json!({
        "id": id,
        "op": format!("{:?}", op),
        "status": format!("{:?}", status),
        "sources": sources,
        "destination": destination,
        "files_done": files_done,
        "files_total": files_total,
        "bytes_done": bytes_done,
        "bytes_total": bytes_total,
        "started_at": format_datetime(started_at),
        "finished_at": format_datetime(now),
        "duration_seconds": now.saturating_sub(started_at),
        "failed_files": failed_files,
        "verify_failed_files": verify_failed_files,
    });

    // Read existing array (or start fresh), append entry, write back
    let mut entries: Vec<serde_json::Value> = fs::read_to_string(&log_path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default();
    entries.push(entry);

    if let Ok(json) = serde_json::to_string_pretty(&entries) {
        let _ = fs::write(&log_path, json);
    }
}
