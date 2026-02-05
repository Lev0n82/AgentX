# AgentX + Kasm Workspaces Implementation Plan

## Executive Summary

Transform AgentX from a CLI-only tool into a comprehensive browser-based multi-agent platform with live visual workspace streaming using Kasm Workspaces for agent desktop environments.

**Timeline:** 6-8 weeks
**Complexity:** Medium-High
**Primary Benefits:** 
- Live visual feedback of agent actions (browsing, coding, testing)
- Browser-based access from any device
- Multi-agent concurrent workspaces
- Production-ready streaming infrastructure

---

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                    Browser: http://localhost:3000               │
├──────────────────┬──────────────────┬──────────────────────────┤
│  Agent Control   │  Workflow Canvas │  Live Workspace Stream   │
│  Dashboard       │  (React Flow)    │  (Kasm iframe embed)     │
│  - Task Input    │  - Agent Nodes   │  - Browser Activity      │
│  - Model Select  │  - Message Flow  │  - Code Execution        │
│  - Start/Stop    │  - Status Colors │  - File Editing          │
│  - Settings      │  - Progress      │  - Testing Output        │
├──────────────────┴──────────────────┴──────────────────────────┤
│  Terminal Logs Stream              │  Files & Artifacts        │
│  - Real-time agent messages        │  - Generated files tree   │
│  - LLM responses                   │  - Download links         │
│  - System events                   │  - Monaco editor preview  │
└────────────────────────────────────┴──────────────────────────┘
                          ▲                    ▲
                          │                    │
                    WebSocket              HTTP/REST
                          │                    │
┌─────────────────────────┴────────────────────┴─────────────────┐
│                   AgentX Web Server (Axum/Rust)                 │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  Core Services                                           │  │
│  │  - WebSocket event streaming                            │  │
│  │  - REST API endpoints                                   │  │
│  │  - Agent lifecycle management                           │  │
│  │  - Task queue & state tracking                          │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  Kasm Integration Layer                                  │  │
│  │  - Workspace provisioning API                           │  │
│  │  - Session management                                   │  │
│  │  - Agent-to-workspace mapping                           │  │
│  │  - Control commands (browser automation, file ops)      │  │
│  └──────────────────────────────────────────────────────────┘  │
│  ┌──────────────────────────────────────────────────────────┐  │
│  │  AgentX Core (Existing)                                  │  │
│  │  - SoftwareCompany                                       │  │
│  │  - Environment & Agents                                  │  │
│  │  - LLM Provider (Ollama integration)                     │  │
│  └──────────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Kasm Workspaces                              │
│  ┌──────────────┬──────────────┬──────────────┬──────────────┐ │
│  │ Workspace 1  │ Workspace 2  │ Workspace 3  │ Workspace 4  │ │
│  │ (Research)   │ (Dev)        │ (Test)       │ (Browser)    │ │
│  │ - Firefox    │ - VS Code    │ - Terminal   │ - Chrome     │ │
│  │ - Terminal   │ - Python     │ - pytest     │ - DevTools   │ │
│  └──────────────┴──────────────┴──────────────┴──────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Ollama LLM Service                           │
│  - qwen2.5-coder:3b                                            │
│  - glm-ocr                                                      │
│  - Other models                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## Phase 1: Foundation & Infrastructure Setup

**Duration:** Week 1-2

### 1.1 Kasm Workspaces Deployment

**Tasks:**
- [ ] Install Kasm Workspaces on host system
- [ ] Configure Kasm with Podman (instead of Docker)
- [ ] Set up admin account and initial configuration
- [ ] Create custom workspace images for AgentX
- [ ] Configure network isolation and security

**Technical Specifications:**
```yaml
# kasm-deployment/podman-compose.yml
version: '3.8'

services:
  kasm-db:
    image: postgres:12-alpine
    container_name: kasm_db
    environment:
      POSTGRES_DB: kasm
      POSTGRES_USER: kasmapp
      POSTGRES_PASSWORD: ${KASM_DB_PASSWORD}
    volumes:
      - kasm-db-data:/var/lib/postgresql/data
    networks:
      - kasm-network
    restart: unless-stopped

  kasm-redis:
    image: redis:6-alpine
    container_name: kasm_redis
    networks:
      - kasm-network
    restart: unless-stopped

  kasm-manager:
    image: kasmweb/manager:1.14.0
    container_name: kasm_manager
    depends_on:
      - kasm-db
      - kasm-redis
    environment:
      DATABASE_URL: postgresql://kasmapp:${KASM_DB_PASSWORD}@kasm-db/kasm
      REDIS_URL: redis://kasm-redis:6379
      SERVER_HOSTNAME: ${KASM_HOSTNAME:-localhost}
    ports:
      - "443:443"
    networks:
      - kasm-network
    volumes:
      - kasm-data:/opt/kasm/current
    restart: unless-stopped

  kasm-agent:
    image: kasmweb/agent:1.14.0
    container_name: kasm_agent
    depends_on:
      - kasm-manager
    environment:
      MANAGER_HOSTNAME: kasm-manager
      SERVER_ID: ${KASM_AGENT_ID:-agent1}
    networks:
      - kasm-network
    volumes:
      - /var/run/podman/podman.sock:/var/run/docker.sock
      - kasm-profiles:/mnt/kasm_profiles
    privileged: true
    restart: unless-stopped

networks:
  kasm-network:
    driver: bridge

volumes:
  kasm-db-data:
  kasm-data:
  kasm-profiles:
```

**Custom Workspace Images:**
```dockerfile
# kasm-workspaces/agentx-researcher/Dockerfile
FROM kasmweb/firefox:1.14.0

USER root

# Install research tools
RUN apt-get update && apt-get install -y \
    python3-pip \
    git \
    curl \
    wget \
    jq \
    && rm -rf /var/lib/apt/lists/*

# Install Python research libraries
RUN pip3 install --no-cache-dir \
    requests \
    beautifulsoup4 \
    selenium \
    arxiv \
    scholarly

# Configure Firefox for automation
COPY firefox-prefs.js /etc/firefox/syspref.js

# Set up workspace directory
RUN mkdir -p /workspace && chown kasm-user:kasm-user /workspace

USER kasm-user
WORKDIR /workspace

# Expose Kasm ports
EXPOSE 6901

ENTRYPOINT ["/dockerstartup/kasm_default_profile.sh", "/dockerstartup/vnc_startup.sh", "/dockerstartup/kasm_startup.sh"]
```

```dockerfile
# kasm-workspaces/agentx-developer/Dockerfile
FROM kasmweb/ubuntu-jammy-desktop:1.14.0

USER root

# Install development tools
RUN apt-get update && apt-get install -y \
    python3.11 \
    python3.11-venv \
    python3-pip \
    nodejs \
    npm \
    git \
    vim \
    code \
    tmux \
    && rm -rf /var/lib/apt/lists/*

# Install VS Code extensions (common for development)
RUN code --install-extension ms-python.python \
    --install-extension ms-python.vscode-pylance \
    --install-extension dbaeumer.vscode-eslint

# Set up workspace
RUN mkdir -p /workspace && chown kasm-user:kasm-user /workspace

USER kasm-user
WORKDIR /workspace

EXPOSE 6901

ENTRYPOINT ["/dockerstartup/kasm_default_profile.sh", "/dockerstartup/vnc_startup.sh", "/dockerstartup/kasm_startup.sh"]
```

**Acceptance Criteria:**

✅ **AC-1.1.1:** Kasm Workspaces successfully deployed with Podman
- [ ] All containers running: `podman ps` shows kasm-manager, kasm-agent, kasm-db, kasm-redis
- [ ] Web interface accessible at https://localhost:443
- [ ] Admin login successful
- [ ] Zero critical errors in logs: `podman logs kasm-manager`

✅ **AC-1.1.2:** Custom workspace images built and registered
- [ ] `agentx-researcher` image built successfully
- [ ] `agentx-developer` image built successfully
- [ ] Both images appear in Kasm workspace catalog
- [ ] Test launch of each image successful (manual verification)

✅ **AC-1.1.3:** Kasm API credentials configured
- [ ] API token generated in Kasm admin panel
- [ ] Token stored securely in AgentX configuration
- [ ] Test API call successful: `GET /api/workspaces` returns 200

### 1.2 AgentX Web Server Foundation

**Tasks:**
- [ ] Create new Rust crate: `crates/agentx-server`
- [ ] Set up Axum web framework
- [ ] Configure WebSocket support (tokio-tungstenite)
- [ ] Create project structure for web server
- [ ] Set up CORS and security middleware

**Technical Specifications:**

```toml
# crates/agentx-server/Cargo.toml
[package]
name = "agentx-server"
version = "0.1.0"
edition = "2021"

[dependencies]
agentx_core = { path = "../agentx" }
agent_roles = { path = "../agent_roles" }

# Web framework
axum = { version = "0.7", features = ["ws", "macros"] }
tower = "0.4"
tower-http = { version = "0.5", features = ["cors", "trace", "fs"] }

# Async runtime
tokio = { version = "1", features = ["full"] }

# WebSocket
tokio-tungstenite = "0.21"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# HTTP client (for Kasm API)
reqwest = { version = "0.11", features = ["json"] }

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Environment
dotenv = "0.15"

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# UUID for session tracking
uuid = { version = "1.4", features = ["v4", "serde"] }
```

**Directory Structure:**
```
crates/agentx-server/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lib.rs
│   ├── api/
│   │   ├── mod.rs
│   │   ├── tasks.rs          # Task management endpoints
│   │   ├── workspaces.rs     # Kasm workspace endpoints
│   │   └── files.rs          # File download endpoints
│   ├── websocket/
│   │   ├── mod.rs
│   │   ├── handler.rs        # WebSocket connection handler
│   │   └── broadcaster.rs    # Event broadcasting
│   ├── kasm/
│   │   ├── mod.rs
│   │   ├── client.rs         # Kasm API client
│   │   ├── workspace.rs      # Workspace management
│   │   └── types.rs          # Kasm data types
│   ├── agent/
│   │   ├── mod.rs
│   │   ├── executor.rs       # Agent execution wrapper
│   │   └── events.rs         # Agent event emission
│   ├── state/
│   │   ├── mod.rs
│   │   └── app_state.rs      # Shared application state
│   └── error.rs              # Error types
└── tests/
    ├── api_tests.rs
    └── websocket_tests.rs
```

**Core Server Implementation:**

```rust
// crates/agentx-server/src/main.rs
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use std::net::SocketAddr;
use tracing::info;

mod api;
mod websocket;
mod kasm;
mod agent;
mod state;
mod error;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Load configuration
    dotenv::dotenv().ok();
    
    // Initialize shared state
    let app_state = state::AppState::new().await?;

    // Build application router
    let app = Router::new()
        // API routes
        .route("/api/v1/health", get(api::health_check))
        .route("/api/v1/tasks", post(api::tasks::create_task))
        .route("/api/v1/tasks/:id", get(api::tasks::get_task_status))
        .route("/api/v1/tasks/:id/cancel", post(api::tasks::cancel_task))
        .route("/api/v1/workspaces", get(api::workspaces::list_workspaces))
        .route("/api/v1/files/:task_id/:filename", get(api::files::download_file))
        
        // WebSocket endpoint
        .route("/ws", get(websocket::handler::ws_handler))
        
        // Static files (will serve React build)
        .nest_service("/", tower_http::services::ServeDir::new("./web-ui/dist"))
        
        // Add state and middleware
        .with_state(app_state)
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http());

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    info!("AgentX Web Server starting on {}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
```

```rust
// crates/agentx-server/src/kasm/client.rs
use reqwest::{Client, header};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct KasmClient {
    base_url: String,
    api_key: String,
    api_key_secret: String,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceRequest {
    pub image_id: String,
    pub enable_sharing: bool,
    pub enable_persistent_profile: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspaceResponse {
    pub workspace_id: String,
    pub session_token: String,
    pub url: String,
}

impl KasmClient {
    pub fn new(base_url: String, api_key: String, api_key_secret: String) -> Self {
        let client = Client::new();
        Self {
            base_url,
            api_key,
            api_key_secret,
            client,
        }
    }

    /// Create a new Kasm workspace
    pub async fn create_workspace(&self, image_id: &str) -> Result<WorkspaceResponse> {
        let url = format!("{}/api/public/request_kasm", self.base_url);
        
        let request_body = WorkspaceRequest {
            image_id: image_id.to_string(),
            enable_sharing: true,
            enable_persistent_profile: true,
        };

        let response = self.client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request_body)
            .send()
            .await?;

        let workspace = response.json::<WorkspaceResponse>().await?;
        Ok(workspace)
    }

    /// Destroy a workspace
    pub async fn destroy_workspace(&self, workspace_id: &str) -> Result<()> {
        let url = format!("{}/api/public/destroy_kasm/{}", self.base_url, workspace_id);
        
        self.client
            .delete(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        Ok(())
    }

    /// Get workspace status
    pub async fn get_workspace_status(&self, workspace_id: &str) -> Result<String> {
        let url = format!("{}/api/public/get_kasm/{}", self.base_url, workspace_id);
        
        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .send()
            .await?;

        let status = response.text().await?;
        Ok(status)
    }
}
```

**Acceptance Criteria:**

✅ **AC-1.2.1:** AgentX web server compiles and runs
- [ ] `cargo build` in `crates/agentx-server` succeeds with 0 errors
- [ ] Server starts: `cargo run --bin agentx-server`
- [ ] Health check responds: `curl http://localhost:8080/api/v1/health` returns 200

✅ **AC-1.2.2:** Kasm API client functional
- [ ] Create workspace test succeeds: workspace ID returned
- [ ] Workspace URL accessible in browser
- [ ] Destroy workspace test succeeds: workspace terminates
- [ ] Error handling: invalid API key returns proper error

✅ **AC-1.2.3:** WebSocket connection established
- [ ] Client can connect to `ws://localhost:8080/ws`
- [ ] Connection stays alive for 60+ seconds
- [ ] Ping/pong heartbeat working
- [ ] Multiple clients can connect simultaneously (test with 5 connections)

---

## Phase 2: Frontend Development & Model Arena

**Duration:** Week 3-5

### 2.1 React Application Setup

**Tasks:**
- [ ] Initialize React + TypeScript project with Vite
- [ ] Set up TailwindCSS
- [ ] Configure React Flow for canvas
- [ ] Install Monaco Editor
- [ ] Set up WebSocket client library
- [ ] Create base component structure

**Technical Specifications:**

```bash
# Create React app
cd /home/levon/Documents/AgentX
npm create vite@latest web-ui -- --template react-ts
cd web-ui
npm install

# Install dependencies
npm install \
  react-flow-renderer \
  @monaco-editor/react \
  zustand \
  axios \
  tailwindcss \
  @headlessui/react \
  @heroicons/react \
  date-fns \
  react-hot-toast

# Initialize Tailwind
npx tailwindcss init -p
```

```typescript
// web-ui/src/types/index.ts
export interface Agent {
  id: string;
  type: 'ProductManager' | 'Architect' | 'Engineer' | 'ProjectManager';
  status: 'idle' | 'thinking' | 'acting' | 'waiting' | 'complete' | 'error';
  currentAction?: string;
  progress?: number;
  workspaceId?: string;
  workspaceUrl?: string;
}

export interface Task {
  id: string;
  idea: string;
  agent: string;
  model: string;
  nRound: number;
  status: 'pending' | 'running' | 'completed' | 'failed';
  createdAt: Date;
  completedAt?: Date;
  agents: Agent[];
  files: GeneratedFile[];
}

export interface GeneratedFile {
  path: string;
  name: string;
  type: 'python' | 'markdown' | 'yaml' | 'json' | 'image';
  size: number;
  createdAt: Date;
  downloadUrl: string;
}

export interface WebSocketMessage {
  type: 'agent_event' | 'stream_chunk' | 'file_created' | 'task_complete' | 'error';
  timestamp: string;
  data: any;
}

export interface KasmWorkspace {
  id: string;
  agentType: string;
  url: string;
  sessionToken: string;
  status: 'starting' | 'running' | 'stopped';
}
```

**Component Structure:**
```
web-ui/src/
├── App.tsx
├── main.tsx
├── types/
│   └── index.ts
├── hooks/
│   ├── useWebSocket.ts
│   ├── useTaskManager.ts
│   └── useKasmWorkspace.ts
├── store/
│   └── appStore.ts                 # Zustand state management
├── services/
│   ├── api.ts                      # REST API client
│   └── websocket.ts                # WebSocket client
├── components/
│   ├── Layout/
│   │   ├── Header.tsx
│   │   └── Sidebar.tsx
│   ├── Dashboard/
│   │   ├── TaskControl.tsx         # Task input & controls
│   │   └── ModelSelector.tsx
│   ├── Canvas/
│   │   ├── AgentFlowCanvas.tsx     # React Flow canvas
│   │   ├── AgentNode.tsx
│   │   └── MessageEdge.tsx
│   ├── Workspace/
│   │   ├── KasmViewer.tsx          # Kasm iframe embed
│   │   └── WorkspaceSelector.tsx
│   ├── Terminal/
│   │   └── LogStream.tsx           # Live log streaming
│   ├── Files/
│   │   ├── FileExplorer.tsx
│   │   └── FilePreview.tsx         # Monaco editor
│   └── common/
│       ├── Button.tsx
│       ├── Card.tsx
│       └── Spinner.tsx
└── utils/
    ├── formatters.ts
    └── constants.ts
```

**Main Application Layout:**

```typescript
// web-ui/src/App.tsx
import { useState } from 'react';
import { Header } from './components/Layout/Header';
import { TaskControl } from './components/Dashboard/TaskControl';
import { AgentFlowCanvas } from './components/Canvas/AgentFlowCanvas';
import { KasmViewer } from './components/Workspace/KasmViewer';
import { LogStream } from './components/Terminal/LogStream';
import { FileExplorer } from './components/Files/FileExplorer';
import { useWebSocket } from './hooks/useWebSocket';
import { useAppStore } from './store/appStore';

function App() {
  const { currentTask, agents, workspaces } = useAppStore();
  const [activeView, setActiveView] = useState<'canvas' | 'workspace'>('canvas');
  
  useWebSocket('ws://localhost:8080/ws');

  return (
    <div className="h-screen flex flex-col bg-gray-900 text-white">
      <Header />
      
      <div className="flex-1 flex overflow-hidden">
        {/* Left Sidebar - Task Control */}
        <div className="w-80 border-r border-gray-700 p-4 overflow-y-auto">
          <TaskControl />
        </div>

        {/* Main Content Area - 2x2 Grid */}
        <div className="flex-1 grid grid-cols-2 grid-rows-2 gap-2 p-2">
          {/* Top Left - Agent Flow Canvas */}
          <div className="border border-gray-700 rounded-lg overflow-hidden">
            <div className="bg-gray-800 px-4 py-2 border-b border-gray-700">
              <h3 className="font-semibold">Agent Workflow</h3>
            </div>
            <AgentFlowCanvas agents={agents} />
          </div>

          {/* Top Right - Kasm Workspace View */}
          <div className="border border-gray-700 rounded-lg overflow-hidden">
            <div className="bg-gray-800 px-4 py-2 border-b border-gray-700 flex justify-between items-center">
              <h3 className="font-semibold">Agent Workspace</h3>
              {workspaces.length > 0 && (
                <select className="bg-gray-700 rounded px-2 py-1 text-sm">
                  {workspaces.map(ws => (
                    <option key={ws.id} value={ws.id}>
                      {ws.agentType} Workspace
                    </option>
                  ))}
                </select>
              )}
            </div>
            <KasmViewer workspace={workspaces[0]} />
          </div>

          {/* Bottom Left - Terminal Logs */}
          <div className="border border-gray-700 rounded-lg overflow-hidden">
            <div className="bg-gray-800 px-4 py-2 border-b border-gray-700">
              <h3 className="font-semibold">Live Logs</h3>
            </div>
            <LogStream />
          </div>

          {/* Bottom Right - File Explorer */}
          <div className="border border-gray-700 rounded-lg overflow-hidden">
            <div className="bg-gray-800 px-4 py-2 border-b border-gray-700">
              <h3 className="font-semibold">Generated Files</h3>
            </div>
            <FileExplorer />
          </div>
        </div>
      </div>
    </div>
  );
}

export default App;
```

**Kasm Viewer Component:**

```typescript
// web-ui/src/components/Workspace/KasmViewer.tsx
import { useEffect, useState } from 'react';
import { KasmWorkspace } from '../../types';

interface KasmViewerProps {
  workspace?: KasmWorkspace;
}

export function KasmViewer({ workspace }: KasmViewerProps) {
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    if (workspace) {
      setLoading(true);
      // Iframe will trigger onLoad when ready
    }
  }, [workspace?.id]);

  if (!workspace) {
    return (
      <div className="h-full flex items-center justify-center bg-gray-800">
        <div className="text-center text-gray-400">
          <svg className="w-16 h-16 mx-auto mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9.75 17L9 20l-1 1h8l-1-1-.75-3M3 13h18M5 17h14a2 2 0 002-2V5a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
          </svg>
          <p className="text-lg font-medium">No Active Workspace</p>
          <p className="text-sm mt-2">Start a task to launch an agent workspace</p>
        </div>
      </div>
    );
  }

  return (
    <div className="h-full relative bg-gray-800">
      {loading && (
        <div className="absolute inset-0 flex items-center justify-center bg-gray-800 z-10">
          <div className="text-center">
            <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-blue-500 mx-auto mb-4"></div>
            <p className="text-gray-400">Loading workspace...</p>
          </div>
        </div>
      )}
      
      <iframe
        src={workspace.url}
        className="w-full h-full"
        onLoad={() => setLoading(false)}
        title={`${workspace.agentType} Workspace`}
        sandbox="allow-same-origin allow-scripts allow-forms allow-downloads"
      />
      
      {workspace.status === 'starting' && (
        <div className="absolute top-2 right-2 bg-yellow-500 text-black px-3 py-1 rounded-full text-sm font-medium">
          Starting...
        </div>
      )}
    </div>
  );
}
```

**WebSocket Hook:**

```typescript
// web-ui/src/hooks/useWebSocket.ts
import { useEffect, useRef } from 'react';
import { useAppStore } from '../store/appStore';
import { WebSocketMessage } from '../types';
import toast from 'react-hot-toast';

export function useWebSocket(url: string) {
  const ws = useRef<WebSocket | null>(null);
  const { addLogEntry, updateAgentStatus, addGeneratedFile, updateWorkspace } = useAppStore();

  useEffect(() => {
    // Connect to WebSocket
    ws.current = new WebSocket(url);

    ws.current.onopen = () => {
      console.log('WebSocket connected');
      toast.success('Connected to AgentX server');
    };

    ws.current.onmessage = (event) => {
      try {
        const message: WebSocketMessage = JSON.parse(event.data);
        
        switch (message.type) {
          case 'agent_event':
            updateAgentStatus(message.data.agent, message.data.status, message.data.action);
            addLogEntry({
              timestamp: message.timestamp,
              agent: message.data.agent,
              level: 'info',
              message: message.data.message,
            });
            break;

          case 'stream_chunk':
            addLogEntry({
              timestamp: message.timestamp,
              agent: message.data.agent,
              level: 'stream',
              message: message.data.content,
            });
            break;

          case 'file_created':
            addGeneratedFile(message.data);
            toast.success(`File created: ${message.data.name}`);
            break;

          case 'workspace_ready':
            updateWorkspace(message.data.workspaceId, {
              url: message.data.url,
              status: 'running',
            });
            break;

          case 'error':
            toast.error(message.data.message);
            addLogEntry({
              timestamp: message.timestamp,
              agent: 'system',
              level: 'error',
              message: message.data.message,
            });
            break;

          case 'task_complete':
            toast.success('Task completed!');
            break;
        }
      } catch (error) {
        console.error('Failed to parse WebSocket message:', error);
      }
    };

    ws.current.onerror = (error) => {
      console.error('WebSocket error:', error);
      toast.error('Connection error');
    };

    ws.current.onclose = () => {
      console.log('WebSocket disconnected');
      toast.error('Disconnected from server');
    };

    // Cleanup on unmount
    return () => {
      if (ws.current) {
        ws.current.close();
      }
    };
  }, [url]);

  return {
    send: (data: any) => {
      if (ws.current && ws.current.readyState === WebSocket.OPEN) {
        ws.current.send(JSON.stringify(data));
      }
    },
  };
}
```

**Acceptance Criteria:**

✅ **AC-2.1.1:** React application builds and runs
- [ ] `npm run dev` starts dev server on port 5173
- [ ] Browser shows AgentX UI without errors
- [ ] All components render without TypeScript errors
- [ ] Tailwind styles applied correctly

✅ **AC-2.1.2:** WebSocket connection functional
- [ ] Frontend connects to backend WebSocket
- [ ] Connection indicator shows "Connected" status
- [ ] Receives and displays test message from server
- [ ] Reconnects automatically on disconnect (within 5 seconds)

✅ **AC-2.1.3:** Kasm iframe embeds correctly
- [ ] Iframe loads Kasm workspace URL
- [ ] Workspace visible and interactive in browser
- [ ] Full-screen toggle works
- [ ] No CORS or iframe blocking errors

### 2.2 Agent Flow Canvas Implementation

**Tasks:**
- [ ] Implement React Flow integration
- [ ] Create custom agent node components
- [ ] Add message flow edges
- [ ] Implement real-time status updates
- [ ] Add animations for message passing

**Technical Specifications:**

```typescript
// web-ui/src/components/Canvas/AgentFlowCanvas.tsx
import ReactFlow, {
  Background,
  Controls,
  MiniMap,
  Node,
  Edge,
  useNodesState,
  useEdgesState,
} from 'react-flow-renderer';
import { useEffect } from 'react';
import { Agent } from '../../types';
import { AgentNode } from './AgentNode';

const nodeTypes = {
  agent: AgentNode,
};

interface AgentFlowCanvasProps {
  agents: Agent[];
}

export function AgentFlowCanvas({ agents }: AgentFlowCanvasProps) {
  const [nodes, setNodes, onNodesChange] = useNodesState([]);
  const [edges, setEdges, onEdgesChange] = useEdgesState([]);

  // Convert agents to React Flow nodes
  useEffect(() => {
    const flowNodes: Node[] = [
      {
        id: 'boss',
        type: 'input',
        data: { label: 'Boss/User' },
        position: { x: 250, y: 0 },
        style: { background: '#4F46E5', color: 'white', border: 'none' },
      },
      ...agents.map((agent, index) => ({
        id: agent.id,
        type: 'agent',
        data: agent,
        position: { x: 100 + index * 200, y: 150 + (index % 2) * 100 },
      })),
    ];

    const flowEdges: Edge[] = [
      {
        id: 'boss-pm',
        source: 'boss',
        target: agents.find(a => a.type === 'ProductManager')?.id || '',
        animated: true,
        style: { stroke: '#4F46E5' },
      },
      // Add more edges based on agent dependencies
    ];

    setNodes(flowNodes);
    setEdges(flowEdges);
  }, [agents]);

  return (
    <div className="h-full bg-gray-900">
      <ReactFlow
        nodes={nodes}
        edges={edges}
        onNodesChange={onNodesChange}
        onEdgesChange={onEdgesChange}
        nodeTypes={nodeTypes}
        fitView
      >
        <Background color="#374151" gap={16} />
        <Controls className="bg-gray-800 border-gray-700" />
        <MiniMap
          nodeColor={(node) => {
            const agent = node.data as Agent;
            switch (agent.status) {
              case 'acting': return '#10B981';
              case 'thinking': return '#F59E0B';
              case 'complete': return '#3B82F6';
              case 'error': return '#EF4444';
              default: return '#6B7280';
            }
          }}
          className="bg-gray-800 border border-gray-700"
        />
      </ReactFlow>
    </div>
  );
}
```

```typescript
// web-ui/src/components/Canvas/AgentNode.tsx
import { Handle, Position } from 'react-flow-renderer';
import { Agent } from '../../types';

interface AgentNodeProps {
  data: Agent;
}

export function AgentNode({ data }: AgentNodeProps) {
  const statusColors = {
    idle: 'bg-gray-600',
    thinking: 'bg-yellow-500',
    acting: 'bg-green-500',
    waiting: 'bg-blue-500',
    complete: 'bg-indigo-500',
    error: 'bg-red-500',
  };

  const statusIcons = {
    idle: '⏸️',
    thinking: '🤔',
    acting: '⚡',
    waiting: '⏳',
    complete: '✅',
    error: '❌',
  };

  return (
    <div className="bg-gray-800 border-2 border-gray-700 rounded-lg shadow-lg min-w-[200px]">
      <Handle type="target" position={Position.Top} className="w-3 h-3" />
      
      <div className={`px-4 py-2 ${statusColors[data.status]} bg-opacity-20 border-b border-gray-700`}>
        <div className="flex items-center justify-between">
          <span className="font-semibold text-white">{data.type}</span>
          <span className="text-2xl">{statusIcons[data.status]}</span>
        </div>
      </div>

      <div className="px-4 py-3 space-y-2">
        <div className="text-sm text-gray-300">
          <div className="font-medium text-gray-400">Status:</div>
          <div className="capitalize">{data.status}</div>
        </div>

        {data.currentAction && (
          <div className="text-sm text-gray-300">
            <div className="font-medium text-gray-400">Action:</div>
            <div className="truncate">{data.currentAction}</div>
          </div>
        )}

        {data.progress !== undefined && (
          <div className="space-y-1">
            <div className="flex justify-between text-xs text-gray-400">
              <span>Progress</span>
              <span>{data.progress}%</span>
            </div>
            <div className="w-full bg-gray-700 rounded-full h-2">
              <div
                className="bg-blue-500 h-2 rounded-full transition-all duration-300"
                style={{ width: `${data.progress}%` }}
              />
            </div>
          </div>
        )}

        {data.workspaceUrl && (
          <a
            href={data.workspaceUrl}
            target="_blank"
            rel="noopener noreferrer"
            className="block text-xs text-blue-400 hover:text-blue-300 truncate"
          >
            🖥️ View Workspace
          </a>
        )}
      </div>

      <Handle type="source" position={Position.Bottom} className="w-3 h-3" />
    </div>
  );
}
```

**Acceptance Criteria:**

✅ **AC-2.2.1:** Agent nodes render correctly
- [ ] All 4 agent types displayed (PM, Architect, Engineer, Project Manager)
- [ ] Nodes positioned in logical workflow order
- [ ] Status colors update in real-time
- [ ] Progress bars animate smoothly

✅ **AC-2.2.2:** Message flow visualization works
- [ ] Edges connect agents in correct order
- [ ] Messages animate along edges when agent acts
- [ ] Edge color reflects message type
- [ ] Animation speed configurable

✅ **AC-2.2.3:** Canvas interactions functional
- [ ] Pan/zoom works smoothly
- [ ] MiniMap shows overview
- [ ] Controls allow fit-to-view
- [ ] Double-click agent opens workspace view

---

### 2.3 Model Arena & Benchmarking System 🆕

**Purpose:** Compare LLM performance side-by-side on identical tasks to help users select the best model for their agent workflows.

**Tasks:**
- [ ] Design arena UI with model selection and comparison views
- [ ] Implement backend arena API endpoints
- [ ] Create benchmark test suite management
- [ ] Build concurrent model execution engine
- [ ] Implement automatic evaluation metrics
- [ ] Create leaderboard and historical tracking
- [ ] Add custom test creation interface

**Architecture:**

```
┌─────────────────────────────────────────────────────────────────┐
│                    Model Arena Dashboard                        │
├──────────────────┬──────────────────┬──────────────────────────┤
│  Model Selection │  Benchmark Suite │  Live Results Table      │
│  ☐ qwen2.5:3b   │  ☐ Code Gen      │  Model    │ Score │ Time │
│  ☐ llama3.2:3b  │  ☐ Research      │  qwen2.5  │ 0.89  │ 4.2s │
│  ☐ codellama    │  ☐ Debugging     │  llama3.2 │ 0.76  │ 6.1s │
│  ☐ deepseek     │  ☐ API Design    │  codellama│ 0.92  │ 3.8s │
│  [Add Model]    │  [Custom Test]   │  deepseek │ 0.88  │ 5.0s │
├─────────────────┴──────────────────┴──────────────────────────┤
│  Side-by-Side Output Comparison                                │
│  ┌─────────────────────┬─────────────────────┐                │
│  │ qwen2.5:3b          │ codellama:7b        │                │
│  │ ─────────────────── │ ─────────────────── │                │
│  │ def calculate_sum(  │ def sum_numbers(    │                │
│  │     numbers: list   │     arr: List[int]  │                │
│  │ ) -> int:           │ ) -> int:           │                │
│  │     """Sum list"""  │     """Calculates   │                │
│  │     return sum(     │     the sum"""      │                │
│  │         numbers     │     total = 0       │                │
│  │     )               │     for num in arr: │                │
│  │                     │         total += num│                │
│  │ ✅ Concise          │     return total    │                │
│  │ ⚡ Fast (2.1s)      │ ⚠️  Verbose         │                │
│  │ 📏 18 tokens        │ ⚡ Medium (3.4s)    │                │
│  │                     │ 📏 42 tokens        │                │
│  └─────────────────────┴─────────────────────┘                │
├─────────────────────────────────────────────────────────────────┤
│  Metrics Visualization                                          │
│  📊 Response Time Chart  │  📈 Token Efficiency  │  🎯 Accuracy│
└─────────────────────────────────────────────────────────────────┘
```

#### 2.3.1 Backend API - Arena Endpoints

**Create new file:** `crates/agentx-server/src/api/arena.rs`

```rust
// Arena API endpoints for model benchmarking

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    error::ServerError,
    state::AppState,
    websocket::broadcaster::{EventBroadcaster, ArenaBroadcastMessage},
};

// Data structures
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BenchmarkTest {
    pub id: String,
    pub name: String,
    pub category: String,  // "code_gen", "research", "debugging", etc.
    pub prompt: String,
    pub expected_criteria: ExpectedCriteria,
    pub rubric: EvaluationRubric,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpectedCriteria {
    pub keywords: Vec<String>,       // Must contain these
    pub max_tokens: Option<usize>,   // Token efficiency target
    pub max_time_ms: Option<u64>,    // Speed requirement
    pub code_executes: Option<bool>, // For code gen tests
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EvaluationRubric {
    pub correctness_weight: f32,  // 0.0-1.0
    pub efficiency_weight: f32,   // 0.0-1.0
    pub clarity_weight: f32,      // 0.0-1.0
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkRequest {
    pub models: Vec<String>,        // e.g., ["qwen2.5-coder:3b", "codellama:7b"]
    pub test_ids: Vec<String>,      // Which tests to run
    pub config: Option<BenchmarkConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BenchmarkConfig {
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub max_tokens: Option<usize>,
    pub parallel: bool,  // Run models concurrently
}

#[derive(Debug, Serialize)]
pub struct BenchmarkResponse {
    pub benchmark_id: String,
    pub status: String,
    pub models: Vec<String>,
    pub tests: Vec<String>,
    pub started_at: String,
}

#[derive(Debug, Serialize)]
pub struct BenchmarkStatus {
    pub id: String,
    pub status: String,  // "pending", "running", "completed", "failed"
    pub progress: f32,   // 0.0-1.0
    pub results: Option<HashMap<String, Vec<TestResult>>>,  // model_id -> results
    pub leaderboard: Option<Vec<ModelPerformance>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestResult {
    pub test_id: String,
    pub model_id: String,
    pub output: String,
    pub metrics: TestMetrics,
    pub timestamp: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TestMetrics {
    pub response_time_ms: u64,
    pub total_tokens: usize,
    pub tokens_per_sec: f32,
    pub correctness_score: f32,    // 0.0-1.0
    pub efficiency_score: f32,     // 0.0-1.0
    pub clarity_score: f32,        // 0.0-1.0
    pub overall_score: f32,        // Weighted composite
    pub keyword_matches: usize,
    pub code_executed: Option<bool>,
}

#[derive(Debug, Serialize, Clone)]
pub struct ModelPerformance {
    pub model_id: String,
    pub total_tests: usize,
    pub wins: usize,                // Times model scored best
    pub avg_response_time_ms: f64,
    pub avg_tokens_per_sec: f64,
    pub avg_correctness: f64,
    pub avg_efficiency: f64,
    pub avg_clarity: f64,
    pub overall_score: f64,
}

#[derive(Debug, Deserialize)]
pub struct LeaderboardQuery {
    pub category: Option<String>,
    pub limit: Option<usize>,
}

// API Handlers

/// POST /api/v1/arena/benchmark
/// Start a new benchmark run across multiple models
pub async fn create_benchmark(
    State(state): State<AppState>,
    Json(req): Json<BenchmarkRequest>,
) -> Result<Json<BenchmarkResponse>, ServerError> {
    let benchmark_id = Uuid::new_v4().to_string();
    let started_at = chrono::Utc::now().to_rfc3339();
    
    // Spawn background task to execute benchmark
    let state_clone = state.clone();
    let models = req.models.clone();
    let test_ids = req.test_ids.clone();
    
    tokio::spawn(async move {
        if let Err(e) = execute_benchmark(
            state_clone,
            benchmark_id.clone(),
            models,
            test_ids,
            req.config,
        ).await {
            eprintln!("Benchmark execution error: {}", e);
        }
    });
    
    Ok(Json(BenchmarkResponse {
        benchmark_id,
        status: "running".to_string(),
        models: req.models,
        tests: req.test_ids,
        started_at,
    }))
}

/// GET /api/v1/arena/benchmark/:id
/// Get benchmark run status and results
pub async fn get_benchmark_status(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> Result<Json<BenchmarkStatus>, ServerError> {
    // TODO: Retrieve from database/state
    Ok(Json(BenchmarkStatus {
        id,
        status: "completed".to_string(),
        progress: 1.0,
        results: None,
        leaderboard: None,
    }))
}

/// GET /api/v1/arena/models
/// List available Ollama models
pub async fn list_models() -> Result<Json<Vec<ModelInfo>>, ServerError> {
    // Call Ollama API to list models
    let client = reqwest::Client::new();
    let response = client
        .get("http://localhost:11434/api/tags")
        .send()
        .await
        .map_err(|e| ServerError::Internal(format!("Ollama API error: {}", e)))?;
    
    let models: OllamaModelsResponse = response.json().await
        .map_err(|e| ServerError::Internal(format!("Parse error: {}", e)))?;
    
    Ok(Json(models.models))
}

/// POST /api/v1/arena/test
/// Create custom benchmark test
pub async fn create_test(
    Json(test): Json<BenchmarkTest>,
) -> Result<Json<TestCreated>, ServerError> {
    // TODO: Store in database
    Ok(Json(TestCreated {
        id: test.id.clone(),
        message: "Test created successfully".to_string(),
    }))
}

/// GET /api/v1/arena/leaderboard
/// Get model performance leaderboard
pub async fn get_leaderboard(
    Query(params): Query<LeaderboardQuery>,
) -> Result<Json<Leaderboard>, ServerError> {
    // TODO: Query database for aggregate stats
    Ok(Json(Leaderboard {
        models: vec![],
        category: params.category,
        updated_at: chrono::Utc::now().to_rfc3339(),
    }))
}

// Helper functions

async fn execute_benchmark(
    state: AppState,
    benchmark_id: String,
    models: Vec<String>,
    test_ids: Vec<String>,
    config: Option<BenchmarkConfig>,
) -> Result<(), String> {
    let broadcaster = &state.event_broadcaster;
    
    // Broadcast start event
    broadcaster.broadcast(ArenaBroadcastMessage::BenchmarkStarted {
        benchmark_id: benchmark_id.clone(),
        models: models.clone(),
        tests: test_ids.clone(),
    }).ok();
    
    // Load test definitions
    let tests = load_benchmark_tests(&test_ids)?;
    
    // Execute each test across all models
    for test in tests {
        // Run models concurrently or sequentially
        let model_results = if config.as_ref().map(|c| c.parallel).unwrap_or(true) {
            execute_parallel(&models, &test, &broadcaster, &benchmark_id).await?
        } else {
            execute_sequential(&models, &test, &broadcaster, &benchmark_id).await?
        };
        
        // Store results
        // TODO: Save to database
    }
    
    // Broadcast completion
    broadcaster.broadcast(ArenaBroadcastMessage::BenchmarkComplete {
        benchmark_id,
        results: Default::default(),
        leaderboard: vec![],
    }).ok();
    
    Ok(())
}

async fn execute_parallel(
    models: &[String],
    test: &BenchmarkTest,
    broadcaster: &EventBroadcaster,
    benchmark_id: &str,
) -> Result<Vec<TestResult>, String> {
    use futures::future::join_all;
    
    let futures = models.iter().map(|model| {
        run_model_on_test(
            model.clone(),
            test.clone(),
            broadcaster.clone(),
            benchmark_id.to_string(),
        )
    });
    
    let results = join_all(futures).await;
    results.into_iter().collect()
}

async fn execute_sequential(
    models: &[String],
    test: &BenchmarkTest,
    broadcaster: &EventBroadcaster,
    benchmark_id: &str,
) -> Result<Vec<TestResult>, String> {
    let mut results = Vec::new();
    
    for model in models {
        let result = run_model_on_test(
            model.clone(),
            test.clone(),
            broadcaster.clone(),
            benchmark_id.to_string(),
        ).await?;
        results.push(result);
    }
    
    Ok(results)
}

async fn run_model_on_test(
    model_id: String,
    test: BenchmarkTest,
    broadcaster: EventBroadcaster,
    benchmark_id: String,
) -> Result<TestResult, String> {
    let start_time = std::time::Instant::now();
    
    // Call Ollama generate API
    let client = reqwest::Client::new();
    let response = client
        .post("http://localhost:11434/api/generate")
        .json(&serde_json::json!({
            "model": model_id,
            "prompt": test.prompt,
            "stream": false,
        }))
        .send()
        .await
        .map_err(|e| format!("Ollama error: {}", e))?;
    
    let result: OllamaGenerateResponse = response.json().await
        .map_err(|e| format!("Parse error: {}", e))?;
    
    let elapsed = start_time.elapsed();
    
    // Evaluate response
    let metrics = evaluate_response(&test, &result, elapsed);
    
    // Broadcast result
    broadcaster.broadcast(ArenaBroadcastMessage::TestComplete {
        benchmark_id,
        model_id: model_id.clone(),
        test_id: test.id.clone(),
        output: result.response.clone(),
        metrics: metrics.clone(),
    }).ok();
    
    Ok(TestResult {
        test_id: test.id,
        model_id,
        output: result.response,
        metrics,
        timestamp: chrono::Utc::now().to_rfc3339(),
    })
}

fn evaluate_response(
    test: &BenchmarkTest,
    result: &OllamaGenerateResponse,
    elapsed: std::time::Duration,
) -> TestMetrics {
    // Calculate metrics
    let response_time_ms = elapsed.as_millis() as u64;
    let total_tokens = result.response.split_whitespace().count();  // Rough estimate
    let tokens_per_sec = if response_time_ms > 0 {
        (total_tokens as f32 * 1000.0) / response_time_ms as f32
    } else {
        0.0
    };
    
    // Keyword matching
    let keyword_matches = test.expected_criteria.keywords.iter()
        .filter(|kw| result.response.contains(kw.as_str()))
        .count();
    let correctness_score = if !test.expected_criteria.keywords.is_empty() {
        keyword_matches as f32 / test.expected_criteria.keywords.len() as f32
    } else {
        1.0
    };
    
    // Efficiency score (inverse of tokens used)
    let efficiency_score = if let Some(max_tokens) = test.expected_criteria.max_tokens {
        (1.0 - (total_tokens as f32 / max_tokens as f32)).max(0.0)
    } else {
        0.5
    };
    
    // Clarity score (placeholder - could use readability metrics)
    let clarity_score = 0.8;
    
    // Overall weighted score
    let overall_score = 
        correctness_score * test.rubric.correctness_weight +
        efficiency_score * test.rubric.efficiency_weight +
        clarity_score * test.rubric.clarity_weight;
    
    TestMetrics {
        response_time_ms,
        total_tokens,
        tokens_per_sec,
        correctness_score,
        efficiency_score,
        clarity_score,
        overall_score,
        keyword_matches,
        code_executed: None,  // TODO: Implement code execution sandbox
    }
}

fn load_benchmark_tests(test_ids: &[String]) -> Result<Vec<BenchmarkTest>, String> {
    // Hardcoded test suite for now
    let all_tests = vec![
        BenchmarkTest {
            id: "code_gen_sum".to_string(),
            name: "Implement Sum Function".to_string(),
            category: "code_gen".to_string(),
            prompt: "Write a Python function called 'calculate_sum' that takes a list of numbers and returns their sum. Include a docstring.".to_string(),
            expected_criteria: ExpectedCriteria {
                keywords: vec!["def".to_string(), "calculate_sum".to_string(), "return".to_string()],
                max_tokens: Some(50),
                max_time_ms: Some(5000),
                code_executes: Some(true),
            },
            rubric: EvaluationRubric {
                correctness_weight: 0.5,
                efficiency_weight: 0.3,
                clarity_weight: 0.2,
            },
        },
        BenchmarkTest {
            id: "research_compare".to_string(),
            name: "Compare Technologies".to_string(),
            category: "research".to_string(),
            prompt: "Compare REST and GraphQL APIs in 3-4 sentences. Focus on key differences.".to_string(),
            expected_criteria: ExpectedCriteria {
                keywords: vec!["REST".to_string(), "GraphQL".to_string(), "difference".to_string()],
                max_tokens: Some(100),
                max_time_ms: Some(8000),
                code_executes: None,
            },
            rubric: EvaluationRubric {
                correctness_weight: 0.6,
                efficiency_weight: 0.2,
                clarity_weight: 0.2,
            },
        },
    ];
    
    let tests: Vec<_> = all_tests.into_iter()
        .filter(|t| test_ids.contains(&t.id))
        .collect();
    
    if tests.is_empty() {
        Err("No valid tests found".to_string())
    } else {
        Ok(tests)
    }
}

// Supporting types
#[derive(Deserialize)]
struct OllamaModelsResponse {
    models: Vec<ModelInfo>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelInfo {
    pub name: String,
    pub size: u64,
    pub digest: String,
    pub modified_at: String,
}

#[derive(Deserialize)]
struct OllamaGenerateResponse {
    response: String,
    #[allow(dead_code)]
    model: String,
    #[allow(dead_code)]
    created_at: String,
}

#[derive(Serialize)]
struct TestCreated {
    id: String,
    message: String,
}

#[derive(Serialize)]
struct Leaderboard {
    models: Vec<ModelPerformance>,
    category: Option<String>,
    updated_at: String,
}
```

**Update WebSocket broadcaster:**

```rust
// Add to crates/agentx-server/src/websocket/broadcaster.rs

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type")]
pub enum ArenaBroadcastMessage {
    BenchmarkStarted {
        benchmark_id: String,
        models: Vec<String>,
        tests: Vec<String>,
    },
    ModelResponse {
        benchmark_id: String,
        model_id: String,
        test_id: String,
        chunk: String,
        elapsed_ms: u64,
    },
    TestComplete {
        benchmark_id: String,
        model_id: String,
        test_id: String,
        output: String,
        metrics: TestMetrics,
    },
    BenchmarkComplete {
        benchmark_id: String,
        results: HashMap<String, Vec<TestResult>>,
        leaderboard: Vec<ModelPerformance>,
    },
}
```

#### 2.3.2 Frontend - Arena UI Components

**Create:** `web-ui/src/pages/ModelArena.tsx`

```typescript
import { useState } from 'react';
import { ModelSelector } from '../components/Arena/ModelSelector';
import { TestSelector } from '../components/Arena/TestSelector';
import { BenchmarkControls } from '../components/Arena/BenchmarkControls';
import { ResultsTable } from '../components/Arena/ResultsTable';
import { ComparisonView } from '../components/Arena/ComparisonView';
import { MetricsCharts } from '../components/Arena/MetricsCharts';
import { useBenchmark } from '../hooks/useBenchmark';

export function ModelArena() {
  const [selectedModels, setSelectedModels] = useState<string[]>([]);
  const [selectedTests, setSelectedTests] = useState<string[]>([]);
  const { benchmark, isRunning, startBenchmark } = useBenchmark();
  
  const handleStart = () => {
    if (selectedModels.length >= 2 && selectedTests.length > 0) {
      startBenchmark(selectedModels, selectedTests);
    }
  };
  
  return (
    <div className="max-w-7xl mx-auto p-6 space-y-6">
      <div className="bg-white rounded-lg shadow p-6">
        <h1 className="text-3xl font-bold mb-2">Model Arena</h1>
        <p className="text-gray-600">
          Compare LLM performance side-by-side on identical benchmark tasks
        </p>
      </div>
      
      <div className="grid grid-cols-2 gap-6">
        <ModelSelector 
          selected={selectedModels} 
          onChange={setSelectedModels} 
        />
        <TestSelector 
          selected={selectedTests} 
          onChange={setSelectedTests} 
        />
      </div>
      
      <BenchmarkControls 
        onStart={handleStart} 
        isRunning={isRunning}
        disabled={selectedModels.length < 2 || selectedTests.length === 0}
      />
      
      {benchmark && (
        <>
          <ResultsTable benchmark={benchmark} />
          <ComparisonView benchmark={benchmark} />
          <MetricsCharts benchmark={benchmark} />
        </>
      )}
    </div>
  );
}
```

**Create:** `web-ui/src/components/Arena/ComparisonView.tsx`

```typescript
import { Benchmark } from '../../types/arena';
import { Prism as SyntaxHighlighter } from 'react-syntax-highlighter';
import { vscDarkPlus } from 'react-syntax-highlighter/dist/esm/styles/prism';

interface ComparisonViewProps {
  benchmark: Benchmark;
}

export function ComparisonView({ benchmark }: ComparisonViewProps) {
  if (!benchmark.results) return null;
  
  return (
    <div className="bg-white rounded-lg shadow p-6">
      <h2 className="text-2xl font-bold mb-4">Side-by-Side Comparison</h2>
      
      <div className="grid grid-cols-2 gap-4">
        {benchmark.models.slice(0, 2).map((modelId) => (
          <div key={modelId} className="border rounded-lg overflow-hidden">
            <div className="bg-gray-800 text-white px-4 py-2 font-semibold">
              {modelId}
            </div>
            
            <div className="p-4 space-y-4">
              {benchmark.results[modelId]?.map((result) => (
                <div key={result.test_id}>
                  <h3 className="text-sm font-medium text-gray-600 mb-2">
                    Test: {result.test_id}
                  </h3>
                  
                  <SyntaxHighlighter 
                    language="python" 
                    style={vscDarkPlus}
                    className="text-sm rounded"
                  >
                    {result.output}
                  </SyntaxHighlighter>
                  
                  <div className="mt-2 flex flex-wrap gap-2">
                    <MetricBadge 
                      icon="⚡" 
                      label={`${result.metrics.response_time_ms}ms`}
                      color="blue"
                    />
                    <MetricBadge 
                      icon="📏" 
                      label={`${result.metrics.total_tokens} tokens`}
                      color="green"
                    />
                    <MetricBadge 
                      icon="🎯" 
                      label={`${(result.metrics.correctness_score * 100).toFixed(0)}%`}
                      color="purple"
                    />
                  </div>
                </div>
              ))}
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}

function MetricBadge({ icon, label, color }: {
  icon: string;
  label: string;
  color: string;
}) {
  const colors = {
    blue: 'bg-blue-100 text-blue-800',
    green: 'bg-green-100 text-green-800',
    purple: 'bg-purple-100 text-purple-800',
  };
  
  return (
    <span className={`inline-flex items-center px-2 py-1 rounded text-xs font-medium ${colors[color]}`}>
      <span className="mr-1">{icon}</span>
      {label}
    </span>
  );
}
```

**Create:** `web-ui/src/hooks/useBenchmark.ts`

```typescript
import { useState, useCallback } from 'react';
import { useWebSocket } from './useWebSocket';
import { api } from '../services/api';
import { Benchmark, BenchmarkResponse } from '../types/arena';

export function useBenchmark() {
  const [benchmark, setBenchmark] = useState<Benchmark | null>(null);
  const [isRunning, setIsRunning] = useState(false);
  
  const { addEventListener, removeEventListener } = useWebSocket();
  
  const startBenchmark = useCallback(async (
    models: string[],
    testIds: string[]
  ) => {
    setIsRunning(true);
    
    try {
      const response = await api.post<BenchmarkResponse>('/arena/benchmark', {
        models,
        test_ids: testIds,
        config: { parallel: true },
      });
      
      setBenchmark({
        id: response.data.benchmark_id,
        models,
        tests: testIds,
        status: 'running',
        results: {},
        leaderboard: [],
      });
      
      // Listen for WebSocket events
      const handleTestComplete = (event: any) => {
        if (event.data.benchmark_id === response.data.benchmark_id) {
          setBenchmark(prev => {
            if (!prev) return null;
            const results = { ...prev.results };
            if (!results[event.data.model_id]) {
              results[event.data.model_id] = [];
            }
            results[event.data.model_id].push({
              test_id: event.data.test_id,
              model_id: event.data.model_id,
              output: event.data.output,
              metrics: event.data.metrics,
              timestamp: new Date().toISOString(),
            });
            return { ...prev, results };
          });
        }
      };
      
      const handleComplete = (event: any) => {
        if (event.data.benchmark_id === response.data.benchmark_id) {
          setBenchmark(prev => prev ? {
            ...prev,
            status: 'completed',
            leaderboard: event.data.leaderboard,
          } : null);
          setIsRunning(false);
        }
      };
      
      addEventListener('test_complete', handleTestComplete);
      addEventListener('benchmark_complete', handleComplete);
      
      return () => {
        removeEventListener('test_complete', handleTestComplete);
        removeEventListener('benchmark_complete', handleComplete);
      };
    } catch (error) {
      console.error('Failed to start benchmark:', error);
      setIsRunning(false);
    }
  }, [addEventListener, removeEventListener]);
  
  return { benchmark, isRunning, startBenchmark };
}
```

#### 2.3.3 Pre-built Benchmark Test Suite

**Benchmark Categories:**

1. **Code Generation** (10 tests)
   - Implement function from docstring
   - Fix buggy code
   - Refactor legacy code
   - Generate unit tests
   - Create CLI script
   - Implement algorithm (sorting, searching)
   - Parse and validate input
   - Generate regex pattern
   - Create data structure
   - Write async function

2. **Research & Analysis** (8 tests)
   - Summarize documentation
   - Compare technologies
   - Extract key information
   - Generate outline
   - Explain concept
   - List pros/cons
   - Suggest alternatives
   - Create glossary

3. **Debugging** (6 tests)
   - Analyze error message
   - Find code issue
   - Suggest fix
   - Explain bug cause
   - Review code quality
   - Security vulnerability scan

4. **API Design** (6 tests)
   - Design REST endpoint
   - Create OpenAPI spec
   - Design GraphQL schema
   - Write API documentation
   - Design error responses
   - Version API schema

**Acceptance Criteria:**

✅ **AC-2.3.1:** Arena backend operational
- [ ] `/api/v1/arena/models` returns Ollama models
- [ ] `/api/v1/arena/benchmark` starts execution
- [ ] Multiple models execute concurrently
- [ ] Results stored and retrievable
- [ ] WebSocket events broadcast correctly

✅ **AC-2.3.2:** Arena UI functional
- [ ] Model selection works (minimum 2, maximum 8)
- [ ] Test selection works (checkbox interface)
- [ ] Start button triggers benchmark
- [ ] Live results update via WebSocket
- [ ] Side-by-side comparison displays

✅ **AC-2.3.3:** Evaluation metrics accurate
- [ ] Response time measured correctly (±50ms)
- [ ] Token counting functional
- [ ] Keyword matching works
- [ ] Overall score calculation correct
- [ ] Leaderboard updates properly

✅ **AC-2.3.4:** Custom tests can be created
- [ ] User can define prompt
- [ ] Can specify expected criteria
- [ ] Can set evaluation weights
- [ ] Custom tests saved to library
- [ ] Custom tests selectable in benchmark

✅ **AC-2.3.5:** Code execution validation works
- [ ] Generated Python code runs in sandbox
- [ ] Execution success/failure recorded
- [ ] Runtime errors captured
- [ ] Test results included in metrics

---

## Phase 3: Integration & Agent-Workspace Binding

**Duration:** Week 5-6

### 3.1 Agent Lifecycle with Kasm Workspaces

**Tasks:**
- [ ] Integrate AgentX core with web server
- [ ] Create agent-to-workspace mapping
- [ ] Implement workspace provisioning on agent start
- [ ] Add workspace destruction on agent complete
- [ ] Implement agent action → workspace command bridge

**Technical Specifications:**

```rust
// crates/agentx-server/src/agent/executor.rs
use anyhow::Result;
use tokio::sync::{mpsc, broadcast};
use uuid::Uuid;
use agentx_core::SoftwareCompany;
use crate::kasm::client::KasmClient;
use crate::state::AppState;

pub struct AgentExecutor {
    kasm_client: KasmClient,
    event_tx: broadcast::Sender<AgentEvent>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AgentEvent {
    pub event_type: String,
    pub agent_type: String,
    pub message: String,
    pub workspace_id: Option<String>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl AgentExecutor {
    pub fn new(kasm_client: KasmClient, event_tx: broadcast::Sender<AgentEvent>) -> Self {
        Self {
            kasm_client,
            event_tx,
        }
    }

    pub async fn execute_task(&self, task_id: String, idea: String, agent_type: String, n_round: i32) -> Result<()> {
        // Step 1: Provision Kasm workspace for research agent
        let researcher_workspace = self.provision_workspace("agentx-researcher").await?;
        
        self.emit_event(AgentEvent {
            event_type: "workspace_ready".to_string(),
            agent_type: "Researcher".to_string(),
            message: "Research workspace provisioned".to_string(),
            workspace_id: Some(researcher_workspace.workspace_id.clone()),
            timestamp: chrono::Utc::now(),
        });

        // Step 2: Provision developer workspace
        let dev_workspace = self.provision_workspace("agentx-developer").await?;
        
        self.emit_event(AgentEvent {
            event_type: "workspace_ready".to_string(),
            agent_type: "Developer".to_string(),
            message: "Development workspace provisioned".to_string(),
            workspace_id: Some(dev_workspace.workspace_id.clone()),
            timestamp: chrono::Utc::now(),
        });

        // Step 3: Execute AgentX core logic
        let company = SoftwareCompany::new(idea.clone(), 3.0, n_round, false, false)?;
        
        // Wrap company execution with event emission
        tokio::spawn(async move {
            // Run company.run() here
            // Emit events for each agent action
        });

        Ok(())
    }

    async fn provision_workspace(&self, image_id: &str) -> Result<WorkspaceResponse> {
        let workspace = self.kasm_client.create_workspace(image_id).await?;
        Ok(workspace)
    }

    fn emit_event(&self, event: AgentEvent) {
        let _ = self.event_tx.send(event);
    }
}
```

**Agent Action Bridge:**

```rust
// crates/agentx-server/src/agent/workspace_bridge.rs
use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Commands that can be sent to agent workspaces
#[derive(Debug, Serialize, Deserialize)]
pub enum WorkspaceCommand {
    OpenBrowser { url: String },
    ExecuteCode { code: String, language: String },
    CreateFile { path: String, content: String },
    RunTerminalCommand { command: String },
}

pub struct WorkspaceBridge {
    workspace_id: String,
    // Connection to workspace (could be WebSocket, API, etc.)
}

impl WorkspaceBridge {
    pub async fn execute_command(&self, command: WorkspaceCommand) -> Result<String> {
        match command {
            WorkspaceCommand::OpenBrowser { url } => {
                // Send command to Kasm workspace to open Firefox with URL
                self.send_browser_command(&url).await
            }
            WorkspaceCommand::ExecuteCode { code, language } => {
                // Execute code in workspace terminal/IDE
                self.execute_in_workspace(&code, &language).await
            }
            WorkspaceCommand::CreateFile { path, content } => {
                // Create file in workspace filesystem
                self.create_workspace_file(&path, &content).await
            }
            WorkspaceCommand::RunTerminalCommand { command } => {
                // Run shell command in workspace
                self.run_terminal(&command).await
            }
        }
    }

    async fn send_browser_command(&self, url: &str) -> Result<String> {
        // Implementation: Use Selenium WebDriver or browser automation
        // to control Firefox in Kasm workspace
        Ok(format!("Opened browser to {}", url))
    }

    async fn execute_in_workspace(&self, code: &str, language: &str) -> Result<String> {
        // Implementation: Execute code via terminal in workspace
        Ok(format!("Executed {} code", language))
    }

    async fn create_workspace_file(&self, path: &str, content: &str) -> Result<String> {
        // Implementation: Write file to workspace persistent volume
        Ok(format!("Created file: {}", path))
    }

    async fn run_terminal(&self, command: &str) -> Result<String> {
        // Implementation: Execute terminal command in workspace
        Ok(format!("Ran command: {}", command))
    }
}
```

**Modify AgentX Core to Emit Events:**

```rust
// crates/agent_provider/src/openai.rs
// Add event emission to existing streaming code

impl OpenAIGPTAPI {
    pub async fn aask_with_events(
        &mut self,
        msg: String,
        system_message: String,
        event_sender: Option<tokio::sync::mpsc::Sender<StreamEvent>>,
    ) -> Result<String> {
        // ... existing setup code ...

        while let Some(result) = stream.next().await {
            match result {
                Ok(response) => {
                    for chat_choice in response.choices {
                        if let Some(delta) = chat_choice.delta.content {
                            content.push_str(&delta);
                            
                            // NEW: Emit stream chunk event
                            if let Some(sender) = &event_sender {
                                let _ = sender.send(StreamEvent::Chunk {
                                    content: delta.clone(),
                                    chunk_id: chunk_count,
                                }).await;
                            }
                            
                            chunk_count += 1;
                        }
                    }
                }
                Err(e) => {
                    error!("Stream error: {:?}", e);
                }
            }
        }

        Ok(content)
    }
}
```

**Acceptance Criteria:**

✅ **AC-3.1.1:** Workspaces provision on task start
- [ ] Starting a task creates 2 Kasm workspaces (researcher + developer)
- [ ] Workspace URLs returned within 30 seconds
- [ ] Workspaces accessible in browser
- [ ] WebSocket event "workspace_ready" received by frontend

✅ **AC-3.1.2:** Agent actions visible in workspace
- [ ] When Research Agent browses, Firefox opens URL in workspace
- [ ] When Engineer writes code, file appears in VS Code in workspace
- [ ] When tests run, terminal output visible in workspace
- [ ] Actions happen within 5 seconds of agent event

✅ **AC-3.1.3:** Workspace cleanup works
- [ ] Task completion destroys associated workspaces
- [ ] `podman ps` shows workspaces terminated
- [ ] No orphaned containers after 10 task runs
- [ ] Workspace data saved before destruction (if needed)

### 3.2 Real-time Event Streaming

**Tasks:**
- [ ] Connect AgentX core events to WebSocket broadcaster
- [ ] Implement log streaming from agents
- [ ] Add file creation event emission
- [ ] Implement progress tracking
- [ ] Add error event handling

**Technical Specifications:**

```rust
// crates/agentx-server/src/websocket/broadcaster.rs
use tokio::sync::broadcast;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum BroadcastMessage {
    AgentEvent {
        timestamp: String,
        agent: String,
        status: String,
        action: Option<String>,
        message: String,
    },
    StreamChunk {
        timestamp: String,
        agent: String,
        content: String,
        chunk_id: usize,
    },
    FileCreated {
        timestamp: String,
        path: String,
        name: String,
        size: usize,
        download_url: String,
    },
    WorkspaceReady {
        timestamp: String,
        workspace_id: String,
        agent_type: String,
        url: String,
    },
    TaskComplete {
        timestamp: String,
        task_id: String,
        duration_seconds: u64,
    },
    Error {
        timestamp: String,
        message: String,
        details: Option<String>,
    },
}

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
```

```rust
// crates/agentx-server/src/websocket/handler.rs
use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use futures::{sink::SinkExt, stream::StreamExt};
use crate::state::AppState;
use crate::websocket::broadcaster::BroadcastMessage;

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.broadcaster.subscribe();

    // Spawn task to receive broadcasts and send to client
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            let json = serde_json::to_string(&msg).unwrap();
            if sender.send(Message::Text(json)).await.is_err() {
                break;
            }
        }
    });

    // Spawn task to receive messages from client
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(Message::Text(text))) = receiver.next().await {
            // Handle client messages (e.g., commands, pings)
            tracing::debug!("Received from client: {}", text);
        }
    });

    // Wait for either task to finish
    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };
}
```

**Acceptance Criteria:**

✅ **AC-3.2.1:** Real-time log streaming works
- [ ] Frontend receives agent log messages within 100ms
- [ ] Messages appear in correct chronological order
- [ ] No message loss during normal operation
- [ ] Can handle 100+ messages per second

✅ **AC-3.2.2:** LLM streaming displays in UI
- [ ] LLM response chunks appear in real-time
- [ ] Chunks concatenate into full response
- [ ] Streaming indicator shows when LLM is generating
- [ ] Final response formatted correctly

✅ **AC-3.2.3:** File creation events work
- [ ] File appears in explorer immediately after creation
- [ ] Download link functional
- [ ] File metadata correct (size, type, timestamp)
- [ ] Notification toast displays

---

## Phase 4: Testing & Polish

**Duration:** Week 7

### 4.1 End-to-End Testing

**Acceptance Criteria:**

✅ **AC-4.1.1:** Complete workflow test
- [ ] User enters task: "Create a simple calculator"
- [ ] Task creates and shows in dashboard
- [ ] 2 Kasm workspaces provision automatically
- [ ] Product Manager agent appears, status "thinking"
- [ ] PRD generation streams to logs in real-time
- [ ] PRD file appears in file explorer
- [ ] Architect agent activates, workspace switches to developer view
- [ ] API design visible in VS Code in workspace
- [ ] Engineer agent writes code, visible in workspace editor
- [ ] Code execution runs in workspace terminal
- [ ] Test output visible in workspace
- [ ] Task completes, all files downloadable
- [ ] Workspaces destroyed after completion
- [ ] Total time < 5 minutes for simple task

✅ **AC-4.1.2:** Multi-agent research test
- [ ] Task: "Research latest AI trends and create report"
- [ ] Research workspace opens with Firefox
- [ ] Browser navigates to search results automatically
- [ ] Web scraping visible in browser
- [ ] Scraped content appears in workspace files
- [ ] Summary generation streams to logs
- [ ] Final report available for download
- [ ] All research sources cited correctly

✅ **AC-4.1.3:** Error handling test
- [ ] Invalid task input shows error message
- [ ] Kasm workspace failure shows retry option
- [ ] LLM timeout handled gracefully
- [ ] Network disconnect shows reconnection indicator
- [ ] Workspace crash detected and reported
- [ ] User can restart failed task

### 4.2 Performance & Optimization

**Acceptance Criteria:**

✅ **AC-4.2.1:** Resource usage acceptable
- [ ] AgentX server RAM usage < 500MB idle
- [ ] Each Kasm workspace RAM usage < 2GB
- [ ] Total system RAM usage < 8GB with 4 agents
- [ ] CPU usage < 50% during normal operation
- [ ] WebSocket latency < 50ms

✅ **AC-4.2.2:** UI responsiveness
- [ ] Canvas pan/zoom smooth (60 FPS)
- [ ] Log scrolling handles 10,000+ lines
- [ ] File explorer loads instantly
- [ ] Kasm iframe loads within 10 seconds
- [ ] No UI freezes during heavy operations

### 4.3 Documentation

**Acceptance Criteria:**

✅ **AC-4.3.1:** User documentation complete
- [ ] Installation guide with step-by-step instructions
- [ ] Quick start tutorial with screenshots
- [ ] Troubleshooting guide for common issues
- [ ] API documentation for advanced users
- [ ] Video walkthrough of key features

✅ **AC-4.3.2:** Developer documentation complete
- [ ] Architecture diagram with explanations
- [ ] API endpoint reference
- [ ] WebSocket protocol specification
- [ ] Kasm integration guide
- [ ] Contributing guidelines

---

## Phase 5: Deployment & Production Readiness

**Duration:** Week 8

### 5.1 Containerization

**Tasks:**
- [ ] Create Dockerfile for AgentX server
- [ ] Create podman-compose.yml for full stack
- [ ] Test build process
- [ ] Optimize image sizes
- [ ] Document deployment

**Technical Specifications:**

```dockerfile
# Dockerfile
FROM rust:1.75-slim as builder

WORKDIR /app

# Copy manifests
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates

# Build release binary
RUN cargo build --release --bin agentx-server

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/agentx-server /usr/local/bin/

# Copy web UI build
COPY web-ui/dist /app/web-ui/dist

WORKDIR /app

EXPOSE 8080

CMD ["agentx-server"]
```

```yaml
# podman-compose.yml
version: '3.8'

services:
  # Ollama LLM Service
  ollama:
    image: ollama/ollama:latest
    container_name: agentx-ollama
    ports:
      - "11434:11434"
    volumes:
      - ollama-data:/root/.ollama
    restart: unless-stopped

  # Kasm Workspaces (full platform)
  kasm-db:
    image: postgres:12-alpine
    container_name: agentx-kasm-db
    environment:
      POSTGRES_DB: kasm
      POSTGRES_USER: kasmapp
      POSTGRES_PASSWORD: ${KASM_DB_PASSWORD:-kasmpassword}
    volumes:
      - kasm-db-data:/var/lib/postgresql/data
    restart: unless-stopped

  kasm-redis:
    image: redis:6-alpine
    container_name: agentx-kasm-redis
    restart: unless-stopped

  kasm-manager:
    image: kasmweb/manager:1.14.0
    container_name: agentx-kasm-manager
    depends_on:
      - kasm-db
      - kasm-redis
    environment:
      DATABASE_URL: postgresql://kasmapp:${KASM_DB_PASSWORD:-kasmpassword}@kasm-db/kasm
      REDIS_URL: redis://kasm-redis:6379
      SERVER_HOSTNAME: ${KASM_HOSTNAME:-localhost}
    ports:
      - "443:443"
    volumes:
      - kasm-data:/opt/kasm/current
    restart: unless-stopped

  kasm-agent:
    image: kasmweb/agent:1.14.0
    container_name: agentx-kasm-agent
    depends_on:
      - kasm-manager
    environment:
      MANAGER_HOSTNAME: kasm-manager
      SERVER_ID: agent1
    volumes:
      - /var/run/podman/podman.sock:/var/run/docker.sock
      - kasm-profiles:/mnt/kasm_profiles
    privileged: true
    restart: unless-stopped

  # AgentX Web Server
  agentx-server:
    build: .
    container_name: agentx-server
    depends_on:
      - ollama
      - kasm-manager
    environment:
      OPENAI_API_BASE: http://ollama:11434/v1
      OPENAI_API_MODEL: qwen2.5-coder:3b
      KASM_API_URL: https://kasm-manager:443
      KASM_API_KEY: ${KASM_API_KEY}
      KASM_API_SECRET: ${KASM_API_SECRET}
      RUST_LOG: info
    ports:
      - "8080:8080"
    volumes:
      - ./workspace:/app/workspace
    restart: unless-stopped

volumes:
  ollama-data:
  kasm-db-data:
  kasm-data:
  kasm-profiles:
```

**.env.example:**
```bash
# Kasm Workspaces
KASM_DB_PASSWORD=your_secure_password_here
KASM_HOSTNAME=localhost
KASM_API_KEY=your_kasm_api_key
KASM_API_SECRET=your_kasm_api_secret

# Ollama
OPENAI_API_BASE=http://localhost:11434/v1
OPENAI_API_MODEL=qwen2.5-coder:3b
```

**Acceptance Criteria:**

✅ **AC-5.1.1:** Podman build successful
- [ ] `podman build -t agentx-server .` completes without errors
- [ ] Image size < 500MB
- [ ] Build time < 5 minutes
- [ ] Multi-stage build works correctly

✅ **AC-5.1.2:** Full stack deployment works
- [ ] `podman-compose up -d` starts all services
- [ ] All health checks pass
- [ ] Web UI accessible at http://localhost:8080
- [ ] Can create and run a test task end-to-end

✅ **AC-5.1.3:** Persistence verified
- [ ] Restart containers, workspace data preserved
- [ ] Ollama models persist across restarts
- [ ] Kasm user profiles persist
- [ ] Generated files available after restart

### 5.2 Security Hardening

**Acceptance Criteria:**

✅ **AC-5.2.1:** Network security
- [ ] Services communicate over internal network only
- [ ] HTTPS enabled for Kasm (self-signed cert OK for dev)
- [ ] API authentication implemented (token-based)
- [ ] CORS configured correctly (whitelist origins)

✅ **AC-5.2.2:** Container security
- [ ] Containers run as non-root where possible
- [ ] Resource limits set (CPU, RAM)
- [ ] Read-only filesystems where appropriate
- [ ] Secrets managed via environment variables

### 5.3 Production Deployment Guide

**Acceptance Criteria:**

✅ **AC-5.3.1:** Deployment documentation
- [ ] Step-by-step installation guide
- [ ] System requirements clearly stated
- [ ] Troubleshooting section complete
- [ ] Upgrade/migration guide provided

✅ **AC-5.3.2:** Monitoring setup
- [ ] Health check endpoints documented
- [ ] Log aggregation configured
- [ ] Resource monitoring enabled
- [ ] Alerting guidelines provided

---

## Success Metrics

### Functional Metrics
- ✅ 100% of agent actions visible in workspaces
- ✅ < 5 second latency from agent action to workspace update
- ✅ 95%+ uptime for 24-hour test period
- ✅ Zero data loss for generated files
- ✅ Complete task success rate > 90%

### Performance Metrics
- ✅ WebSocket latency < 100ms p95
- ✅ UI frame rate > 30 FPS during operation
- ✅ Task completion time < 5 minutes for simple tasks
- ✅ System RAM usage < 8GB with 4 concurrent agents
- ✅ Workspace provision time < 30 seconds

### User Experience Metrics
- ✅ Time to first visible output < 10 seconds
- ✅ Log messages readable and informative
- ✅ File download success rate 100%
- ✅ Zero UI crashes during testing
- ✅ Mobile/tablet view functional (responsive design)

---

## Risk Mitigation

### Technical Risks

**Risk 1: Kasm Workspace Performance**
- *Mitigation:* Use lightweight desktop environments (XFCE instead of KDE)
- *Fallback:* Implement workspace pooling for faster provisioning

**Risk 2: WebSocket Connection Stability**
- *Mitigation:* Implement automatic reconnection with exponential backoff
- *Fallback:* Polling API as backup for critical updates

**Risk 3: Ollama + Kasm Resource Conflict**
- *Mitigation:* Set explicit resource limits in podman-compose
- *Fallback:* Run Ollama on separate machine if needed

**Risk 4: Browser Automation Complexity**
- *Mitigation:* Use proven Selenium WebDriver library
- *Fallback:* Manual browsing with visual monitoring only

### Timeline Risks

**Risk 1: Frontend Development Delays**
- *Mitigation:* Use pre-built component libraries (Headless UI, Tailwind UI)
- *Buffer:* Allocate 1 extra week for frontend polish

**Risk 2: Kasm Integration Complexity**
- *Mitigation:* Start with simple workspace provisioning, add features incrementally
- *Buffer:* Kasm has excellent documentation and community support

---

## Deliverables Checklist

### Code Deliverables
- [ ] `crates/agentx-server` - Web server crate
- [ ] `web-ui/` - React frontend application
- [ ] `kasm-workspaces/` - Custom workspace Dockerfiles
- [ ] `podman-compose.yml` - Full stack deployment
- [ ] `Dockerfile` - AgentX server container

### Documentation Deliverables
- [ ] `docs/INSTALLATION.md` - Installation guide
- [ ] `docs/USER_GUIDE.md` - User manual
- [ ] `docs/API_REFERENCE.md` - API documentation
- [ ] `docs/DEVELOPER_GUIDE.md` - Developer docs
- [ ] `docs/TROUBLESHOOTING.md` - Common issues

### Testing Deliverables
- [ ] Unit tests for server (>80% coverage)
- [ ] Integration tests for API endpoints
- [ ] End-to-end test scripts
- [ ] Performance benchmarks
- [ ] Security audit checklist

---

## Post-Implementation Roadmap

### Phase 6 (Future Enhancements)
- [ ] Multi-user support with authentication
- [ ] Workspace templates for different agent types
- [ ] Session recording and playback
- [ ] Cloud deployment (AWS/GCP/Azure)
- [ ] Mobile app (React Native)
- [ ] Agent marketplace (community agents)
- [ ] Collaborative features (multiple users on one task)

---

## Conclusion

This implementation plan provides a comprehensive path to transforming AgentX from a CLI tool into a production-ready, browser-based multi-agent platform with live visual workspaces powered by Kasm.

**Key Advantages:**
- ✅ Live visual feedback of agent actions
- ✅ Production-grade streaming infrastructure
- ✅ Scalable architecture
- ✅ Professional user experience
- ✅ Podman-native deployment

**Timeline:** 8 weeks from start to production deployment
**Effort:** ~160-200 hours of development
**Result:** Fully functional, browser-accessible AgentX with visual agent workspaces
