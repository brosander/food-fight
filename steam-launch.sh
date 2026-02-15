#!/usr/bin/env bash
# Launch script for Steam (Non-Steam Game entry).
# Point Steam at this file: /home/deck/foodwars/steam-launch.sh
#
# Gaming Mode sandboxes /etc/passwd so podman/distrobox can't start.
# Build with the Ubuntu 22.04 distrobox (glibc 2.35) so the binary runs
# directly on the SteamOS host without any container at runtime.
export BEVY_ASSET_ROOT=/home/deck/foodwars
# Use Gamescope's native Wayland compositor instead of XWayland.
# WAYLAND_DISPLAY is not set by default in Gaming Mode but GAMESCOPE_WAYLAND_DISPLAY is.
# Native Wayland clients get proper focus without the XWayland atom mismatch.
export WAYLAND_DISPLAY="${GAMESCOPE_WAYLAND_DISPLAY:-gamescope-0}"
exec /home/deck/foodwars/target/release/food_fight
