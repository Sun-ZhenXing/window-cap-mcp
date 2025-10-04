"""
Example usage of window_cap_mcp Python library
"""

import window_cap_mcp as wc
import base64
from pathlib import Path


def main():
    print("=== Window Capture MCP Python Example ===\n")

    # Get monitors
    print("1. Getting monitor information...")
    monitors = wc.get_monitors()
    print(f"Found {wc.get_monitor_count()} monitors:\n")

    for monitor in monitors:
        print(f"  Monitor {monitor.index}:")
        print(f"    Name: {monitor.name}")
        print(f"    Position: ({monitor.x}, {monitor.y})")
        print(f"    Size: {monitor.width}x{monitor.height}")
        print(f"    Primary: {monitor.is_primary}")
        print()

    # Capture screenshot from primary monitor
    print("2. Capturing screenshot from primary monitor...")
    try:
        screenshot_base64 = wc.capture_monitor()
        screenshot_data = base64.b64decode(screenshot_base64)

        output_path = Path("screenshot_monitor.png")
        output_path.write_bytes(screenshot_data)
        print(f"   Screenshot saved to: {output_path.absolute()}")
        print(f"   Size: {len(screenshot_data)} bytes\n")
    except Exception as e:
        print(f"   Error: {e}\n")

    # Get windows
    print("3. Getting window information...")
    windows = wc.get_windows()
    print(f"Found {wc.get_window_count()} windows:\n")

    visible_windows = [
        w for w in windows if not w.is_minimized and w.width > 0 and w.height > 0
    ]

    for i, window in enumerate(visible_windows[:5]):  # Show first 5 visible windows
        print(f"  Window {i + 1}:")
        print(f"    ID: {window.id}")
        print(f"    Title: {window.title}")
        print(f"    App: {window.app_name}")
        print(f"    Position: ({window.x}, {window.y})")
        print(f"    Size: {window.width}x{window.height}")
        print(f"    Minimized: {window.is_minimized}, Maximized: {window.is_maximized}")
        print()

    # Capture screenshot from first visible window
    if visible_windows:
        print("4. Capturing screenshot from first visible window...")
        first_window = visible_windows[0]
        print(f"   Target window: {first_window.title} (ID: {first_window.id})")

        try:
            window_screenshot = wc.capture_window(first_window.id)
            window_data = base64.b64decode(window_screenshot)

            output_path = Path("screenshot_window.png")
            output_path.write_bytes(window_data)
            print(f"   Screenshot saved to: {output_path.absolute()}")
            print(f"   Size: {len(window_data)} bytes\n")
        except Exception as e:
            print(f"   Error: {e}\n")
    else:
        print("4. No visible windows found to capture\n")

    # Test monitor to_dict() method
    if monitors:
        print("5. Testing to_dict() method...")
        monitor_dict = monitors[0].to_dict()
        print(f"   Monitor dict: {monitor_dict}\n")

    print("=== Example completed ===")


if __name__ == "__main__":
    main()
