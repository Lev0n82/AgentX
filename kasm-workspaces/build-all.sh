#!/bin/bash
# Build all AgentX workspace images

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo "======================================"
echo "  Building AgentX Workspace Images"
echo "======================================"
echo ""

# Build researcher workspace
echo "📦 Building agentx-researcher..."
cd "$SCRIPT_DIR/agentx-researcher"
chmod +x build.sh scripts/research-helper.sh
./build.sh
echo ""

# Build developer workspace
echo "📦 Building agentx-developer..."
cd "$SCRIPT_DIR/agentx-developer"
chmod +x build.sh scripts/dev-helper.sh
./build.sh
echo ""

echo "======================================"
echo "  Build Complete!"
echo "======================================"
echo ""
echo "✅ Built Images:"
podman images | grep agentx
echo ""
echo "📝 Next Steps:"
echo "   1. Go to Kasm admin panel: https://localhost:443"
echo "   2. Navigate to Admin → Workspaces"
echo "   3. Add both workspace images"
echo "   4. Test launching each workspace"
echo ""
