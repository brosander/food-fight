# Cafeteria Food Fight

A chaotic 2D top-down multiplayer food fight for 1–4 players. Students battle with hurled pizza, bouncing meatballs, and arcing jello in a school cafeteria — while avoiding the Teacher, Principal, and Lunch Lady who are very much not okay with any of this.

Built with Rust and Bevy 0.15. Native on macOS and Steam Deck.

---

## Gameplay

- **Up to 4 players** join with their gamepads and fight to knock each other out
- **8 food types** — pizza, meatball, jello, sandwich, apple, soup, cookie, cake — each with different speed, damage, and trajectory
- **6 launchers** scattered around the map for big plays: slingshot, catapult, soda cannon, and more
- **3 projectile trajectories** — straight, arcing (with gravity), and bouncing (wall reflections)
- **NPC authority figures** patrol the cafeteria and chase anyone they catch in the act:
  - **Teacher** — immediately hunts down whoever picks up a launcher, no questions asked
  - **Principal** — wide detection cone, long stun if caught
  - **Lunch Lady** — stationary at the counter but watching
- **Elimination & lunch detention** — when a player's health hits zero they are banished to a corner table for the rest of the round. Each corner has a dedicated detention seat (one per player). Eliminated players can't move, throw, or interact — they just sit there watching.
- Last player standing wins the round
- **Multi-round sessions** — after each round the scoreboard shows cumulative damage dealt and **Detention Slips** (how many rivals you personally sent to lunch detention). Press **START** to play another round with the same players and carry the scores forward, or **B (East)** to return to the main menu and reset everything

Controls are fully gamepad-driven. Plug in Xbox 360 / Xbox One / PS4 / PS5 / Switch Pro controllers and press **A (South)** to join the lobby.

---

## Controls

| Action | Button |
|---|---|
| Move | Left stick |
| Aim | Right stick |
| Pick up food | A / Cross (South) |
| Pick up launcher | X / Square (West) |
| Throw / Fire | RT |
| Pause / Ready | Start |
| Leave lobby | B / Circle (East) |
| Quit | Select / Back |

---

## Building on macOS

### Prerequisites

```bash
xcode-select --install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

No Vulkan SDK needed — Bevy uses Metal on macOS.

### Run

```bash
cargo run --release
```

The game opens in a 1280×800 window. Connect Bluetooth controllers via **System Settings → Bluetooth** before launching. The lobby detects them automatically.

For Bluetooth controller pairing troubleshooting, see [MACBOOK.md](MACBOOK.md).

---

## Building for Steam Deck

The Steam Deck runs SteamOS (immutable filesystem). We build inside a persistent Ubuntu 22.04 container via Distrobox so the binary links against glibc 2.35, which is compatible with the host at runtime without needing the container to be active.

### One-time setup on the Deck (Desktop Mode)

`steamdeck.sh` automates the container creation, dependency install, and build. Run it from Konsole on the Deck:

```bash
./steamdeck.sh setup   # one-time: creates Ubuntu 22.04 container, installs Rust + deps
./steamdeck.sh build   # compile release binary
./steamdeck.sh run     # compile and launch directly (Desktop Mode)
```

### Sync source and build

From your development machine, sync the source to the Deck (the script handles the rest):

```bash
rsync -avz --exclude target/ --exclude .git/ --exclude steam_appid.txt \
    ~/path/to/foodwars/ deck@steamdeck:~/foodwars/
```

Then on the Deck:

```bash
./steamdeck.sh build
```

### Add to Steam (Gaming Mode)

1. In Desktop Mode, open Steam → **Add a Game → Add a Non-Steam Game**
2. Browse to `steam-launch.sh` at `/home/deck/foodwars/steam-launch.sh`
3. Switch to Gaming Mode — the game appears in your library

The launch script sets up the environment Gamescope needs to transition from the Steam UI to the game. Do not put `steam_appid.txt` on the Deck — it breaks the Gaming Mode display handoff.

> **Note:** Wired Xbox 360 controllers work out of the box via the `xpad` kernel driver. Plug them into a USB hub connected to the Deck.

For the full deployment guide, see [STEAMDECK.md](STEAMDECK.md).

---

## Tech Stack

- **[Bevy 0.15](https://bevyengine.org/)** — ECS game engine
- **[gilrs](https://gitlab.com/gilrs-project/gilrs)** — gamepad input (default)
- **[steamworks 0.12](https://github.com/nickel-org/steamworks-rs)** — Steam Input backend (optional, `--features steam`)
- Custom AABB collision — no physics engine
- Procedural map — no tilemap crate

---

## Project Status

The core game loop is complete and playable.

- [x] Movement, collision, play area bounds
- [x] All 8 food types with distinct stats
- [x] 6 launcher types with cooldowns and charge-up
- [x] 3 projectile trajectories (straight, arc, bounce)
- [x] NPC state machines (patrol → suspicious → chase → catch)
- [x] Dynamic lobby (1–4 players, press to join)
- [x] Full game flow: menu → lobby → playing → round over
- [x] Elimination & lunch detention (corner tables, last player standing wins)
- [x] Multi-round sessions with cumulative scoring (damage + detention slips)
- [x] Steam Deck Gaming Mode support
- [x] Sprite art for players, NPCs, food, launchers, effects, UI
- [x] Audio
- [ ] Additional maps
