# Window Capture MCP Server

A cross-platform window screenshot MCP server and library implemented in Rust, based on [rmcp](https://crates.io/crates/rmcp) and [xcap](https://crates.io/crates/xcap).

## Features

- Get monitor count and details
- Capture screenshots of monitors and windows
- List and close windows
- Cross-platform support (Windows, macOS, Linux)
- Supports multiple transport modes (STDIO, SSE, HTTP)
- Available as Rust library and Python package

## Installation

### As MCP Server

```bash
# Build from source
make build
make run
```

### As Python Package

```bash
pip install window-cap-mcp
```

### As Rust Library

```toml
[dependencies]
window_cap_mcp_lib = "0.1"
```

## Usage

### MCP Server

```bash
# STDIO mode (for Claude Desktop)
window-cap-mcp

# SSE mode
window-cap-mcp --sse --port 3000

# HTTP mode
window-cap-mcp --http --port 8080
```

### Python Library

```python
import window_cap_mcp as wc

# Capture monitor screenshot
screenshot = wc.capture_monitor()

# Run as MCP server
wc.run_server()
```

### Rust Library

```rust
use window_cap_mcp_lib::{Monitor, Window};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let monitors = Monitor::all()?;
    monitors[0].capture_image()?.save("screenshot.png")?;
    Ok(())
}
```

## Claude Desktop Configuration

Edit the configuration file:

- **Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
- **macOS/Linux**: `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "window-cap": {
      "command": "window-cap-mcp"
    }
  }
}
```

## MCP Tools

### get_monitor_count

Get monitor information.

```json
{
  "count": 2,
  "monitors": [
    {
      "index": 0,
      "name": "Display 1",
      "width": 1920,
      "height": 1080,
      "is_primary": true
    }
  ]
}
```

### get_screen_screenshot

Capture monitor screenshot (Base64-encoded PNG).

**Parameters**: `monitor_index` (optional)

### get_window_list

Get all windows information.

```json
{
  "count": 5,
  "windows": [
    {
      "id": 12345,
      "title": "VS Code",
      "app_name": "Code.exe",
      "width": 1280,
      "height": 720
    }
  ]
}
```

### get_window_screenshot

Capture window screenshot (Base64-encoded PNG).

**Parameters**: `window_id` (required)

### close_window

Close a window by ID.

**Parameters**: `window_id` (required)

## Development

```bash
make help          # View commands
make build         # Build project
make test          # Run tests
make clean         # Clean artifacts
```

## License

[MIT](./LICENSE)
