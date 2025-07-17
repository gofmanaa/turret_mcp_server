mod turret_mcp_server;

use crate::turret_mcp_server::Turret;
use anyhow::Result;
use rmcp::{
    ServiceExt,
    transport::sse_server::{SseServer},
    transport::stdio,
};
use std::env;
use std::net::SocketAddr;
use tracing_subscriber::{self, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP server");

    if env::var("INDOCKER").unwrap_or_default() == "true" {
        let addr: SocketAddr = "0.0.0.0:8080".parse()?;
        tracing::info!("transport: sse");
        tracing::info!("Listening on http://{}", addr);
        let sse_server = SseServer::serve(addr)
            .await?
            .with_service_directly(Turret::new);

        tokio::signal::ctrl_c().await?;
        sse_server.cancel();
    } else {
        tracing::info!("transport: stdio");
        let service = Turret::new().serve(stdio()).await.inspect_err(|e| {
            tracing::error!("serving error: {:?}", e);
        })?;

        service.waiting().await?;
    }

    Ok(())
}
