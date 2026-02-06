#!/bin/bash
# Build script for agentx-researcher workspace image

set -e

IMAGE_NAME="agentx-researcher"
IMAGE_TAG="latest"

echo "Building ${IMAGE_NAME}:${IMAGE_TAG}..."

# Build with Podman
podman build -t ${IMAGE_NAME}:${IMAGE_TAG} .

echo "Build complete!"
echo "Image: ${IMAGE_NAME}:${IMAGE_TAG}"
echo ""
echo "To test locally:"
echo "  podman run -p 6901:6901 ${IMAGE_NAME}:${IMAGE_TAG}"
echo ""
echo "Access via: https://localhost:6901"
