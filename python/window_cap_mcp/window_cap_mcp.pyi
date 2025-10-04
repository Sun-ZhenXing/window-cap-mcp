"""Type stubs for window_cap_mcp"""

from typing import List, Optional

class PyMonitor:
    """Represents a monitor/screen"""

    index: int
    name: str
    x: int
    y: int
    width: int
    height: int
    is_primary: bool

    def __repr__(self) -> str: ...
    def to_dict(self) -> dict: ...

class PyWindow:
    """Represents a window"""

    id: int
    title: str
    app_name: str
    x: int
    y: int
    width: int
    height: int
    is_minimized: bool
    is_maximized: bool

    def __repr__(self) -> str: ...
    def to_dict(self) -> dict: ...

def get_monitors() -> List[PyMonitor]:
    """Get list of all monitors"""
    ...

def get_monitor_count() -> int:
    """Get count of monitors"""
    ...

def capture_monitor(monitor_index: Optional[int] = None) -> str:
    """
    Capture screenshot from monitor

    Args:
        monitor_index: Optional monitor index. If None, captures from primary monitor.

    Returns:
        Base64 encoded PNG image
    """
    ...

def get_windows() -> List[PyWindow]:
    """Get list of all windows"""
    ...

def get_window_count() -> int:
    """Get count of windows"""
    ...

def capture_window(window_id: int) -> str:
    """
    Capture screenshot from window

    Args:
        window_id: Window ID

    Returns:
        Base64 encoded PNG image
    """
    ...

def run_server(
    sse: bool = False,
    http: bool = False,
    port: int = 8080,
    host: str = "127.0.0.1",
) -> None:
    """
    Run the MCP server

    Args:
        sse: Use SSE (Server-Sent Events) protocol
        http: Use Streamable HTTP protocol
        port: Port to listen on (for HTTP/SSE mode)
        host: Host to bind to (for HTTP/SSE mode)

    Examples:
        >>> import window_cap_mcp as wc
        >>> # Run in STDIO mode (default)
        >>> wc.run_server()
        >>>
        >>> # Run in SSE mode
        >>> wc.run_server(sse=True, port=8080)
        >>>
        >>> # Run in HTTP mode
        >>> wc.run_server(http=True, port=3000, host="0.0.0.0")
    """
    ...
