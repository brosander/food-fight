#!/usr/bin/env bash
# Run on the Steam Deck in desktop mode (Konsole).
# Usage:
#   ./steamdeck.sh setup   — one-time: create container, install deps + Rust
#   ./steamdeck.sh build   — compile release binary inside container
#   ./steamdeck.sh run     — compile (if needed) and run

set -euo pipefail

CONTAINER="foodwars-dev"
PROJECT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

in_container() {
    distrobox enter --name "$CONTAINER" -- bash -c "$1"
}

setup() {
    if ! command -v distrobox &>/dev/null; then
        echo "ERROR: distrobox not found. Update SteamOS to 3.4+ or install it manually."
        exit 1
    fi

    if distrobox list 2>/dev/null | grep -q "$CONTAINER"; then
        echo "Container '$CONTAINER' already exists — skipping creation."
    else
        echo "==> Creating Ubuntu 22.04 container '$CONTAINER'..."
        echo "    (Ubuntu 22.04 uses glibc 2.35, which runs on the SteamOS host.)"
        distrobox create --name "$CONTAINER" --image ubuntu:22.04
    fi

    echo "==> Installing build dependencies..."
    in_container "
        sudo apt-get update -qq
        sudo apt-get install -y --no-install-recommends \
            build-essential \
            pkg-config \
            curl \
            libasound2-dev \
            libudev-dev \
            libx11-dev \
            libxcursor-dev \
            libxi-dev \
            libxkbcommon-dev \
            libxkbcommon-x11-0 \
            libxrandr-dev \
            libxrender-dev \
            libwayland-dev \
            libvulkan-dev
    "

    echo "==> Installing Rust toolchain..."
    in_container "
        if command -v cargo &>/dev/null; then
            echo 'Rust already installed: '\$(rustc --version)
        else
            curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable
        fi
    "

    echo ""
    echo "Setup complete. Run './steamdeck.sh run' to build and launch."
}

build() {
    echo "==> Building (release)..."
    in_container "
        source \"\$HOME/.cargo/env\" 2>/dev/null || true
        cd '$PROJECT'
        cargo build --release
    "
    echo "==> Binary: $PROJECT/target/release/food_fight"
}

run_game() {
    echo "==> Building and running..."
    in_container "
        source \"\$HOME/.cargo/env\" 2>/dev/null || true
        cd '$PROJECT'
        cargo run --release
    "
}

case "${1:-help}" in
    setup) setup ;;
    build) build ;;
    run)   run_game ;;
    *)
        echo "Usage: $0 {setup|build|run}"
        echo ""
        echo "  setup  — one-time: create container, install deps + Rust (~5 min)"
        echo "  build  — compile release binary"
        echo "  run    — compile and launch the game"
        ;;
esac
