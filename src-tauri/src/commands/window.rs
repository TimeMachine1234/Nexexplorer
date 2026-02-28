use tauri::Window;

#[tauri::command]
pub async fn toggle_fullscreen(window: Window) -> Result<bool, String> {
    let is_fullscreen = window.is_fullscreen().map_err(|e| e.to_string())?;
    
    if is_fullscreen {
        window.set_fullscreen(false).map_err(|e| e.to_string())?;
        Ok(false)
    } else {
        window.set_fullscreen(true).map_err(|e| e.to_string())?;
        Ok(true)
    }
}

#[tauri::command]
pub async fn snap_left(window: Window) -> Result<(), String> {
    // Get window monitor info
    let monitor = window.current_monitor().map_err(|e| e.to_string())?
        .ok_or("No monitor found")?;
    
    let monitor_size = monitor.size();
    let monitor_position = monitor.position();
    
    // Calculate left half of screen with Windows taskbar consideration
    let taskbar_height = 40; // Approximate taskbar height
    let window_width = monitor_size.width / 2;
    let window_height = monitor_size.height - taskbar_height;
    
    window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
        x: monitor_position.x,
        y: monitor_position.y,
    })).map_err(|e| e.to_string())?;
    
    window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
        width: window_width,
        height: window_height,
    })).map_err(|e| e.to_string())?;
    
    window.unmaximize().map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn snap_right(window: Window) -> Result<(), String> {
    // Get window monitor info
    let monitor = window.current_monitor().map_err(|e| e.to_string())?
        .ok_or("No monitor found")?;
    
    let monitor_size = monitor.size();
    let monitor_position = monitor.position();
    
    // Calculate right half of screen with Windows taskbar consideration
    let taskbar_height = 40; // Approximate taskbar height
    let window_width = monitor_size.width / 2;
    let window_height = monitor_size.height - taskbar_height;
    
    window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
        x: monitor_position.x + window_width as i32,
        y: monitor_position.y,
    })).map_err(|e| e.to_string())?;
    
    window.set_size(tauri::Size::Physical(tauri::PhysicalSize {
        width: window_width,
        height: window_height,
    })).map_err(|e| e.to_string())?;
    
    window.unmaximize().map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
pub async fn is_fullscreen(window: Window) -> Result<bool, String> {
    window.is_fullscreen().map_err(|e| e.to_string())
}

// quadrant: 0=top-left, 1=top-right, 2=bottom-left, 3=bottom-right
#[tauri::command]
pub async fn snap_quarter(window: Window, quadrant: u8) -> Result<(), String> {
    let monitor = window.current_monitor().map_err(|e| e.to_string())?
        .ok_or("No monitor found")?;

    let monitor_size = monitor.size();
    let monitor_pos = monitor.position();

    let half_w = monitor_size.width / 2;
    let half_h = (monitor_size.height as i32 - 40).max(200) as u32; // subtract taskbar
    let half_h_half = half_h / 2;

    let (x, y, w, h) = match quadrant {
        0 => (monitor_pos.x,              monitor_pos.y,              half_w, half_h_half), // top-left
        1 => (monitor_pos.x + half_w as i32, monitor_pos.y,           half_w, half_h_half), // top-right
        2 => (monitor_pos.x,              monitor_pos.y + half_h_half as i32, half_w, half_h_half), // bottom-left
        3 => (monitor_pos.x + half_w as i32, monitor_pos.y + half_h_half as i32, half_w, half_h_half), // bottom-right
        _ => return Err("Invalid quadrant".to_string()),
    };

    window.unmaximize().map_err(|e| e.to_string())?;
    window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }))
        .map_err(|e| e.to_string())?;
    window.set_size(tauri::Size::Physical(tauri::PhysicalSize { width: w, height: h }))
        .map_err(|e| e.to_string())?;

    Ok(())
}
