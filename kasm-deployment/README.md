# Kasm Workspaces Deployment Guide

## Overview

This directory contains the configuration and custom workspace images for deploying Kasm Workspaces with AgentX.

## Prerequisites

- Linux system with Podman installed
- At least 4GB RAM available
- 20GB disk space
- Ports 443 and 6901 available

## Directory Structure

```
kasm-deployment/
├── docker-compose.yml          # Docker Compose config (if using Docker)
├── podman-compose.yml          # Podman Compose config (recommended)
├── .env.example               # Environment variables template
└── README.md                  # This file

kasm-workspaces/
├── agentx-researcher/         # Research workspace image
│   ├── Dockerfile
│   ├── build.sh
│   └── scripts/
└── agentx-developer/          # Development workspace image
    ├── Dockerfile
    ├── build.sh
    └── scripts/
```

## Quick Start

### 1. Install Podman (if not already installed)

```bash
# Ubuntu/Debian
sudo apt-get update
sudo apt-get install -y podman podman-compose

# Fedora/RHEL
sudo dnf install -y podman podman-compose

# Arch Linux
sudo pacman -S podman podman-compose
```

### 2. Configure Environment

```bash
cd kasm-deployment
cp .env.example .env

# Edit .env with your preferred settings
nano .env
```

### 3. Deploy Kasm Workspaces

```bash
# Start Kasm services with Podman
podman-compose -f podman-compose.yml up -d

# Check status
podman ps

# View logs
podman logs kasm_manager
```

### 4. Initial Setup

1. Access Kasm web interface: https://localhost:443
2. Login with default credentials:
   - Username: `admin@kasm.local`
   - Password: `admin` (change immediately!)
3. Complete the initial setup wizard
4. Generate API credentials:
   - Navigate to Admin → API Tokens
   - Create new token
   - Save the API Key and Secret

### 5. Build Custom Workspace Images

```bash
# Build researcher workspace
cd ../kasm-workspaces/agentx-researcher
chmod +x build.sh scripts/research-helper.sh
./build.sh

# Build developer workspace
cd ../agentx-developer
chmod +x build.sh scripts/dev-helper.sh
./build.sh
```

### 6. Register Workspace Images in Kasm

1. In Kasm web interface, go to: **Admin → Workspaces**
2. Click **Add Workspace**
3. For **agentx-researcher**:
   - Name: `AgentX Researcher`
   - Docker Image: `agentx-researcher:latest`
   - Docker Registry: `Local Podman`
   - Thumbnail: Upload or leave default
4. Repeat for **agentx-developer**

### 7. Update AgentX Configuration

```bash
cd ../../
nano .env  # or edit your main .env file

# Add the following:
KASM_API_URL=https://localhost:443
KASM_API_KEY=<your-api-key>
KASM_API_SECRET=<your-api-secret>
```

## Testing

### Test Workspace Provisioning

```bash
# Using Kasm API
curl -k -X POST https://localhost:443/api/public/request_workspace \
  -H "Authorization: Bearer YOUR_API_TOKEN" \
  -H "Content-Type: application/json" \
  -d '{
    "image_id": "agentx-researcher",
    "enable_sharing": false
  }'
```

### Test AgentX Server Integration

```bash
cd crates/agentx-server
cargo run

# In another terminal:
curl http://localhost:8080/api/v1/health
```

## Acceptance Criteria Checklist

### AC-1.1.1: Kasm Workspaces Deployed with Podman
- [ ] All containers running: `podman ps` shows kasm-manager, kasm-agent, kasm-db, kasm-redis
- [ ] Web interface accessible at https://localhost:443
- [ ] Admin login successful
- [ ] Zero critical errors in logs: `podman logs kasm-manager`

### AC-1.1.2: Custom Workspace Images Built
- [ ] `agentx-researcher` image built successfully
- [ ] `agentx-developer` image built successfully
- [ ] Both images appear in Kasm workspace catalog
- [ ] Test launch of each image successful

### AC-1.1.3: Kasm API Credentials Configured
- [ ] API token generated in Kasm admin panel
- [ ] Token stored securely in AgentX configuration
- [ ] Test API call successful: `GET /api/workspaces` returns 200

## Troubleshooting

### Podman Socket Issues

If you encounter socket permission issues:

```bash
# Enable Podman socket
systemctl --user enable --now podman.socket

# Set environment variable
export DOCKER_HOST=unix:///run/user/$UID/podman/podman.sock
```

### Network Connectivity

If containers can't communicate:

```bash
# Check network
podman network ls
podman network inspect kasm-network

# Recreate network if needed
podman-compose down
podman network rm kasm-network
podman-compose up -d
```

### Image Build Failures

If workspace images fail to build:

```bash
# Check Podman storage
podman system df

# Prune if needed
podman system prune -a

# Rebuild with verbose output
podman build --no-cache -t agentx-researcher:latest .
```

## Maintenance

### Updating Kasm

```bash
# Pull latest images
podman pull kasmweb/manager:1.14.0
podman pull kasmweb/agent:1.14.0
podman pull postgres:12-alpine
podman pull redis:6-alpine

# Restart services
podman-compose down
podman-compose up -d
```

### Backup

```bash
# Backup database
podman exec kasm_db pg_dump -U kasmapp kasm > kasm-backup-$(date +%Y%m%d).sql

# Backup volumes
podman volume export kasm-db-data > kasm-db-data.tar
```

## Security Considerations

1. **Change Default Passwords**: Update admin password immediately
2. **Use HTTPS**: Kasm uses self-signed certificates by default - consider proper SSL certificates
3. **API Credentials**: Store API keys securely, never commit to version control
4. **Network Isolation**: Consider using firewall rules to restrict access
5. **Regular Updates**: Keep Kasm and workspace images up to date

## Next Steps

After successful deployment:
1. ✅ Test workspace creation via API
2. ✅ Integrate with AgentX server
3. ✅ Test agent workflow with live workspaces
4. Move to Phase 2: Frontend Development

## Resources

- [Kasm Workspaces Documentation](https://kasmweb.com/docs)
- [Podman Documentation](https://docs.podman.io/)
- [AgentX Implementation Plan](../docs/KASM_IMPLEMENTATION_PLAN.md)
