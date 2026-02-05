use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceRequest {
    pub image_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_sharing: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_persistent_profile: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceResponse {
    pub workspace_id: String,
    pub session_token: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kasm_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkspaceStatus {
    pub workspace_id: String,
    pub status: String,
    pub operational_status: String,
}

#[derive(Debug, Clone)]
pub struct KasmConfig {
    pub base_url: String,
    pub api_key: String,
    pub api_secret: String,
}

impl KasmConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            base_url: std::env::var("KASM_API_URL")
                .unwrap_or_else(|_| "https://localhost:443".to_string()),
            api_key: std::env::var("KASM_API_KEY")
                .unwrap_or_else(|_| "mock_api_key".to_string()),
            api_secret: std::env::var("KASM_API_SECRET")
                .unwrap_or_else(|_| "mock_api_secret".to_string()),
        })
    }
}
