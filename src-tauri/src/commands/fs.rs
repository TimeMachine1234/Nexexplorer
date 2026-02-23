use serde::Serialize;
use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;

#[derive(Debug, Serialize)]
pub struct FileEntry {
    pub name: String,
    pub is_dir: bool,
    pub size: u64,
    pub modified: String,
    pub extension: String,
    pub is_hidden: bool,
}

#[derive(Debug, Serialize)]
pub struct DriveInfo {
    pub letter: String,
    pub label: String,
    pub total_space: u64,
    pub free_space: u64,
}

#[tauri::command]
pub fn list_directory(path: String) -> Result<Vec<FileEntry>, String> {
    let dir_path = Path::new(&path);

    if !dir_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    if !dir_path.is_dir() {
        return Err(format!("Path is not a directory: {}", path));
    }

    let mut entries = Vec::new();

    let read_dir = fs::read_dir(dir_path).map_err(|e| format!("Cannot read directory: {}", e))?;

    for entry_result in read_dir {
        let entry = match entry_result {
            Ok(e) => e,
            Err(_) => continue,
        };

        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let name = entry.file_name().to_string_lossy().to_string();

        let modified = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| {
                // Convert to ISO 8601 string
                let secs = d.as_secs() as i64;
                let naive = chrono_lite_timestamp(secs);
                naive
            })
            .unwrap_or_default();

        let extension = if metadata.is_dir() {
            String::new()
        } else {
            entry
                .path()
                .extension()
                .map(|e| format!(".{}", e.to_string_lossy()))
                .unwrap_or_default()
        };

        let is_hidden = is_hidden_file(&entry, &name);

        entries.push(FileEntry {
            name,
            is_dir: metadata.is_dir(),
            size: metadata.len(),
            modified,
            extension,
            is_hidden,
        });
    }

    Ok(entries)
}

/// Simple timestamp to ISO-ish string without pulling in chrono crate
fn chrono_lite_timestamp(unix_secs: i64) -> String {
    // Calculate date components from unix timestamp
    let secs_per_day: i64 = 86400;
    let mut days = unix_secs / secs_per_day;
    let day_secs = (unix_secs % secs_per_day) as u32;
    let hours = day_secs / 3600;
    let minutes = (day_secs % 3600) / 60;

    // Days since 1970-01-01
    let mut year: i32 = 1970;
    loop {
        let days_in_year = if is_leap_year(year) { 366 } else { 365 };
        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        year += 1;
    }

    let month_days = if is_leap_year(year) {
        [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    } else {
        [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
    };

    let mut month: u32 = 1;
    for &md in &month_days {
        if days < md {
            break;
        }
        days -= md;
        month += 1;
    }
    let day = days + 1;

    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:00",
        year, month, day, hours, minutes
    )
}

fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

#[tauri::command]
pub fn list_drives() -> Result<Vec<DriveInfo>, String> {
    let mut drives = Vec::new();

    // Check common drive letters on Windows
    for letter in b'A'..=b'Z' {
        let drive_path = format!("{}:\\", letter as char);
        let path = Path::new(&drive_path);
        if path.exists() {
            let label = get_volume_label(&drive_path);
            let (total, free) = get_disk_space(&drive_path);
            drives.push(DriveInfo {
                letter: (letter as char).to_string(),
                label,
                total_space: total,
                free_space: free,
            });
        }
    }

    Ok(drives)
}

fn get_volume_label(drive_path: &str) -> String {
    // Try to read volume label — fallback to "Local Disk"
    let label_path = format!("{}", drive_path);
    match fs::read_dir(&label_path) {
        Ok(_) => "Local Disk".to_string(),
        Err(_) => "Drive".to_string(),
    }
}

#[cfg(target_os = "windows")]
fn get_disk_space(drive_path: &str) -> (u64, u64) {
    use std::ffi::OsStr;
    use std::os::windows::ffi::OsStrExt;

    extern "system" {
        fn GetDiskFreeSpaceExW(
            lpDirectoryName: *const u16,
            lpFreeBytesAvailableToCaller: *mut u64,
            lpTotalNumberOfBytes: *mut u64,
            lpTotalNumberOfFreeBytes: *mut u64,
        ) -> i32;
    }

    let wide: Vec<u16> = OsStr::new(drive_path)
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let mut free_available: u64 = 0;
    let mut total: u64 = 0;
    let mut total_free: u64 = 0;

    unsafe {
        let result = GetDiskFreeSpaceExW(
            wide.as_ptr(),
            &mut free_available,
            &mut total,
            &mut total_free,
        );
        if result != 0 {
            return (total, free_available);
        }
    }

    (0, 0)
}

#[cfg(not(target_os = "windows"))]
fn get_disk_space(_drive_path: &str) -> (u64, u64) {
    (0, 0)
}

#[cfg(target_os = "windows")]
fn is_hidden_file(entry: &fs::DirEntry, name: &str) -> bool {
    use std::os::windows::fs::MetadataExt;
    if name.starts_with('.') {
        return true;
    }
    if let Ok(meta) = entry.metadata() {
        // FILE_ATTRIBUTE_HIDDEN = 0x2
        return (meta.file_attributes() & 0x2) != 0;
    }
    false
}

#[cfg(not(target_os = "windows"))]
fn is_hidden_file(_entry: &fs::DirEntry, name: &str) -> bool {
    name.starts_with('.')
}

#[tauri::command]
pub fn open_file(path: String) -> Result<(), String> {
    use std::process::Command;

    let file_path = Path::new(&path);
    if !file_path.exists() {
        return Err(format!("File does not exist: {}", path));
    }

    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "start", "", &path])
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open file: {}", e))?;
    }

    Ok(())
}

#[tauri::command]
pub fn copy_items(sources: Vec<String>, destination: String) -> Result<(), String> {
    let dest_path = Path::new(&destination);
    if !dest_path.is_dir() {
        return Err(format!("Destination is not a directory: {}", destination));
    }

    for source in &sources {
        let src = Path::new(source);
        if !src.exists() {
            return Err(format!("Source does not exist: {}", source));
        }
        let file_name = src.file_name().ok_or("Invalid source path")?;
        let target = dest_path.join(file_name);

        if src.is_dir() {
            copy_dir_recursive(src, &target)
                .map_err(|e| format!("Failed to copy directory {}: {}", source, e))?;
        } else {
            fs::copy(src, &target)
                .map_err(|e| format!("Failed to copy {}: {}", source, e))?;
        }
    }
    Ok(())
}

fn copy_dir_recursive(src: &Path, dst: &Path) -> std::io::Result<()> {
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir_recursive(&src_path, &dst_path)?;
        } else {
            fs::copy(&src_path, &dst_path)?;
        }
    }
    Ok(())
}
