#!/usr/bin/env bash
# Run a sprite generation script inside Docker (with Pillow installed).
# Usage: ./sprites/run.sh <script.py>
# Example: ./sprites/run.sh generate_launchers.py
set -euo pipefail

SCRIPT="${1:?Usage: $0 <script.py>}"
SPRITES_DIR="$(cd "$(dirname "$0")" && pwd)"
ASSETS_DIR="$(cd "$SPRITES_DIR/../assets" && pwd)"

docker run --rm \
  -v "$SPRITES_DIR:/scripts" \
  -v "$ASSETS_DIR:/assets" \
  python:3.12-slim \
  bash -c "pip install pillow -q --root-user-action=ignore 2>/dev/null && python3 /scripts/$SCRIPT"
