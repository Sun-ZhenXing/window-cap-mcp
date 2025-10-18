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
    assert hasattr(wc, "close_window")
    assert hasattr(wc, "run_server")
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
        assert hasattr(m, "x")
        assert hasattr(m, "y")

        # Test property values
        assert isinstance(m.index, int)
        assert isinstance(m.name, str)
        assert isinstance(m.width, int)
        assert isinstance(m.height, int)
        assert isinstance(m.is_primary, bool)
        assert m.width > 0
        assert m.height > 0

        # Test to_dict
        d = m.to_dict()
        assert isinstance(d, dict)
        assert "index" in d
        assert "name" in d
        assert "width" in d
        assert "height" in d
        assert "is_primary" in d
        assert "x" in d
        assert "y" in d

        # Test __repr__
        repr_str = repr(m)
        assert isinstance(repr_str, str)
        assert "PyMonitor" in repr_str


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
        assert hasattr(w, "x")
        assert hasattr(w, "y")
        assert hasattr(w, "is_minimized")
        assert hasattr(w, "is_maximized")

        # Test property types
        assert isinstance(w.id, int)
        assert isinstance(w.title, str)
        assert isinstance(w.app_name, str)
        assert isinstance(w.width, int)
        assert isinstance(w.height, int)
        assert isinstance(w.x, int)
        assert isinstance(w.y, int)
        assert isinstance(w.is_minimized, bool)
        assert isinstance(w.is_maximized, bool)

        # Test to_dict
        d = w.to_dict()
        assert isinstance(d, dict)
        assert "id" in d
        assert "title" in d
        assert "app_name" in d
        assert "width" in d
        assert "height" in d
        assert "x" in d
        assert "y" in d
        assert "is_minimized" in d
        assert "is_maximized" in d

        # Test __repr__
        repr_str = repr(w)
        assert isinstance(repr_str, str)
        assert "PyWindow" in repr_str


def test_capture():
    """Test capture functions"""
    import window_cap_mcp as wc
    import base64

    # Test monitor capture with specific index
    screenshot = wc.capture_monitor(0)
    assert isinstance(screenshot, str)
    assert len(screenshot) > 0

    # Verify it's valid base64
    data = base64.b64decode(screenshot)
    assert len(data) > 0
    # Check PNG header
    assert data[:8] == b"\x89PNG\r\n\x1a\n"

    # Test monitor capture with None (primary monitor)
    screenshot_primary = wc.capture_monitor(None)
    assert isinstance(screenshot_primary, str)
    assert len(screenshot_primary) > 0

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


def test_close_window_errors():
    """Test close_window error handling"""
    import window_cap_mcp as wc
    import pytest

    # Test with invalid window ID
    fake_window_id = 999999999
    with pytest.raises(ValueError) as exc_info:
        wc.close_window(fake_window_id)

    assert "does not exist" in str(exc_info.value) or "not found" in str(exc_info.value)


def test_close_window_validation():
    """Test close_window with real window (non-destructive)"""
    import window_cap_mcp as wc

    # Get windows
    windows = wc.get_windows()

    if not windows:
        # Skip test if no windows available
        return

    # Verify the function exists and is callable
    # Note: We don't actually close windows in automated tests
    # as that could be destructive to the test environment
    assert callable(wc.close_window)

    # In a real-world scenario, you would:
    # 1. Create/open a test application
    # 2. Get its window ID
    # 3. Call close_window(window_id)
    # 4. Verify the window is closed
    # But that's too complex for a unit test


def test_capture_errors():
    """Test capture function error handling"""
    import window_cap_mcp as wc
    import pytest

    # Test capture_window with invalid ID
    with pytest.raises(ValueError) as exc_info:
        wc.capture_window(999999999)

    assert "not found" in str(exc_info.value)


def test_monitor_capture_with_index():
    """Test monitor capture with different index values"""
    import window_cap_mcp as wc
    import base64

    monitor_count = wc.get_monitor_count()

    # Test valid indices
    for i in range(monitor_count):
        screenshot = wc.capture_monitor(i)
        assert isinstance(screenshot, str)
        assert len(screenshot) > 0

        data = base64.b64decode(screenshot)
        assert data[:8] == b"\x89PNG\r\n\x1a\n"
