#!/usr/bin/env bash
# Build the server Docker image locally and push to GitHub Container Registry.
#
# Usage:
#   ./infra/scripts/build-and-push.sh          # build + push :latest
#   ./infra/scripts/build-and-push.sh v0.1.0   # build + push :v0.1.0 + :latest
#
# Prerequisites:
#   podman login ghcr.io -u Detair
#   (or: echo $GITHUB_TOKEN | podman login ghcr.io -u Detair --password-stdin)

set -euo pipefail

REPO="ghcr.io/detair/kaiku/server"
TAG="${1:-latest}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(cd "$SCRIPT_DIR/../.." && pwd)"

echo "[+] Building server image..."
podman build \
    -t "$REPO:$TAG" \
    -t "$REPO:latest" \
    -f "$ROOT_DIR/infra/docker/Dockerfile" \
    "$ROOT_DIR"

echo "[+] Pushing $REPO:$TAG..."
podman push "$REPO:$TAG"

if [[ "$TAG" != "latest" ]]; then
    echo "[+] Pushing $REPO:latest..."
    podman push "$REPO:latest"
fi

echo "[+] Done. Deploy on VPS with:"
echo "    ./infra/scripts/deploy.sh --server-only"
