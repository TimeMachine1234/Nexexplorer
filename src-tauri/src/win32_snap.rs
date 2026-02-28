/// Win32 WndProc subclassing: intercepts WM_NCHITTEST and returns HTMAXBUTTON
/// when the cursor is over our custom maximize button region.
/// This makes Windows 11 show the native snap layout flyout on hover.

use std::sync::atomic::{AtomicI32, AtomicIsize, Ordering};
use tauri::{Runtime, WebviewWindow};

static MAX_BTN_LEFT:   AtomicI32  = AtomicI32::new(0);
static MAX_BTN_TOP:    AtomicI32  = AtomicI32::new(0);
static MAX_BTN_RIGHT:  AtomicI32  = AtomicI32::new(0);
static MAX_BTN_BOTTOM: AtomicI32  = AtomicI32::new(0);

#[cfg(target_os = "windows")]
static ORIG_WNDPROC: AtomicIsize = AtomicIsize::new(0);

/// Called from Tauri command with screen-space pixel coords of the maximize button.
/// The frontend should report logical pixels. We scale them by the window's scale factor.
#[tauri::command]
pub fn set_maximize_button_rect(left: i32, top: i32, right: i32, bottom: i32) {
    MAX_BTN_LEFT.store(left, Ordering::Relaxed);
    MAX_BTN_TOP.store(top, Ordering::Relaxed);
    MAX_BTN_RIGHT.store(right, Ordering::Relaxed);
    MAX_BTN_BOTTOM.store(bottom, Ordering::Relaxed);
}

#[cfg(target_os = "windows")]
unsafe extern "system" fn snap_wnd_proc(
    hwnd: windows_sys::Win32::Foundation::HWND,
    msg: u32,
    wparam: windows_sys::Win32::Foundation::WPARAM,
    lparam: windows_sys::Win32::Foundation::LPARAM,
) -> windows_sys::Win32::Foundation::LRESULT {
    use windows_sys::Win32::UI::WindowsAndMessaging::{
        GWLP_WNDPROC, WM_NCHITTEST, WM_NCDESTROY, HTMAXBUTTON,
        SetWindowLongPtrW,
    };
    use windows_sys::Win32::Graphics::Gdi::ScreenToClient;

    let orig = ORIG_WNDPROC.load(Ordering::Relaxed);

    if msg == WM_NCHITTEST {
        let x = (lparam & 0xFFFF) as i16 as i32;
        let y = ((lparam >> 16) & 0xFFFF) as i16 as i32;

        let mut pt = windows_sys::Win32::Foundation::POINT { x, y };
        ScreenToClient(hwnd, &mut pt);

        // Max button rect from frontend is usually in logical pixels.
        // If we use Physical size, we might need DPI scaling. For now assume coords are correct if scaled right.
        let l = MAX_BTN_LEFT.load(Ordering::Relaxed);
        let t = MAX_BTN_TOP.load(Ordering::Relaxed);
        let r = MAX_BTN_RIGHT.load(Ordering::Relaxed);
        let b = MAX_BTN_BOTTOM.load(Ordering::Relaxed);

        // pt is now in client coords. Compare with our button rect.
        // Actually, if the frontend gave us physical pixels, we just compare directly.
        // Let's compare screen coords directly first since WM_NCHITTEST lparam is screen coords.
        // BUT webview coordinates are relative to the client area!
        // So pt.x/pt.y are relative to the top-left of the webview.
        
        // Let's use the client coordinates for comparison, since getBoundingClientRect() returns client coordinates.
        if l < r && t < b && pt.x >= l && pt.x <= r && pt.y >= t && pt.y <= b {
            return HTMAXBUTTON as isize;
        }
    }

    if msg == WM_NCDESTROY {
        SetWindowLongPtrW(hwnd, GWLP_WNDPROC, orig);
    }

    let orig_fn: unsafe extern "system" fn(
        windows_sys::Win32::Foundation::HWND,
        u32,
        windows_sys::Win32::Foundation::WPARAM,
        windows_sys::Win32::Foundation::LPARAM,
    ) -> windows_sys::Win32::Foundation::LRESULT = std::mem::transmute(orig);
    orig_fn(hwnd, msg, wparam, lparam)
}

/// Subclass the window's WndProc so WM_NCHITTEST can return HTMAXBUTTON.
/// Call once from `.setup()` after the window handle is available.
pub fn install_snap_hook<R: Runtime>(window: &WebviewWindow<R>) {
    #[cfg(target_os = "windows")]
    {
        use raw_window_handle::{HasWindowHandle, RawWindowHandle};
        use windows_sys::Win32::UI::WindowsAndMessaging::{SetWindowLongPtrW, GWLP_WNDPROC};

        let handle = match <WebviewWindow<R> as HasWindowHandle>::window_handle(window) {
            Ok(h) => h,
            Err(_) => return,
        };

        let hwnd = match handle.as_raw() {
            RawWindowHandle::Win32(h) => h.hwnd.get() as windows_sys::Win32::Foundation::HWND,
            _ => return,
        };

        unsafe {
            let orig = SetWindowLongPtrW(hwnd, GWLP_WNDPROC, snap_wnd_proc as *const () as isize);
            ORIG_WNDPROC.store(orig, Ordering::Relaxed);
        }
    }
}
