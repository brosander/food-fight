#!/usr/bin/env bash
# Run the game on the Steam Deck from your Mac.
# Logs stream back to this terminal over SSH.
# The game renders on the Deck's screen (Desktop Mode required).
#
# Usage:
#   ./deck-run.sh          — sync + build + run
#   ./deck-run.sh run      — sync + run pre-built binary (skip build)
#   ./deck-run.sh sync     — sync only
#   ./deck-run.sh build    — sync + build only

set -euo pipefail

DECK="deck@steamdeck"
REMOTE_DIR="~/foodwars"
LOCAL_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONTAINER="foodwars-dev"

# Display environment for the Deck's active KDE/Plasma session.
# SteamOS Desktop Mode runs X11 on :0 by default.
# We pull the real values from the plasmashell process so this works
# even if the session is on a different display.
FIND_DISPLAY='
PLASMA_PID=$(pgrep -x plasmashell 2>/dev/null | head -1)
if [ -n "$PLASMA_PID" ]; then
    eval $(cat /proc/$PLASMA_PID/environ | tr "\0" "\n" \
        | grep -E "^(DISPLAY|WAYLAND_DISPLAY|XDG_RUNTIME_DIR)=" \
        | sed "s/^/export /")
else
    # Fallback: SteamOS Desktop Mode defaults
    export DISPLAY=:0
    export XDG_RUNTIME_DIR=/run/user/1000
fi
'

sync_code() {
    echo "==> Syncing code to $DECK:$REMOTE_DIR ..."
    rsync -avz --exclude target/ --exclude .git/ --exclude steam_appid.txt \
        "$LOCAL_DIR/" "$DECK:$REMOTE_DIR/"
}

run_in_container() {
    # $1 = command to run inside the distrobox
    ssh -t "$DECK" bash -lc "
        $FIND_DISPLAY
        export DISPLAY WAYLAND_DISPLAY XDG_RUNTIME_DIR 2>/dev/null || true
        distrobox enter --name $CONTAINER -- bash -lc '
            source \"\$HOME/.cargo/env\" 2>/dev/null || true
            cd $REMOTE_DIR
            $1
        '
    "
}

case "${1:-default}" in
    sync)
        sync_code
        ;;
    build)
        sync_code
        echo "==> Building (release) on Deck..."
        run_in_container "cargo build --release 2>&1"
        echo "==> Build complete."
        ;;
    run)
        sync_code
        echo "==> Running pre-built binary on Deck (logs streaming here)..."
        run_in_container "cargo run --release 2>&1"
        ;;
    default)
        sync_code
        echo "==> Building + running on Deck (logs streaming here)..."
        run_in_container "cargo run --release 2>&1"
        ;;
    *)
        echo "Usage: $0 {sync|build|run}"
        echo ""
        echo "  (no arg)  — sync + build + run"
        echo "  run       — sync + run (skip build if binary exists)"
        echo "  build     — sync + build only"
        echo "  sync      — rsync source only"
        ;;
esac
