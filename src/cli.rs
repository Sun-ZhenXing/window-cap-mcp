use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "window-cap-mcp")]
#[command(about = "Cross-platform window and screen screenshot MCP server", long_about = None)]
pub struct Cli {
    /// Use SSE (Server-Sent Events) protocol
    #[arg(long)]
    pub sse: bool,

    /// Use Streamable HTTP protocol
    #[arg(long)]
    pub http: bool,

    /// Port to listen on (for HTTP/SSE mode)
    #[arg(long, default_value = "8080")]
    pub port: u16,

    /// Host to bind to (for HTTP/SSE mode)
    #[arg(long, default_value = "127.0.0.1")]
    pub host: String,
}
