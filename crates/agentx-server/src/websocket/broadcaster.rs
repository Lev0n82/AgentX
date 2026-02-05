use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BroadcastMessage {
    #[serde(rename = "agent_event")]
    AgentEvent {
        timestamp: DateTime<Utc>,
        agent: String,
        status: String,
        action: Option<String>,
        message: String,
    },
    #[serde(rename = "stream_chunk")]
    StreamChunk {
        timestamp: DateTime<Utc>,
        agent: String,
        content: String,
        chunk_id: usize,
    },
    #[serde(rename = "file_created")]
    FileCreated {
        timestamp: DateTime<Utc>,
        path: String,
        name: String,
        size: usize,
        download_url: String,
    },
    #[serde(rename = "workspace_ready")]
    WorkspaceReady {
        timestamp: DateTime<Utc>,
        workspace_id: String,
        agent_type: String,
        url: String,
    },
    #[serde(rename = "task_complete")]
    TaskComplete {
        timestamp: DateTime<Utc>,
        task_id: String,
        duration_seconds: u64,
    },
    #[serde(rename = "error")]
    Error {
        timestamp: DateTime<Utc>,
        message: String,
        details: Option<String>,
    },
}

#[derive(Clone)]
pub struct EventBroadcaster {
    sender: broadcast::Sender<BroadcastMessage>,
}

impl EventBroadcaster {
    pub fn new(capacity: usize) -> Self {
        let (sender, _) = broadcast::channel(capacity);
        Self { sender }
    }

    pub fn subscribe(&self) -> broadcast::Receiver<BroadcastMessage> {
        self.sender.subscribe()
    }

    pub fn broadcast(&self, message: BroadcastMessage) {
        let _ = self.sender.send(message);
    }

    pub fn get_sender(&self) -> broadcast::Sender<BroadcastMessage> {
        self.sender.clone()
    }
}
