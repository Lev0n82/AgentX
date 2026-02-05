use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use tracing::{debug, error, info};
use crate::state::AppState;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    info!("WebSocket connection request received");
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.broadcaster.subscribe();

    info!("WebSocket client connected");

    // Send initial connection message
    let welcome_msg = serde_json::json!({
        "type": "connected",
        "message": "Connected to AgentX server"
    });
    
    if sender.send(Message::Text(welcome_msg.to_string())).await.is_err() {
        error!("Failed to send welcome message");
        return;
    }

    // Spawn task to receive broadcasts and send to client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let json = match serde_json::to_string(&msg) {
                Ok(j) => j,
                Err(e) => {
                    error!("Failed to serialize message: {}", e);
                    continue;
                }
            };
            
            debug!("Sending to WebSocket client: {}", json);
            
            if sender.send(Message::Text(json)).await.is_err() {
                error!("Client disconnected, stopping send task");
                break;
            }
        }
    });

    // Spawn task to receive messages from client (for ping/pong, commands, etc.)
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Text(text) => {
                    debug!("Received from client: {}", text);
                    // Handle client commands here if needed
                }
                Message::Ping(_) => {
                    debug!("Received ping from client");
                }
                Message::Pong(_) => {
                    debug!("Received pong from client");
                }
                Message::Close(_) => {
                    info!("Client sent close message");
                    break;
                }
                _ => {}
            }
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => {
            debug!("Send task completed");
            recv_task.abort();
        }
        _ = (&mut recv_task) => {
            debug!("Receive task completed");
            send_task.abort();
        }
    };

    info!("WebSocket client disconnected");
}
