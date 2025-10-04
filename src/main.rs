use window_cap_mcp_lib::server;

#[tokio::main]
async fn main() {
    if let Err(e) = server::run_server().await {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
