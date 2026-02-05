use reqwest::Client;
use tracing::{debug, error, info};
use crate::error::{Result, ServerError};
use super::types::{KasmConfig, WorkspaceRequest, WorkspaceResponse, WorkspaceStatus};

#[derive(Debug, Clone)]
pub struct KasmClient {
    config: KasmConfig,
    client: Client,
}

impl KasmClient {
    pub fn new(config: KasmConfig) -> Self {
        let client = Client::builder()
            .danger_accept_invalid_certs(true) // For self-signed certs in dev
            .build()
            .unwrap();
        
        Self { config, client }
    }

    /// Create a new Kasm workspace
    pub async fn create_workspace(&self, image_id: &str) -> Result<WorkspaceResponse> {
        info!("Creating Kasm workspace with image: {}", image_id);
        
        let url = format!("{}/api/public/request_kasm", self.config.base_url);
        
        let request_body = WorkspaceRequest {
            image_id: image_id.to_string(),
            enable_sharing: Some(true),
            enable_persistent_profile: Some(true),
        };

        debug!("Sending workspace creation request to: {}", url);

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .json(&request_body)
            .send()
            .await
            .map_err(|e| ServerError::KasmError(format!("Failed to create workspace: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Kasm API error {}: {}", status, body);
            return Err(ServerError::KasmError(format!("API returned {}: {}", status, body)));
        }

        let workspace = response.json::<WorkspaceResponse>()
            .await
            .map_err(|e| ServerError::KasmError(format!("Failed to parse response: {}", e)))?;

        info!("Workspace created successfully: {}", workspace.workspace_id);
        Ok(workspace)
    }

    /// Destroy a workspace
    pub async fn destroy_workspace(&self, workspace_id: &str) -> Result<()> {
        info!("Destroying workspace: {}", workspace_id);
        
        let url = format!("{}/api/public/destroy_kasm/{}", self.config.base_url, workspace_id);

        let response = self.client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await
            .map_err(|e| ServerError::KasmError(format!("Failed to destroy workspace: {}", e)))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            error!("Failed to destroy workspace {}: {} - {}", workspace_id, status, body);
            return Err(ServerError::KasmError(format!("Failed to destroy workspace: {}", body)));
        }

        info!("Workspace destroyed: {}", workspace_id);
        Ok(())
    }

    /// Get workspace status
    pub async fn get_workspace_status(&self, workspace_id: &str) -> Result<WorkspaceStatus> {
        debug!("Getting status for workspace: {}", workspace_id);
        
        let url = format!("{}/api/public/get_kasm/{}", self.config.base_url, workspace_id);

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .send()
            .await
            .map_err(|e| ServerError::KasmError(format!("Failed to get workspace status: {}", e)))?;

        if !response.status().is_success() {
            return Err(ServerError::KasmError("Failed to get workspace status".to_string()));
        }

        let status = response.json::<WorkspaceStatus>()
            .await
            .map_err(|e| ServerError::KasmError(format!("Failed to parse status: {}", e)))?;

        Ok(status)
    }

    /// Health check for Kasm API
    pub async fn health_check(&self) -> Result<bool> {
        debug!("Checking Kasm API health");
        
        let url = format!("{}/api/public/health", self.config.base_url);

        match self.client.get(&url).send().await {
            Ok(response) => {
                let is_healthy = response.status().is_success();
                if is_healthy {
                    info!("Kasm API is healthy");
                } else {
                    error!("Kasm API health check failed: {}", response.status());
                }
                Ok(is_healthy)
            }
            Err(e) => {
                error!("Kasm API unreachable: {}", e);
                Ok(false)
            }
        }
    }
}
