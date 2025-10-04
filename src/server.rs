use crate::cli::Cli;
use crate::handler::WindowCapServer;
use clap::Parser;
use rmcp::{
    transport::{
        sse_server::SseServer,
        stdio,
        streamable_http_server::{
            session::local::LocalSessionManager,
            tower::{StreamableHttpServerConfig, StreamableHttpService},
        },
    },
    ServiceExt,
};
use std::net::SocketAddr;
use std::sync::Arc;

pub async fn run_server() -> anyhow::Result<()> {
    let cli = Cli::parse();

    if cli.sse {
        eprintln!("Starting server in SSE mode...");
        let addr: SocketAddr = format!("{}:{}", cli.host, cli.port).parse()?;
        eprintln!("Binding to: {}", addr);

        let ct = SseServer::serve(addr)
            .await?
            .with_service(WindowCapServer::new);

        eprintln!("SSE server started, visit http://{}", addr);
        tokio::signal::ctrl_c().await?;
        ct.cancel();
        eprintln!("Server stopped");
    } else if cli.http {
        eprintln!("Starting server in HTTP Streamable mode...");
        let addr: SocketAddr = format!("{}:{}", cli.host, cli.port).parse()?;
        eprintln!("Binding to: {}", addr);

        let session_manager = Arc::new(LocalSessionManager::default());
        let service_factory = || Ok(WindowCapServer::new());
        let config = StreamableHttpServerConfig::default();
        let http_service = StreamableHttpService::new(service_factory, session_manager, config);
        let listener = tokio::net::TcpListener::bind(addr).await?;
        eprintln!("HTTP Streamable server started at http://{}", addr);

        loop {
            let (stream, peer_addr) = listener.accept().await?;
            eprintln!("Accepted connection from: {}", peer_addr);
            let http_service = http_service.clone();
            tokio::spawn(async move {
                use hyper_util::rt::TokioIo;
                use hyper_util::server::conn::auto::Builder;
                use tower::Service;
                let io = TokioIo::new(stream);
                let service = http_service.clone();
                let hyper_service = hyper::service::service_fn(move |req| {
                    let mut svc = service.clone();
                    async move { svc.call(req).await }
                });
                if let Err(e) = Builder::new(hyper_util::rt::TokioExecutor::new())
                    .serve_connection(io, hyper_service)
                    .await
                {
                    eprintln!("Error serving connection: {}", e);
                }
            });
        }
    } else {
        eprintln!("Starting server in STDIO mode...");
        let server = WindowCapServer::new();
        let service = server.serve(stdio()).await?;
        service.waiting().await?;
    }
    Ok(())
}
