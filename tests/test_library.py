"""
Quick test script for window_cap_mcp Python library
"""


def test_imports():
    """Test that all imports work"""
    import window_cap_mcp as wc

    # Check all expected functions exist
    assert hasattr(wc, "get_monitors")
    assert hasattr(wc, "get_monitor_count")
    assert hasattr(wc, "capture_monitor")
    assert hasattr(wc, "get_windows")
    assert hasattr(wc, "get_window_count")
    assert hasattr(wc, "capture_window")
    assert hasattr(wc, "PyMonitor")
    assert hasattr(wc, "PyWindow")


def test_monitors():
    """Test monitor functions"""
    import window_cap_mcp as wc

    # Test get_monitor_count
    count = wc.get_monitor_count()
    assert count > 0, "Should have at least one monitor"

    # Test get_monitors
    monitors = wc.get_monitors()
    assert len(monitors) == count

    # Test monitor properties
    if monitors:
        m = monitors[0]
        assert hasattr(m, "index")
        assert hasattr(m, "name")
        assert hasattr(m, "width")
        assert hasattr(m, "height")
        assert hasattr(m, "is_primary")

        # Test to_dict
        d = m.to_dict()
        assert isinstance(d, dict)


def test_windows():
    """Test window functions"""
    import window_cap_mcp as wc

    # Test get_window_count
    count = wc.get_window_count()

    # Test get_windows
    windows = wc.get_windows()
    assert len(windows) == count

    # Test window properties
    if windows:
        w = windows[0]
        assert hasattr(w, "id")
        assert hasattr(w, "title")
        assert hasattr(w, "app_name")
        assert hasattr(w, "width")
        assert hasattr(w, "height")

        # Test to_dict
        d = w.to_dict()
        assert isinstance(d, dict)


def test_capture():
    """Test capture functions"""
    import window_cap_mcp as wc
    import base64

    # Test monitor capture
    screenshot = wc.capture_monitor(0)
    assert isinstance(screenshot, str)
    assert len(screenshot) > 0

    # Verify it's valid base64
    data = base64.b64decode(screenshot)
    assert len(data) > 0
    # Check PNG header
    assert data[:8] == b"\x89PNG\r\n\x1a\n"

    # Test window capture (if windows available)
    windows = wc.get_windows()
    visible = [w for w in windows if not w.is_minimized and w.width > 0]

    if visible:
        window_screenshot = wc.capture_window(visible[0].id)
        assert isinstance(window_screenshot, str)
        assert len(window_screenshot) > 0

        window_data = base64.b64decode(window_screenshot)
        assert len(window_data) > 0
        assert window_data[:8] == b"\x89PNG\r\n\x1a\n"
