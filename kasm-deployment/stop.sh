#!/bin/bash
# Stop and remove Kasm Workspaces

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "🛑 Stopping Kasm Workspaces..."

podman-compose -f podman-compose.yml down

echo "✅ Kasm Workspaces stopped."
echo ""
echo "To remove all data (including volumes):"
echo "  podman-compose -f podman-compose.yml down -v"
