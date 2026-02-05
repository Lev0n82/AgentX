use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::{info, error};
use chrono::Utc;
use crate::error::{Result, ServerError};
use crate::state::{AppState, TaskStatus};
use crate::websocket::BroadcastMessage;

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub idea: String,
    #[serde(default = "default_agent")]
    pub agent: String,
    #[serde(default = "default_model")]
    pub model: String,
    #[serde(default = "default_n_round")]
    pub n_round: i32,
}

fn default_agent() -> String {
    "MetaGPT".to_string()
}

fn default_model() -> String {
    "qwen2.5-coder:3b".to_string()
}

fn default_n_round() -> i32 {
    4
}

#[derive(Debug, Serialize)]
pub struct CreateTaskResponse {
    pub task_id: String,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TaskStatusResponse {
    pub id: String,
    pub idea: String,
    pub status: String,
    pub created_at: String,
    pub workspaces: Vec<String>,
}

pub async fn create_task(
    State(state): State<AppState>,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<Json<CreateTaskResponse>> {
    info!("Creating new task: {}", payload.idea);

    // Create task
    let task = state.create_task(
        payload.idea.clone(),
        payload.agent,
        payload.model,
        payload.n_round,
    ).await;

    // Broadcast task creation event
    state.broadcaster.broadcast(BroadcastMessage::AgentEvent {
        timestamp: Utc::now(),
        agent: "system".to_string(),
        status: "pending".to_string(),
        action: Some("create_task".to_string()),
        message: format!("Task created: {}", payload.idea),
    });

    // Spawn task executor in background
    let task_id = task.id.clone();
    let state_clone = state.clone();
    tokio::spawn(async move {
        if let Err(e) = execute_task(task_id.clone(), state_clone).await {
            error!("Task {} failed: {}", task_id, e);
        }
    });

    Ok(Json(CreateTaskResponse {
        task_id: task.id,
        status: "pending".to_string(),
        message: "Task created and queued for execution".to_string(),
    }))
}

pub async fn get_task_status(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Json<TaskStatusResponse>> {
    info!("Getting status for task: {}", task_id);

    let task = state.get_task(&task_id).await
        .ok_or_else(|| ServerError::TaskNotFound(task_id.clone()))?;

    Ok(Json(TaskStatusResponse {
        id: task.id,
        idea: task.idea,
        status: format!("{:?}", task.status).to_lowercase(),
        created_at: task.created_at.to_rfc3339(),
        workspaces: task.workspaces,
    }))
}

pub async fn cancel_task(
    State(state): State<AppState>,
    Path(task_id): Path<String>,
) -> Result<Json<serde_json::Value>> {
    info!("Cancelling task: {}", task_id);

    state.update_task_status(&task_id, TaskStatus::Failed).await;

    // Broadcast cancellation
    state.broadcaster.broadcast(BroadcastMessage::AgentEvent {
        timestamp: Utc::now(),
        agent: "system".to_string(),
        status: "cancelled".to_string(),
        action: Some("cancel_task".to_string()),
        message: format!("Task {} cancelled", task_id),
    });

    Ok(Json(serde_json::json!({
        "task_id": task_id,
        "status": "cancelled",
        "message": "Task cancelled successfully"
    })))
}

async fn execute_task(task_id: String, state: AppState) -> anyhow::Result<()> {
    use std::time::Duration;
    
    info!("Starting task execution: {}", task_id);

    // Update status to running
    state.update_task_status(&task_id, TaskStatus::Running).await;

    // Broadcast start event
    state.broadcaster.broadcast(BroadcastMessage::AgentEvent {
        timestamp: Utc::now(),
        agent: "system".to_string(),
        status: "running".to_string(),
        action: Some("start_task".to_string()),
        message: format!("Task {} started", task_id),
    });

    // Step 1: Create workspaces
    info!("Provisioning workspaces for task {}", task_id);
    
    // For now, mock workspace creation (will integrate real Kasm later)
    let workspace_id_1 = format!("ws-research-{}", &task_id[..8]);
    let workspace_id_2 = format!("ws-dev-{}", &task_id[..8]);
    
    state.add_workspace_to_task(&task_id, workspace_id_1.clone()).await;
    state.add_workspace_to_task(&task_id, workspace_id_2.clone()).await;

    // Broadcast workspace ready events
    tokio::time::sleep(Duration::from_secs(2)).await;
    
    state.broadcaster.broadcast(BroadcastMessage::WorkspaceReady {
        timestamp: Utc::now(),
        workspace_id: workspace_id_1.clone(),
        agent_type: "Researcher".to_string(),
        url: format!("http://localhost:6901/?workspace={}", workspace_id_1),
    });

    state.broadcaster.broadcast(BroadcastMessage::WorkspaceReady {
        timestamp: Utc::now(),
        workspace_id: workspace_id_2.clone(),
        agent_type: "Developer".to_string(),
        url: format!("http://localhost:6901/?workspace={}", workspace_id_2),
    });

    // Step 2: Execute agents (mock for now)
    let agents = vec!["ProductManager", "Architect", "Engineer"];
    
    for agent in agents {
        tokio::time::sleep(Duration::from_secs(3)).await;
        
        state.broadcaster.broadcast(BroadcastMessage::AgentEvent {
            timestamp: Utc::now(),
            agent: agent.to_string(),
            status: "thinking".to_string(),
            action: Some(format!("Process {}", agent)),
            message: format!("{} is analyzing the task", agent),
        });

        // Simulate LLM streaming
        for i in 0..5 {
            tokio::time::sleep(Duration::from_millis(500)).await;
            state.broadcaster.broadcast(BroadcastMessage::StreamChunk {
                timestamp: Utc::now(),
                agent: agent.to_string(),
                content: format!("Processing step {}...\n", i + 1),
                chunk_id: i,
            });
        }

        state.broadcaster.broadcast(BroadcastMessage::AgentEvent {
            timestamp: Utc::now(),
            agent: agent.to_string(),
            status: "complete".to_string(),
            action: Some(format!("{} completed", agent)),
            message: format!("{} finished processing", agent),
        });
    }

    // Step 3: Complete task
    tokio::time::sleep(Duration::from_secs(1)).await;
    state.update_task_status(&task_id, TaskStatus::Completed).await;

    state.broadcaster.broadcast(BroadcastMessage::TaskComplete {
        timestamp: Utc::now(),
        task_id: task_id.clone(),
        duration_seconds: 20,
    });

    info!("Task {} completed successfully", task_id);

    Ok(())
}
