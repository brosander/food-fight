"""Cafeteria background sprite — 960×640 RGBA PNG.

Draws the full cafeteria layout at 1:1 game resolution:
  Floor:           tiled linoleum, 32px tiles with 1px grout lines
  Perimeter walls: 16px thick dark border (top/bottom/left/right)
  Tables:          6 cafeteria tables in 2×3 grid, 120×24 each
  Lunch counter:   400×20 at top (y=260)
  Pillars:         24×80 each at x=±380
  Detention tables: 80×50 at the four corners
  Door markers:    40×20 at bottom-left/right player entry points
  Melee markers:   40×16 subtle X marking at (-280, 50) and (280, -50)

Game coordinate system: origin at center, y-up.
  Play area: x ∈ [-480, 480], y ∈ [-320, 320]
  Image coords: img_x = game_x + 480, img_y = 320 - game_y

Usage:  ./sprites/run.sh generate_cafeteria.py
Output: /assets/sprites/map/cafeteria_bg.png
"""

import os
from PIL import Image, ImageDraw

OUT_PNG = "/assets/sprites/map/cafeteria_bg.png"
W, H = 960, 640

# ---------------------------------------------------------------------------
# Color palette — named constants only, no raw tuples inline
# ---------------------------------------------------------------------------

# Floor — cream linoleum tiles
FL_TILE_A  = (218, 208, 193, 255)   # light cream tile
FL_TILE_B  = (210, 200, 185, 255)   # slightly darker checker tile
FL_GROUT   = (190, 178, 160, 255)   # 1px grout line

# Walls — dark brown, viewed from above
WL_FACE    = (112, 92, 72, 255)     # wall top surface
WL_SHADOW  = (80,  63, 48, 255)     # inner shadow strip

# Cafeteria tables — warm tan wood
TB_SURFACE = (178, 132, 78, 255)    # table top
TB_EDGE    = (135, 96,  48, 255)    # bottom/right shadow edge
TB_HILITE  = (195, 150, 95, 255)    # top highlight strip
TB_GRAIN   = (168, 123, 72, 255)    # wood grain lines

# Lunch counter — light stainless/beige
CT_SURFACE = (214, 208, 198, 255)   # counter surface
CT_EDGE    = (170, 162, 150, 255)   # front edge shadow
CT_DIV     = (198, 192, 180, 255)   # section divider lines

# Pillars — grey-brown concrete
PL_FACE    = (150, 140, 127, 255)   # pillar surface
PL_SHADOW  = (103,  93,  82, 255)   # edge shadow
PL_DETAIL  = (135, 126, 114, 255)   # horizontal detail lines

# Detention tables — dark scratched wood
DT_SURFACE = (112, 72,  35, 255)    # dark wood surface
DT_EDGE    = ( 80, 50,  22, 255)    # darker edge
DT_SCRATCH = (103, 65,  30, 255)    # scratch marks for character

# Door markers — faintly green floor tile
DR_FLOOR   = (200, 210, 196, 255)   # greenish floor variant
DR_BORDER  = (163, 172, 159, 255)   # door frame outline

# Melee spawn markers — lighter floor with subtle X
ML_BASE    = (208, 198, 180, 255)   # slightly lighter floor spot
ML_CROSS   = (162, 150, 130, 255)   # X line color

# ---------------------------------------------------------------------------
# Coordinate helpers
# ---------------------------------------------------------------------------

def px(game_x: float) -> int:
    """Game x → image x."""
    return int(game_x + 480)

def py(game_y: float) -> int:
    """Game y → image y (flipped)."""
    return int(320 - game_y)

def grect(game_x: float, game_y: float, w: float, h: float):
    """(game center, size) → pixel (x1, y1, x2, y2)."""
    cx, cy = px(game_x), py(game_y)
    hw, hh = int(w // 2), int(h // 2)
    return cx - hw, cy - hh, cx + hw, cy + hh

# ---------------------------------------------------------------------------
# Drawing helpers — all rects use ImageDraw.rectangle (C-level, fast)
# ---------------------------------------------------------------------------

def fr(d, x1, y1, x2, y2, color):
    """Fill rect [x1,y1) to [x2,y2) — exclusive right/bottom."""
    if x2 > x1 and y2 > y1:
        d.rectangle([x1, y1, x2 - 1, y2 - 1], fill=color)

# ---------------------------------------------------------------------------
# Element draw functions
# ---------------------------------------------------------------------------

def draw_floor(d):
    """32×32 linoleum tiles with 1px grout lines."""
    TILE = 32
    fr(d, 0, 0, W, H, FL_GROUT)  # grout fills background
    cols = W // TILE + 1
    rows = H // TILE + 1
    for ty in range(rows):
        for tx in range(cols):
            x1 = tx * TILE + 1
            y1 = ty * TILE + 1
            x2 = min((tx + 1) * TILE, W)
            y2 = min((ty + 1) * TILE, H)
            color = FL_TILE_A if (tx + ty) % 2 == 0 else FL_TILE_B
            fr(d, x1, y1, x2, y2, color)


def draw_wall_h(d, x1, y1, x2, y2):
    """Horizontal wall band."""
    fr(d, x1, y1, x2, y2, WL_FACE)
    fr(d, x1, y2 - 3, x2, y2, WL_SHADOW)


def draw_wall_v(d, x1, y1, x2, y2):
    """Vertical wall band."""
    fr(d, x1, y1, x2, y2, WL_FACE)
    fr(d, x2 - 3, y1, x2, y2, WL_SHADOW)


def draw_table(d, x1, y1, x2, y2):
    """Top-down cafeteria table with wood grain."""
    fr(d, x1, y1, x2, y2, TB_SURFACE)
    # Bottom and right shadow
    fr(d, x1, y2 - 3, x2, y2, TB_EDGE)
    fr(d, x2 - 2, y1, x2, y2, TB_EDGE)
    # Top highlight
    fr(d, x1, y1, x2, y1 + 2, TB_HILITE)
    # Vertical grain lines every 8px along table length
    w = x2 - x1
    for i in range(1, w // 8):
        lx = x1 + i * 8
        if lx < x2 - 3:
            fr(d, lx, y1 + 2, lx + 1, y2 - 3, TB_GRAIN)


def draw_counter(d, x1, y1, x2, y2):
    """Lunch counter with section dividers."""
    fr(d, x1, y1, x2, y2, CT_SURFACE)
    # Front edge (bottom) and side caps
    fr(d, x1, y2 - 3, x2, y2, CT_EDGE)
    fr(d, x1, y1, x1 + 2, y2, CT_EDGE)
    fr(d, x2 - 2, y1, x2, y2, CT_EDGE)
    # Section divider lines every 50px
    w = x2 - x1
    for i in range(1, w // 50 + 1):
        lx = x1 + i * 50
        if lx < x2 - 3:
            fr(d, lx, y1, lx + 1, y2 - 3, CT_DIV)


def draw_pillar(d, x1, y1, x2, y2):
    """Concrete pillar with edge shadows and horizontal detail lines."""
    fr(d, x1, y1, x2, y2, PL_FACE)
    # Edge shadows on all four sides
    fr(d, x1,     y1, x1 + 3, y2, PL_SHADOW)
    fr(d, x1,     y1, x2,     y1 + 3, PL_SHADOW)
    fr(d, x2 - 3, y1, x2,     y2, PL_SHADOW)
    # Horizontal detail lines every 10px
    h = y2 - y1
    for i in range(1, h // 10):
        ly = y1 + i * 10
        if ly < y2 - 3:
            fr(d, x1 + 3, ly, x2 - 3, ly + 1, PL_DETAIL)


def draw_detention_table(d, x1, y1, x2, y2):
    """Dark corner detention table with scratches."""
    fr(d, x1, y1, x2, y2, DT_SURFACE)
    fr(d, x1, y2 - 3, x2, y2, DT_EDGE)
    fr(d, x2 - 2, y1, x2, y2, DT_EDGE)
    # Scratch marks — give it a worn, isolating vibe
    fr(d, x1 + 5,  y1 + 6,  x1 + 18, y1 + 7,  DT_SCRATCH)
    fr(d, x1 + 10, y1 + 3,  x1 + 11, y1 + 14, DT_SCRATCH)
    fr(d, x1 + 24, y1 + 10, x1 + 36, y1 + 11, DT_SCRATCH)


def draw_door_marker(d, x1, y1, x2, y2):
    """Door opening — slightly greenish floor with frame outline."""
    fr(d, x1, y1, x2, y2, DR_FLOOR)
    fr(d, x1,     y1,     x2,     y1 + 1, DR_BORDER)
    fr(d, x1,     y2 - 1, x2,     y2,     DR_BORDER)
    fr(d, x1,     y1,     x1 + 1, y2,     DR_BORDER)
    fr(d, x2 - 1, y1,     x2,     y2,     DR_BORDER)


def draw_melee_marker(d, x1, y1, x2, y2):
    """Subtle X marking on floor for melee weapon spawn point."""
    fr(d, x1, y1, x2, y2, ML_BASE)
    cx = (x1 + x2) // 2
    cy = (y1 + y2) // 2
    hs = min((x2 - x1) // 2 - 2, (y2 - y1) // 2 - 2)
    for i in range(-hs, hs + 1):
        # Top-left to bottom-right diagonal
        bx, by = cx + i, cy + i
        if x1 <= bx < x2 and y1 <= by < y2:
            d.point((bx, by), fill=ML_CROSS)
        # Top-right to bottom-left diagonal
        fx, fy = cx + i, cy - i
        if x1 <= fx < x2 and y1 <= fy < y2:
            d.point((fx, fy), fill=ML_CROSS)


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------

def main():
    os.makedirs(os.path.dirname(OUT_PNG), exist_ok=True)
    img = Image.new("RGBA", (W, H))
    d = ImageDraw.Draw(img)

    # 1. Floor tiles (full area)
    draw_floor(d)

    # 2. Detention tables at four corners (visual only, no collision)
    for gx, gy_ in [(-400, -260), (400, -260), (-400, 260), (400, 260)]:
        draw_detention_table(d, *grect(gx, gy_, 80, 50))

    # 3. Door opening markers (bottom wall)
    for gx, gy_ in [(-400, -300), (400, -300)]:
        draw_door_marker(d, *grect(gx, gy_, 40, 20))

    # 4. Melee weapon spawn markers
    for gx, gy_ in [(-280, 50), (280, -50)]:
        draw_melee_marker(d, *grect(gx, gy_, 40, 16))

    # 5. Cafeteria tables — 2 rows × 3 columns
    for gy_ in [-120, 120]:
        for gx in [-250, 0, 250]:
            draw_table(d, *grect(gx, gy_, 120, 24))

    # 6. Lunch counter (top area)
    draw_counter(d, *grect(0, 260, 400, 20))

    # 7. Pillars
    for gx in [-380, 380]:
        draw_pillar(d, *grect(gx, 0, 24, 80))

    # 8. Perimeter walls (drawn last — on top of any corner overlaps)
    #   top:    game y 304→320  → img y   0→16
    draw_wall_h(d, 0,   0,   W,   16)
    #   bottom: game y -320→-304 → img y 624→640
    draw_wall_h(d, 0,   624, W,   640)
    #   left:   game x -480→-448 → img x   0→32
    draw_wall_v(d, 0,   0,   32,  H)
    #   right:  game x  448→480  → img x 928→960
    draw_wall_v(d, 928, 0,   960, H)

    img.save(OUT_PNG)
    print(f"Saved {OUT_PNG}  ({W}×{H} RGBA)")


if __name__ == "__main__":
    main()
