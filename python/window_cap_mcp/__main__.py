"""
Command-line interface for window-cap-mcp server.

This module provides a CLI for running the window capture MCP server.
"""

import sys
import argparse


def main():
    """Main entry point for the CLI."""
    parser = argparse.ArgumentParser(
        prog="window-cap-mcp",
        description="Cross-platform window and screen screenshot MCP server",
    )
    parser.add_argument(
        "--sse",
        action="store_true",
        help="Use SSE (Server-Sent Events) protocol",
    )
    parser.add_argument(
        "--http",
        action="store_true",
        help="Use Streamable HTTP protocol",
    )
    parser.add_argument(
        "--port",
        type=int,
        default=8080,
        help="Port to listen on (for HTTP/SSE mode) (default: 8080)",
    )
    parser.add_argument(
        "--host",
        type=str,
        default="127.0.0.1",
        help='Host to bind to (for HTTP/SSE mode) (default: "127.0.0.1")',
    )

    args = parser.parse_args()

    # Import run_server here to avoid issues if the module is not yet compiled
    try:
        from window_cap_mcp import run_server
    except ImportError as e:
        print(
            f"Error: Failed to import window_cap_mcp module.\n"
            f"Details: {e}\n"
            "Please ensure the package is properly installed.",
            file=sys.stderr,
        )
        sys.exit(1)

    # Run the server with the specified options
    try:
        run_server(sse=args.sse, http=args.http, port=args.port, host=args.host)
    except KeyboardInterrupt:
        print("\nServer stopped by user", file=sys.stderr)
        sys.exit(130)  # Standard exit code for SIGINT
    except Exception as e:
        print(f"Error running server: {e}", file=sys.stderr)
        sys.exit(1)


if __name__ == "__main__":
    main()
