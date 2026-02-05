use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use uuid::Uuid;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use crate::kasm::KasmClient;
use crate::websocket::EventBroadcaster;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub idea: String,
    pub agent_type: String,
    pub model: String,
    pub n_round: i32,
    pub status: TaskStatus,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub workspaces: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Clone)]
pub struct AppState {
    pub kasm_client: KasmClient,
    pub broadcaster: EventBroadcaster,
    pub tasks: Arc<RwLock<HashMap<String, Task>>>,
}

impl AppState {
    pub async fn new(kasm_client: KasmClient) -> Self {
        Self {
            kasm_client,
            broadcaster: EventBroadcaster::new(1000),
            tasks: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn create_task(&self, idea: String, agent_type: String, model: String, n_round: i32) -> Task {
        let task = Task {
            id: Uuid::new_v4().to_string(),
            idea,
            agent_type,
            model,
            n_round,
            status: TaskStatus::Pending,
            created_at: Utc::now(),
            completed_at: None,
            workspaces: Vec::new(),
        };

        let mut tasks = self.tasks.write().await;
        tasks.insert(task.id.clone(), task.clone());

        task
    }

    pub async fn get_task(&self, task_id: &str) -> Option<Task> {
        let tasks = self.tasks.read().await;
        tasks.get(task_id).cloned()
    }

    pub async fn update_task_status(&self, task_id: &str, status: TaskStatus) {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(task_id) {
            task.status = status.clone();
            if status == TaskStatus::Completed || status == TaskStatus::Failed {
                task.completed_at = Some(Utc::now());
            }
        }
    }

    pub async fn add_workspace_to_task(&self, task_id: &str, workspace_id: String) {
        let mut tasks = self.tasks.write().await;
        if let Some(task) = tasks.get_mut(task_id) {
            task.workspaces.push(workspace_id);
        }
    }
}
