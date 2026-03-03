"""
Melee weapon spritesheet generator.

Atlas layout: 6 columns x 2 rows, each cell 32x32px.  Total: 192x64px RGBA.
  Col 0: ground pickup, frame 1 (idle)
  Col 1: ground pickup, frame 2 (sparkle / pickup indicator)
  Col 2: equipped / held (static)
  Col 3: active frame 1  — parry flash (tray) / swing windup (baguette)
  Col 4: active frame 2  — block pulse A / swing peak
  Col 5: active frame 3  — block pulse B (dim) / swing recovery
  Row 0: LunchTray
  Row 1: Baguette

Animation sequences:
  Ground pickup   : cols 0-1, looping, 3 fps
  Equipped        : col 2, static
  LunchTray parry : col 3 (flash), one-shot
  LunchTray block : cols 4-5, looping, 4 fps (pulse)
  Baguette swing  : cols 3-5, one-shot, 15 fps, then return to col 2

Run via Docker:
    ./sprites/run.sh generate_melee.py

Output: assets/sprites/melee/melee_weapons.png
"""

import os
from PIL import Image

ASSETS  = "/assets"
OUT_PNG = f"{ASSETS}/sprites/melee/melee_weapons.png"

COLS = 6
ROWS = 2
CELL = 32

# ---------------------------------------------------------------------------
# Palette
# ---------------------------------------------------------------------------
T = (0, 0, 0, 0)   # transparent

# Lunch tray — gray plastic cafeteria tray
TR_RIM    = ( 80,  80,  74, 255)   # dark rim / border
TR_BODY   = (165, 165, 156, 255)   # main surface
TR_LIGHT  = (205, 205, 196, 255)   # highlight band
TR_SHINE  = (232, 232, 226, 255)   # bright shine spot
TR_SHADOW = (120, 120, 113, 255)   # shadow / depth
TR_DIV    = (135, 135, 127, 255)   # compartment divider
TR_GLOW   = (255, 215,  50, 255)   # active-block glow (gold)
TR_GLOW_DIM = (190, 150,  30, 255) # dim block pulse glow
TR_FLASH  = (255, 248, 180, 255)   # parry flash — bright white-gold
TR_SPARK  = (245, 245, 200, 255)   # sparkle dot

# Baguette — French bread weapon
BA_BORDER = (118,  68,  18, 255)   # dark crust outline
BA_CRUST  = (208, 148,  48, 255)   # main golden crust
BA_LIGHT  = (238, 200, 108, 255)   # baked highlight
BA_CRUMB  = (250, 232, 178, 255)   # interior crumb (cut ends)
BA_SCORE  = (160, 100,  26, 255)   # scoring crack lines
BA_SPARK  = (252, 230, 140, 255)   # sparkle / motion color


# ---------------------------------------------------------------------------
# Drawing helpers
# ---------------------------------------------------------------------------

def px(img, x, y, c):
    if 0 <= x < img.width and 0 <= y < img.height:
        img.putpixel((x, y), c)


def fill_rect(img, x1, y1, x2, y2, c):
    for y in range(y1, y2 + 1):
        for x in range(x1, x2 + 1):
            px(img, x, y, c)


def sparkle_4pt(img, cx, cy, main, arm):
    """4-point star sparkle."""
    px(img, cx,     cy,     main)
    px(img, cx - 1, cy,     arm)
    px(img, cx + 1, cy,     arm)
    px(img, cx,     cy - 1, arm)
    px(img, cx,     cy + 1, arm)


# ---------------------------------------------------------------------------
# LunchTray drawing
# ---------------------------------------------------------------------------

def draw_tray(img, ox, oy, *, glow=False, sparkle=False, parry=False, glow_dim=False):
    """
    Draw a cafeteria lunch tray inside the 32x32 cell whose top-left is (ox, oy).
    The tray is 26px wide × 11px tall, centred vertically.

    glow=True    : gold 1px outline (block state)
    parry=True   : bright 2px white-gold flash (parry flash — overrides glow)
    glow_dim=True: dimmer gold outline (block pulse B — used with glow=True)
    sparkle=True : pickup star indicator
    """
    tx = ox + 3   # tray left edge
    ty = oy + 11  # tray top edge
    tw = 26       # tray width
    th = 11       # tray height

    # -- Body fill
    fill_rect(img, tx + 1, ty + 1, tx + tw - 2, ty + th - 2, TR_BODY)

    # -- Highlight band (2nd pixel row from top)
    fill_rect(img, tx + 2, ty + 2, tx + tw - 3, ty + 3, TR_LIGHT)

    # -- Shadow band (2nd pixel row from bottom)
    fill_rect(img, tx + 2, ty + th - 3, tx + tw - 3, ty + th - 2, TR_SHADOW)

    # -- Rim / border (1px)
    for x in range(tx, tx + tw):
        px(img, x, ty,          TR_RIM)
        px(img, x, ty + th - 1, TR_RIM)
    for y in range(ty + 1, ty + th - 1):
        px(img, tx,          y, TR_RIM)
        px(img, tx + tw - 1, y, TR_RIM)

    # -- Round the four corners (remove border pixel)
    for cx2, cy2 in [(tx, ty), (tx + tw - 1, ty),
                     (tx, ty + th - 1), (tx + tw - 1, ty + th - 1)]:
        px(img, cx2, cy2, T)

    # -- Shine spot (top-left quadrant)
    px(img, tx + 3, ty + 2, TR_SHINE)
    px(img, tx + 4, ty + 2, TR_SHINE)
    px(img, tx + 3, ty + 3, TR_SHINE)

    # -- Compartment divider (horizontal, centre of tray)
    mid_y = ty + th // 2
    for x in range(tx + 6, tx + tw - 6):
        px(img, x, mid_y, TR_DIV)

    # -- Parry flash: 2px wide bright glow + inner gold ring
    if parry:
        # Outer flash ring (2px out)
        for x in range(tx - 2, tx + tw + 2):
            px(img, x, ty - 2,     TR_FLASH)
            px(img, x, ty + th + 1, TR_FLASH)
        for y in range(ty - 1, ty + th + 1):
            px(img, tx - 2,     y, TR_FLASH)
            px(img, tx + tw + 1, y, TR_FLASH)
        # Inner gold ring (1px out)
        for x in range(tx - 1, tx + tw + 1):
            px(img, x, ty - 1,     TR_GLOW)
            px(img, x, ty + th,    TR_GLOW)
        for y in range(ty, ty + th):
            px(img, tx - 1,     y, TR_GLOW)
            px(img, tx + tw,    y, TR_GLOW)
        px(img, tx - 1, ty - 1,     TR_GLOW)
        px(img, tx + tw, ty - 1,    TR_GLOW)
        px(img, tx - 1, ty + th,    TR_GLOW)
        px(img, tx + tw, ty + th,   TR_GLOW)

    # -- Regular glow outline (1px) for block state
    elif glow:
        gc = TR_GLOW_DIM if glow_dim else TR_GLOW
        for x in range(tx - 1, tx + tw + 1):
            px(img, x, ty - 1,     gc)
            px(img, x, ty + th,    gc)
        for y in range(ty, ty + th):
            px(img, tx - 1,     y, gc)
            px(img, tx + tw,    y, gc)
        px(img, tx - 1, ty - 1,     gc)
        px(img, tx + tw, ty - 1,    gc)
        px(img, tx - 1, ty + th,    gc)
        px(img, tx + tw, ty + th,   gc)

    # -- Optional sparkle (pickup indicator, top-right area)
    if sparkle:
        sparkle_4pt(img, tx + tw + 1, ty - 2, TR_SHINE, TR_SPARK)


def draw_tray_row(img, y_base):
    draw_tray(img, 0 * CELL, y_base)                              # col 0: ground idle
    draw_tray(img, 1 * CELL, y_base, sparkle=True)                # col 1: ground + sparkle
    draw_tray(img, 2 * CELL, y_base)                              # col 2: equipped (same idle)
    draw_tray(img, 3 * CELL, y_base, parry=True)                  # col 3: parry flash (bright)
    draw_tray(img, 4 * CELL, y_base, glow=True)                   # col 4: block pulse A (gold)
    draw_tray(img, 5 * CELL, y_base, glow=True, glow_dim=True)    # col 5: block pulse B (dim)


# ---------------------------------------------------------------------------
# Baguette drawing
# ---------------------------------------------------------------------------

def baguette_pixels_horizontal(cx, cy, length=28, thickness=7):
    """
    Return a list of (x, y, color) for a horizontal baguette centred at (cx, cy).
    thickness must be odd (centre + arms).
    """
    half_l = length // 2
    half_t = thickness // 2
    pixels = []

    for dx in range(-half_l, half_l + 1):
        x = cx + dx
        for dt in range(-half_t, half_t + 1):
            y = cy + dt
            is_left_tip  = dx == -half_l
            is_right_tip = dx ==  half_l
            is_top_edge  = dt == -half_t
            is_bot_edge  = dt ==  half_t

            # Corners of bread body — round off extreme corners
            if (is_left_tip or is_right_tip) and (is_top_edge or is_bot_edge):
                # skip — transparent corners give rounded ends
                continue
            elif is_left_tip and abs(dt) == half_t - 1:
                pixels.append((x, y, BA_CRUMB))   # crumb at left cut end
            elif is_right_tip and abs(dt) == half_t - 1:
                pixels.append((x, y, BA_CRUMB))   # crumb at right cut end
            elif is_left_tip or is_right_tip:
                pixels.append((x, y, BA_BORDER))
            elif is_top_edge or is_bot_edge:
                pixels.append((x, y, BA_BORDER))
            elif dt == -half_t + 1:
                pixels.append((x, y, BA_LIGHT))   # top highlight row
            elif dt ==  half_t - 1:
                pixels.append((x, y, BA_CRUST))   # lower body
            else:
                pixels.append((x, y, BA_CRUST))

    # Score marks (diagonal cuts across top)
    for sx in range(cx - half_l + 5, cx + half_l - 4, 5):
        pixels.append((sx,     cy - half_t + 2, BA_SCORE))
        pixels.append((sx + 1, cy - half_t + 3, BA_SCORE))

    return pixels


def baguette_pixels_angled(cx, cy, rise_total, length=26, thickness=5):
    """
    Return pixels for a baguette at a given angle.
    rise_total: total vertical displacement (negative = right end higher).
    Used for diagonal, windup, and recovery frames.
    """
    pixels = []
    half_l = length // 2

    centres = []
    for t in range(-half_l, half_l + 1):
        bx = cx + t
        by = cy + round(rise_total * t / (2 * half_l))
        centres.append((bx, by))

    for i, (bx, by) in enumerate(centres):
        is_tip_l = i == 0
        is_tip_r = i == len(centres) - 1
        for dt in range(-thickness // 2, thickness // 2 + 1):
            y = by + dt
            is_top = dt == -thickness // 2
            is_bot = dt ==  thickness // 2
            if (is_tip_l or is_tip_r) and (is_top or is_bot):
                continue
            elif is_tip_l or is_tip_r:
                c = BA_CRUMB if abs(dt) < thickness // 2 else BA_BORDER
            elif is_top or is_bot:
                c = BA_BORDER
            elif dt == -thickness // 2 + 1:
                c = BA_LIGHT
            else:
                c = BA_CRUST
            pixels.append((bx, y, c))

    # Score marks along the top edge
    for bx, by in centres[4:-4:6]:
        pixels.append((bx,     by - thickness // 2 + 1, BA_SCORE))
        pixels.append((bx + 1, by - thickness // 2 + 2, BA_SCORE))

    return pixels


def baguette_pixels_diagonal(cx, cy, length=26, thickness=5):
    """Baguette angled ~18° upward (slight upward tilt, left to right) — swing peak."""
    return baguette_pixels_angled(cx, cy, rise_total=-4, length=length, thickness=thickness)


def baguette_pixels_windup(cx, cy, length=24, thickness=5):
    """Baguette pulled back for swing — steep upward tilt (rise_total = -8)."""
    return baguette_pixels_angled(cx, cy, rise_total=-8, length=length, thickness=thickness)


def baguette_pixels_recovery(cx, cy, length=24, thickness=5):
    """Baguette follow-through after swing — slight downward tilt (rise_total = +5)."""
    return baguette_pixels_angled(cx, cy, rise_total=5, length=length, thickness=thickness)


def draw_baguette(img, ox, oy, *, diagonal=False, sparkle=False, motion=False,
                  windup=False, recovery=False):
    """Draw a baguette in a 32x32 cell at (ox, oy)."""
    cx = ox + 16
    cy = oy + 16

    if windup:
        pixels = baguette_pixels_windup(cx, cy)
    elif recovery:
        pixels = baguette_pixels_recovery(cx, cy)
    elif diagonal:
        pixels = baguette_pixels_diagonal(cx, cy)
    else:
        pixels = baguette_pixels_horizontal(cx, cy)

    for x, y, c in pixels:
        px(img, x, y, c)

    # Motion lines for peak swing (to the left, trailing behind a rightward swing)
    if motion:
        start_x = cx - 14
        for i, trail_x in enumerate(range(start_x - 1, start_x - 8, -1)):
            alpha = 160 - i * 25
            if alpha <= 0:
                break
            for dy in (-2, 0, 2):
                px(img, trail_x, cy + dy, (*BA_SPARK[:3], alpha))

    # Short fading trail for recovery (baguette completing the arc)
    if recovery:
        for i, trail_x in enumerate(range(cx - 13, cx - 17, -1)):
            alpha = 90 - i * 28
            if alpha <= 0:
                break
            for dy in (-1, 0, 1):
                px(img, trail_x, cy + dy, (*BA_SPARK[:3], alpha))

    # Pickup sparkle (small 4-point star, top-right)
    if sparkle:
        sparkle_4pt(img, cx + 13, cy - 5, BA_SPARK, BA_LIGHT)


def draw_baguette_row(img, y_base):
    draw_baguette(img, 0 * CELL, y_base)                              # col 0: ground idle
    draw_baguette(img, 1 * CELL, y_base, sparkle=True)                # col 1: pickup sparkle
    draw_baguette(img, 2 * CELL, y_base)                              # col 2: equipped
    draw_baguette(img, 3 * CELL, y_base, windup=True)                 # col 3: swing windup
    draw_baguette(img, 4 * CELL, y_base, diagonal=True, motion=True)  # col 4: swing peak
    draw_baguette(img, 5 * CELL, y_base, recovery=True)               # col 5: swing recovery


# ---------------------------------------------------------------------------
# Row registry
# ---------------------------------------------------------------------------

ROW_GENERATORS = [
    (0, draw_tray_row),
    (1, draw_baguette_row),
]


def main():
    os.makedirs(os.path.dirname(OUT_PNG), exist_ok=True)
    img = Image.new("RGBA", (COLS * CELL, ROWS * CELL), T)

    for row_idx, draw_fn in ROW_GENERATORS:
        y_base = row_idx * CELL
        print(f"Drawing row {row_idx} at y={y_base}…")
        draw_fn(img, y_base)

    img.save(OUT_PNG)
    print(f"Saved {OUT_PNG}  ({img.width}x{img.height})")


if __name__ == "__main__":
    main()
