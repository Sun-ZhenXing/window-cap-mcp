"""
Window Capture MCP - Python bindings

A cross-platform window capture library for Python, built with Rust.

Example:
    >>> import window_cap_mcp as wc
    >>> # Get all monitors
    >>> monitors = wc.get_monitors()
    >>> print(f"Found {len(monitors)} monitors")
    >>>
    >>> # Capture screenshot from primary monitor
    >>> screenshot = wc.capture_monitor()
    >>> print(f"Screenshot length: {len(screenshot)}")
    >>>
    >>> # Get all windows
    >>> windows = wc.get_windows()
    >>> for window in windows:
    ...     print(f"Window: {window.title} ({window.app_name})")
    >>>
    >>> # Capture screenshot from specific window
    >>> if windows:
    ...     screenshot = wc.capture_window(windows[0].id)
"""

from .window_cap_mcp import (
    PyMonitor,
    PyWindow,
    get_monitors,
    get_monitor_count,
    capture_monitor,
    get_windows,
    get_window_count,
    capture_window,
    run_server,
)

__version__ = "0.2.0"

__all__ = [
    "PyMonitor",
    "PyWindow",
    "get_monitors",
    "get_monitor_count",
    "capture_monitor",
    "get_windows",
    "get_window_count",
    "capture_window",
    "run_server",
]
