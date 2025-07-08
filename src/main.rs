mod counter;

use std::sync::Arc;
use anyhow::Result;

use rmcp::{ServiceExt, transport::stdio, tool_router, tool, model::*, Error as McpError};
use rmcp::handler::server::tool::ToolRouter;
use tokio::sync::Mutex;
use tracing_subscriber::{self, EnvFilter};
use crate::counter::Counter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP server");

    // Create an instance of our counter router
    let service = Counter::new().serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}