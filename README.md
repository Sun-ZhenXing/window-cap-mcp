# Window Capture MCP Server

A cross-platform window screenshot MCP server and library implemented in Rust, based on [rmcp](https://crates.io/crates/rmcp) and [xcap](https://crates.io/crates/xcap).

## Features

- ✅ Get monitor count and details
- ✅ Capture screenshots of specified monitors
- ✅ Get list of all windows
- ✅ Capture screenshots of specified windows
- ✅ Close windows by ID
- ✅ Cross-platform support (Windows, macOS, Linux)
- ✅ Base64-encoded PNG format screenshots
- ✅ Support for multiple transport modes (STDIO, SSE, HTTP Streamable)
- ✨ **Provides both Rust library and Python library**

## What's New in v0.2.0

🎉 **Python Native Server API**: You can now run the MCP server directly from Python without needing a separate Rust binary!

```python
import window_cap_mcp as wc

# Run server in Python
wc.run_server()                          # STDIO mode
wc.run_server(sse=True, port=8080)      # SSE mode
wc.run_server(http=True, port=3000)     # HTTP mode
```

Benefits:

- ✅ Install once with pip - no additional compilation needed
- ✅ Works out of the box on all platforms
- ✅ Easy to integrate into Python scripts
- ✅ Same performance as the Rust binary

## Usage

This project offers three usage modes:

1. **MCP Server**: Run as an MCP server, can be integrated with clients like Claude Desktop
2. **Rust Library**: Use as a Rust crate in other Rust projects
3. **Python Library**: Use in Python projects through Python bindings

## Quick Start

### Requirements

- [Rust](https://rustup.rs/) - Rust toolchain
- [uv](https://docs.astral.sh/uv/) - Python package manager (for Python library development)
- `make` - Build tool (Windows users can install via [Chocolatey](https://chocolatey.org/): `choco install make`)

### Building the Project

```bash
# View all available commands
make help

# Build all components
make

# Run MCP server
make run
```

## Usage Instructions

### 1. MCP Server Mode

```bash
# Build and run
make run

# Or run manually
./target/release/window-cap-mcp          # STDIO mode (for Claude Desktop)
./target/release/window-cap-mcp --sse    # SSE HTTP server mode
./target/release/window-cap-mcp --http   # HTTP Streamable mode
```

### 2. Rust Library Mode

```rust
use window_cap_mcp_lib::{Monitor, Window};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitors = Monitor::all()?;
    monitors[0].capture_image()?.save("screenshot.png")?;
    Ok(())
}
```

Run example: `make run-example`

### 3. Python Library Mode

```bash
# Build and install
make install-python
```

```python
import window_cap_mcp as wc
import base64

screenshot = wc.capture_monitor()
data = base64.b64decode(screenshot)
with open("screenshot.png", "wb") as f:
    f.write(data)
```

Run example: `uv run python examples/python_example.py`

#### Using Python CLI

After installing the Python package, you can use the `window-cap-mcp` command directly:

```bash
# Run in STDIO mode (default, for Claude Desktop integration)
window-cap-mcp

# Run in SSE mode on custom host/port
window-cap-mcp --sse --host 0.0.0.0 --port 3000

# Run in HTTP Streamable mode
window-cap-mcp --http --port 8080

# View help
window-cap-mcp --help
```

Or use it as a Python module:

```bash
python -m window_cap_mcp --help
```

**Claude Desktop Configuration for Python Package**:

After installing via `pip install window-cap-mcp`, update your Claude Desktop config:

```json
{
  "mcpServers": {
    "window-cap": {
      "command": "window-cap-mcp"
    }
  }
}
```

Or specify the full Python path:

```json
{
  "mcpServers": {
    "window-cap": {
      "command": "python",
      "args": ["-m", "window_cap_mcp"]
    }
  }
}
```

## MCP Tool Functions

### 1. get_monitor_count

Get monitor count and details.

**Parameters**: None

**Returns**:

```json
{
  "count": 2,
  "monitors": [
    {
      "index": 0,
      "name": "Display 1",
      "x": 0,
      "y": 0,
      "width": 1920,
      "height": 1080,
      "is_primary": true
    },
    {
      "index": 1,
      "name": "Display 2",
      "x": 1920,
      "y": 0,
      "width": 1920,
      "height": 1080,
      "is_primary": false
    }
  ]
}
```

### 2. get_screen_screenshot

Capture a screenshot of a specified monitor (Base64-encoded PNG format).

**Parameters**:

- `monitor_index` (optional): Monitor index, defaults to primary monitor if not specified

**Returns**:

```json
{
  "monitor_index": 0,
  "monitor_name": "Display 1",
  "width": 1920,
  "height": 1080,
  "image_base64": "iVBORw0KGgoAAAANSUhEUgA...",
  "format": "png"
}
```

### 3. get_window_list

Get a list of all windows.

**Parameters**: None

**Returns**:

```json
{
  "count": 5,
  "windows": [
    {
      "id": 12345,
      "title": "VS Code",
      "app_name": "Code.exe",
      "x": 100,
      "y": 100,
      "width": 1280,
      "height": 720,
      "is_minimized": false,
      "is_maximized": false
    }
  ]
}
```

### 4. get_window_screenshot

Capture a screenshot of a specified window (Base64-encoded PNG format).

**Parameters**:

- `window_id` (required): Window ID

**Returns**:

```json
{
  "window_id": 12345,
  "title": "VS Code",
  "app_name": "Code.exe",
  "width": 1280,
  "height": 720,
  "image_base64": "iVBORw0KGgoAAAANSUhEUgA...",
  "format": "png"
}
```

### 5. close_window

Close a window by its ID. This tool sends a close request to the specified window.

**Parameters**:

- `window_id` (required): Window ID to close

**Returns**:

```json
{
  "success": true,
  "message": "Successfully closed window: VS Code [Code.exe] (ID: 12345)"
}
```

**Platform Support**:

- ✅ Windows: Uses `WM_CLOSE` message
- ✅ macOS: Uses Cocoa NSWindow close method
- ✅ Linux: Uses X11 `WM_DELETE_WINDOW` protocol

**Note**: This sends a polite close request to the window. The application can choose to ignore it (e.g., if there are unsaved changes).

## Claude Desktop Configuration

Edit the configuration file and add the following:

- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
- **macOS/Linux**: `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "window-cap": {
      "command": "<PATH_TO_BINARY>"
    }
  }
}
```

## Development Commands

```bash
make help          # View all commands
make build         # Build all components
make test          # Run tests
make fmt           # Format code
make check         # Check code
make clean         # Clean build artifacts
```

## License

[MIT](./LICENSE)
