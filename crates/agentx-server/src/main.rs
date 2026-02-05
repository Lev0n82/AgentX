use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use tracing::info;
use dotenv::dotenv;

mod api;
mod websocket;
mod kasm;
mod state;
mod error;

use crate::kasm::{KasmClient, KasmConfig};
use crate::state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Load environment variables
    dotenv().ok();

    info!("Starting AgentX Web Server v{}", env!("CARGO_PKG_VERSION"));

    // Initialize Kasm client
    let kasm_config = KasmConfig::from_env()?;
    info!("Kasm API URL: {}", kasm_config.base_url);
    
    let kasm_client = KasmClient::new(kasm_config);

    // Check Kasm health (non-blocking)
    tokio::spawn({
        let client = kasm_client.clone();
        async move {
            match client.health_check().await {
                Ok(true) => info!("✅ Kasm Workspaces is healthy"),
                Ok(false) => info!("⚠️  Kasm Workspaces health check failed"),
                Err(e) => info!("⚠️  Could not reach Kasm Workspaces: {}", e),
            }
        }
    });

    // Initialize shared application state
    let app_state = AppState::new(kasm_client).await;

    // Build application router
    let app = Router::new()
        // Health check
        .route("/api/v1/health", get(api::health_check))
        
        // Task management
        .route("/api/v1/tasks", post(api::tasks::create_task))
        .route("/api/v1/tasks/:id", get(api::tasks::get_task_status))
        .route("/api/v1/tasks/:id/cancel", post(api::tasks::cancel_task))
        
        // WebSocket endpoint
        .route("/ws", get(websocket::ws_handler))
        
        // Add state
        .with_state(app_state)
        
        // Add middleware
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("🚀 AgentX Web Server listening on {}", addr);
    info!("📡 WebSocket endpoint: ws://localhost:8080/ws");
    info!("🏥 Health check: http://localhost:8080/api/v1/health");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
