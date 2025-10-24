use xcap::Window;

#[cfg(target_os = "windows")]
use windows::Win32::Foundation::HWND;
#[cfg(target_os = "windows")]
use windows::Win32::UI::WindowsAndMessaging::{PostMessageW, WM_CLOSE};

#[cfg(target_os = "linux")]
use std::ptr;
#[cfg(target_os = "linux")]
use x11::xlib;

/// Close a window by its ID with validation and informative message
///
/// This function validates the window exists, closes it, and returns a success message
/// with the window's title and app name.
///
/// # Arguments
/// * `window_id` - The ID of the window to close
///
/// # Returns
/// * `Ok(String)` - Success message with window details
/// * `Err(String)` - Error message if window not found or closing failed
pub fn close_window_with_info(window_id: u32) -> Result<String, String> {
    // Verify window exists
    let windows = Window::all().map_err(|e| format!("Failed to get window list: {}", e))?;

    let window = windows
        .iter()
        .find(|w| w.id().unwrap_or(0) == window_id)
        .ok_or_else(|| format!("Window ID {} does not exist", window_id))?;

    let window_title = window.title().unwrap_or_default();
    let window_app = window.app_name().unwrap_or_default();

    // Call platform-specific close function
    close_window_by_id(window_id)?;

    Ok(format!(
        "Successfully closed window: {} [{}] (ID: {})",
        window_title, window_app, window_id
    ))
}

/// Close a window by its ID (platform-specific implementation)
///
/// # Arguments
/// * `window_id` - The ID of the window to close
///
/// # Returns
/// * `Ok(())` - Window close request sent successfully
/// * `Err(String)` - Error message if closing failed
///
/// # Platform-specific behavior
/// - Windows: Sends WM_CLOSE message
/// - macOS: Calls NSWindow close method
/// - Linux: Sends X11 WM_DELETE_WINDOW protocol message
///
/// # Safety
/// This function uses unsafe code to interact with platform-specific APIs.
/// The window ID must be valid for the current platform.
#[cfg(target_os = "windows")]
pub fn close_window_by_id(window_id: u32) -> Result<(), String> {
    unsafe {
        let hwnd = HWND(window_id as isize as *mut std::ffi::c_void);
        PostMessageW(
            hwnd,
            WM_CLOSE,
            windows::Win32::Foundation::WPARAM(0),
            windows::Win32::Foundation::LPARAM(0),
        )
        .map_err(|e| format!("Failed to close window: {}", e))?;
    }
    Ok(())
}

#[cfg(target_os = "macos")]
pub fn close_window_by_id(window_id: u32) -> Result<(), String> {
    use objc2::msg_send;
    use objc2::runtime::AnyObject;

    unsafe {
        let window_ptr = window_id as usize as *mut AnyObject;
        if window_ptr.is_null() {
            return Err("Invalid window ID".to_string());
        }

        let window = window_ptr
            .as_ref()
            .ok_or_else(|| "Invalid window pointer".to_string())?;

        let _: () = msg_send![window, close];
    }
    Ok(())
}

#[cfg(target_os = "linux")]
pub fn close_window_by_id(window_id: u32) -> Result<(), String> {
    unsafe {
        let display = xlib::XOpenDisplay(ptr::null());
        if display.is_null() {
            return Err("Failed to open X11 display".to_string());
        }

        let window = window_id as xlib::Window;

        // Send WM_DELETE_WINDOW message
        let wm_protocols = xlib::XInternAtom(
            display,
            c"WM_PROTOCOLS".as_ptr(),
            xlib::False,
        );
        let wm_delete_window = xlib::XInternAtom(
            display,
            c"WM_DELETE_WINDOW".as_ptr(),
            xlib::False,
        );

        let mut event: xlib::XClientMessageEvent = std::mem::zeroed();
        event.type_ = xlib::ClientMessage;
        event.window = window;
        event.message_type = wm_protocols;
        event.format = 32;
        event.data.as_longs_mut()[0] = wm_delete_window as i64;
        event.data.as_longs_mut()[1] = xlib::CurrentTime as i64;

        let result = xlib::XSendEvent(
            display,
            window,
            xlib::False,
            xlib::NoEventMask,
            &mut event as *mut xlib::XClientMessageEvent as *mut xlib::XEvent,
        );

        xlib::XFlush(display);
        xlib::XCloseDisplay(display);

        if result == 0 {
            return Err("Failed to send close event".to_string());
        }
    }
    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
pub fn close_window_by_id(_window_id: u32) -> Result<(), String> {
    Err("Window closing is not supported on this platform".to_string())
}
