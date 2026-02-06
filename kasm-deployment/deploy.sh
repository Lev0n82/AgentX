#!/bin/bash
# Deploy Kasm Workspaces with Podman
# This script automates the deployment process

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

echo "======================================"
echo "  AgentX Kasm Workspaces Deployment"
echo "======================================"
echo ""

# Check for Podman
if ! command -v podman &> /dev/null; then
    echo "❌ Podman is not installed!"
    echo "Please install Podman first:"
    echo "  Ubuntu/Debian: sudo apt-get install podman podman-compose"
    echo "  Fedora/RHEL: sudo dnf install podman podman-compose"
    exit 1
fi

# Check for podman-compose
if ! command -v podman-compose &> /dev/null; then
    echo "❌ podman-compose is not installed!"
    echo "Please install podman-compose first."
    exit 1
fi

echo "✅ Podman found: $(podman --version)"
echo "✅ podman-compose found: $(podman-compose --version)"
echo ""

# Check for .env file
if [ ! -f .env ]; then
    echo "⚠️  No .env file found. Creating from template..."
    cp .env.example .env
    echo "📝 Please edit .env with your configuration:"
    echo "   nano .env"
    echo ""
    read -p "Press Enter to continue after editing .env..."
fi

# Load environment
source .env

echo "🚀 Starting Kasm Workspaces deployment..."
echo ""

# Start services
podman-compose -f podman-compose.yml up -d

echo ""
echo "⏳ Waiting for services to start..."
sleep 10

# Check status
echo ""
echo "📊 Container Status:"
podman ps --filter "name=kasm" --format "table {{.Names}}\t{{.Status}}\t{{.Ports}}"

echo ""
echo "======================================"
echo "  Deployment Summary"
echo "======================================"
echo ""
echo "✅ Kasm Workspaces deployed!"
echo ""
echo "🌐 Access Kasm Web Interface:"
echo "   URL: https://localhost:443"
echo "   Default Username: admin@kasm.local"
echo "   Default Password: admin"
echo ""
echo "⚠️  IMPORTANT:"
echo "   1. Change the default admin password immediately!"
echo "   2. Generate API credentials in Admin → API Tokens"
echo "   3. Update .env with your API credentials"
echo ""
echo "📖 Next Steps:"
echo "   1. Build workspace images: cd ../kasm-workspaces && ./build-all.sh"
echo "   2. Register images in Kasm admin panel"
echo "   3. Test workspace creation"
echo ""
echo "📝 View logs:"
echo "   podman logs kasm_manager"
echo "   podman logs kasm_agent"
echo ""
