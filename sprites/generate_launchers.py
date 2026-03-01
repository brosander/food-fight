"""
Launcher spritesheet generator.

Atlas layout: 5 columns x N rows, each cell 32x32px.
Columns: [ground_pickup, equipped_held, firing_1, firing_2, depleted]

Run via Docker:
    ./sprites/run.sh generate_launchers.py

This script is additive — it reads the existing launchers.png and adds
any missing rows, so you can re-run it safely without losing existing sprites.
"""

from PIL import Image

ASSETS = "/assets"
LAUNCHER_PNG = f"{ASSETS}/sprites/launchers/launchers.png"

COLS = 5
CELL = 32

# ---------------------------------------------------------------------------
# Palette
# ---------------------------------------------------------------------------
T    = (  0,   0,   0,   0)   # transparent
GD   = ( 28, 115,  28, 255)   # watermelon dark green skin
GL   = ( 80, 175,  45, 255)   # watermelon light green stripe
GH   = (150, 230, 100, 255)   # highlight spec
GBDR = ( 12,  55,  12, 255)   # dark green border
WD   = (138,  87,  38, 255)   # wood
WDK  = ( 92,  56,  18, 255)   # wood shadow
EL   = (215, 195,  85, 255)   # elastic band


# ---------------------------------------------------------------------------
# Drawing primitives (work directly on the Image object)
# ---------------------------------------------------------------------------

def px(img, x, y, c):
    if 0 <= x < img.width and 0 <= y < img.height:
        img.putpixel((x, y), c)


def draw_watermelon(img, cx, cy, rx=10, ry=9):
    """Striped green oval with a dark outline and a highlight spec."""
    for y in range(cy - ry - 1, cy + ry + 2):
        for x in range(cx - rx - 1, cx + rx + 2):
            dx = x - cx
            dy = y - cy
            in_outer = (dx / (rx + 0.5)) ** 2 + (dy / (ry + 0.5)) ** 2 <= 1.0
            in_inner = (dx / rx) ** 2 + (dy / ry) ** 2 <= 1.0
            if in_outer and not in_inner:
                px(img, x, y, GBDR)
            elif in_inner:
                px(img, x, y, GL if (dx + 100) % 5 <= 1 else GD)
    # Highlight (top-left quadrant)
    hx = cx - max(rx // 3, 1)
    hy = cy - max(ry // 3, 1)
    px(img, hx, hy, GH)
    px(img, hx + 1, hy, GH)


def draw_line(img, x1, y1, x2, y2, c1, c2=None):
    """2px-wide line; c2 is the shadow color (defaults to c1)."""
    if c2 is None:
        c2 = c1
    dx = x2 - x1
    dy = y2 - y1
    steps = max(abs(dx), abs(dy), 1)
    for i in range(steps + 1):
        x = round(x1 + dx * i / steps)
        y = round(y1 + dy * i / steps)
        px(img, x, y, c1)
        if abs(dy) >= abs(dx):   # more vertical → shadow right
            px(img, x + 1, y, c2)
        else:                    # more horizontal → shadow below
            px(img, x, y + 1, c2)


def draw_elastic(img, x1, y1, x2, y2):
    """Single-pixel straight elastic band."""
    dx = x2 - x1
    dy = y2 - y1
    steps = max(abs(dx), abs(dy), 1)
    for i in range(steps + 1):
        px(img, round(x1 + dx * i / steps), round(y1 + dy * i / steps), EL)


# ---------------------------------------------------------------------------
# WatermelonCatapult row  (row index 5, y_base = 5 * 32 = 160)
#
# Visual concept: Y-shaped wooden slingshot with a big watermelon as the
# projectile. Single-shot, devastating.
#
# Each frame is 32x32. Absolute x ranges:
#   frame 0  x=[0,31]
#   frame 1  x=[32,63]
#   frame 2  x=[64,95]
#   frame 3  x=[96,127]
#   frame 4  x=[128,159]
# ---------------------------------------------------------------------------

def draw_watermelon_catapult_row(img, y_base):
    Y = y_base

    # ---- Frame 0 (x=[0,31]): Ground pickup --------------------------------
    # Watermelon (big, left) + upright slingshot Y-frame (right)
    draw_watermelon(img, 11, Y + 18, 10, 9)
    draw_line(img, 24, Y + 29, 24, Y + 19, WD, WDK)   # stem
    draw_line(img, 24, Y + 19, 18, Y + 12, WD, WDK)   # left fork
    draw_line(img, 24, Y + 19, 30, Y + 12, WD, WDK)   # right fork
    draw_elastic(img, 18, Y + 12, 15, Y + 15)          # loose left band
    draw_elastic(img, 30, Y + 12, 31, Y + 15)          # loose right band

    # ---- Frame 1 (x=[32,63]): Equipped / held ------------------------------
    # Horizontal slingshot pointing right, watermelon loaded in the fork.
    draw_line(img, 36, Y + 16, 46, Y + 16, WD, WDK)   # handle
    draw_line(img, 46, Y + 16, 52, Y + 10, WD, WDK)   # upper fork
    draw_line(img, 46, Y + 16, 52, Y + 22, WD, WDK)   # lower fork
    draw_watermelon(img, 56, Y + 16, 7, 6)             # x=[49,63] ✓
    draw_elastic(img, 52, Y + 10, 50, Y + 11)          # upper band (slack)
    draw_elastic(img, 52, Y + 22, 50, Y + 21)          # lower band (slack)

    # ---- Frame 2 (x=[64,95]): Firing 1 — pulled back ----------------------
    # Watermelon dragged left, bands fully stretched.
    draw_line(img, 79, Y + 16, 86, Y + 16, WD, WDK)   # handle
    draw_line(img, 86, Y + 16, 92, Y + 10, WD, WDK)   # upper fork
    draw_line(img, 86, Y + 16, 92, Y + 22, WD, WDK)   # lower fork
    draw_watermelon(img, 70, Y + 16, 6, 6)             # x=[64,76] ✓
    draw_elastic(img, 92, Y + 10, 76, Y + 11)          # stretched upper band
    draw_elastic(img, 92, Y + 22, 76, Y + 21)          # stretched lower band

    # ---- Frame 3 (x=[96,127]): Firing 2 — released ------------------------
    # Watermelon flying right past the fork, bands snapped forward.
    draw_line(img, 100, Y + 16, 110, Y + 16, WD, WDK)  # handle
    draw_line(img, 110, Y + 16, 116, Y + 10, WD, WDK)  # upper fork
    draw_line(img, 110, Y + 16, 116, Y + 22, WD, WDK)  # lower fork
    draw_watermelon(img, 122, Y + 16, 6, 6)             # x=[116,128] ✓
    draw_elastic(img, 116, Y + 10, 118, Y + 12)         # snapped upper
    draw_elastic(img, 116, Y + 22, 118, Y + 20)         # snapped lower
    # Speed dots
    px(img, 127, Y + 14, EL)
    px(img, 127, Y + 18, EL)

    # ---- Frame 4 (x=[128,159]): Depleted — empty slingshot ----------------
    draw_line(img, 135, Y + 16, 145, Y + 16, WD, WDK)  # handle
    draw_line(img, 145, Y + 16, 151, Y + 10, WD, WDK)  # upper fork
    draw_line(img, 145, Y + 16, 151, Y + 22, WD, WDK)  # lower fork
    draw_elastic(img, 151, Y + 10, 155, Y + 14)         # limp upper band
    draw_elastic(img, 151, Y + 22, 155, Y + 18)         # limp lower band


# ---------------------------------------------------------------------------
# Row registry — add new rows here
# ---------------------------------------------------------------------------
# Each entry: (row_index, draw_fn)
# draw_fn signature: draw_fn(img, y_base)
ROW_GENERATORS = [
    (5, draw_watermelon_catapult_row),
]


def main():
    img = Image.open(LAUNCHER_PNG).convert("RGBA")
    w, h = img.size
    current_rows = h // CELL

    max_row = max(r for r, _ in ROW_GENERATORS)
    needed_rows = max(current_rows, max_row + 1)

    if needed_rows > current_rows:
        new_img = Image.new("RGBA", (w, needed_rows * CELL), T)
        new_img.paste(img, (0, 0))
    else:
        new_img = img.copy()

    for row_idx, draw_fn in ROW_GENERATORS:
        y_base = row_idx * CELL
        print(f"Drawing row {row_idx} at y={y_base}…")
        draw_fn(new_img, y_base)

    new_img.save(LAUNCHER_PNG)
    print(f"Saved {LAUNCHER_PNG}  ({new_img.width}x{new_img.height})")


if __name__ == "__main__":
    main()
