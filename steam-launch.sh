#!/usr/bin/env bash
# Launch script for Steam (Non-Steam Game entry).
# Point Steam at this file: /home/deck/foodwars/steam-launch.sh
#
# Gaming Mode sandboxes /etc/passwd so podman/distrobox can't start.
# Build with the Ubuntu 22.04 distrobox (glibc 2.35) so the binary runs
# directly on the SteamOS host without any container at runtime.
export BEVY_ASSET_ROOT=/home/deck/foodwars

# Propagate Steam's shortcut ID to the env var Gamescope WSI reads when
# associating a Wayland surface with the active game (steam app id in surface state).
# Steam sets SteamGameId (camelCase); the WSI layer reads STEAM_GAME_ID.
export STEAM_GAME_ID="${SteamGameId:-0}"

# Dump the Gaming Mode environment so we can see what's actually set.
#env > /tmp/foodwars-env.log 2>&1

exec /home/deck/foodwars/target/release/food_fight
