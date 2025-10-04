use std::fs;
/// Example usage of window_cap_mcp as a Rust library
use window_cap_mcp_lib::{Monitor, Window};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Window Capture MCP Rust Library Example ===\n");

    // Get monitors
    println!("1. Getting monitor information...");
    let monitors = Monitor::all()?;
    println!("Found {} monitors:\n", monitors.len());

    for (index, monitor) in monitors.iter().enumerate() {
        println!("  Monitor {}:", index);
        println!("    Name: {}", monitor.name().unwrap_or_default());
        println!(
            "    Position: ({}, {})",
            monitor.x().unwrap_or(0),
            monitor.y().unwrap_or(0)
        );
        println!(
            "    Size: {}x{}",
            monitor.width().unwrap_or(0),
            monitor.height().unwrap_or(0)
        );
        println!("    Primary: {}", monitor.is_primary().unwrap_or(false));
        println!();
    }

    // Capture screenshot from primary monitor
    println!("2. Capturing screenshot from primary monitor...");
    let primary_monitor = monitors
        .iter()
        .find(|m| m.is_primary().unwrap_or(false))
        .or_else(|| monitors.first())
        .ok_or("No monitor found")?;

    let image = primary_monitor.capture_image()?;
    let mut buffer = Vec::new();
    image.write_to(
        &mut std::io::Cursor::new(&mut buffer),
        image::ImageFormat::Png,
    )?;

    fs::write("screenshot_monitor_rust.png", &buffer)?;
    println!("   Screenshot saved to: screenshot_monitor_rust.png");
    println!("   Size: {} bytes\n", buffer.len());

    // Get windows
    println!("3. Getting window information...");
    let windows = Window::all()?;
    println!("Found {} windows:\n", windows.len());

    let visible_windows: Vec<_> = windows
        .iter()
        .filter(|w| {
            !w.is_minimized().unwrap_or(false)
                && w.width().unwrap_or(0) > 0
                && w.height().unwrap_or(0) > 0
        })
        .collect();

    for (i, window) in visible_windows.iter().enumerate().take(5) {
        println!("  Window {}:", i + 1);
        println!("    ID: {}", window.id().unwrap_or(0));
        println!("    Title: {}", window.title().unwrap_or_default());
        println!("    App: {}", window.app_name().unwrap_or_default());
        println!(
            "    Position: ({}, {})",
            window.x().unwrap_or(0),
            window.y().unwrap_or(0)
        );
        println!(
            "    Size: {}x{}",
            window.width().unwrap_or(0),
            window.height().unwrap_or(0)
        );
        println!(
            "    Minimized: {}, Maximized: {}",
            window.is_minimized().unwrap_or(false),
            window.is_maximized().unwrap_or(false)
        );
        println!();
    }

    // Capture screenshot from first visible window
    if let Some(first_window) = visible_windows.first() {
        println!("4. Capturing screenshot from first visible window...");
        println!(
            "   Target window: {} (ID: {})",
            first_window.title().unwrap_or_default(),
            first_window.id().unwrap_or(0)
        );

        let window_image = first_window.capture_image()?;
        let mut window_buffer = Vec::new();
        window_image.write_to(
            &mut std::io::Cursor::new(&mut window_buffer),
            image::ImageFormat::Png,
        )?;

        fs::write("screenshot_window_rust.png", &window_buffer)?;
        println!("   Screenshot saved to: screenshot_window_rust.png");
        println!("   Size: {} bytes\n", window_buffer.len());
    } else {
        println!("4. No visible windows found to capture\n");
    }

    // Demonstrate base64 encoding (like in Python API)
    println!("5. Base64 encoding example...");
    let base64_image = base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        &buffer[..100.min(buffer.len())], // Just first 100 bytes as example
    );
    println!(
        "   First 100 bytes as base64: {}...\n",
        &base64_image[..50.min(base64_image.len())]
    );

    println!("=== Example completed ===");

    Ok(())
}
