# Sprite Sheet Generation Prompts

Use these prompts with Claude's web app image generation to create all sprite sheets for the Cafeteria Food Fight game. After generating each image, save it to the specified path under `assets/sprites/`.

**Style Guide for ALL prompts:** Whimsical, chunky pixel art. School cafeteria theme. Characters have oversized heads, stubby limbs, and exaggerated expressions. Think "cartoon meets retro SNES." Transparent backgrounds (PNG). Bright, saturated colors with dark pixel outlines. Each frame in the grid should be clearly separated.

---

## 1. PLAYER SPRITE SHEETS (4 separate images)

Each player sprite sheet is a **256x128 pixel** image containing a **8-column x 4-row grid** of **32x32 pixel frames**. Transparent background.

### Prompt for Player 1 (Blue) — save as `assets/sprites/players/player_blue.png`

```
Create a pixel art sprite sheet for a 2D top-down school kid character. The image should be exactly 256x128 pixels, arranged as an 8-column by 4-row grid of 32x32 pixel cells. Transparent background. Whimsical, chunky pixel art style with dark outlines — like a cute SNES-era character.

The character is a chubby kid wearing a BLUE school uniform/hoodie with a backpack. Oversized round head, big expressive eyes, stubby limbs.

Row 1 (top) — WALK DOWN (facing camera): 4 walk-cycle frames in columns 1-4, then 2 idle frames (slight breathing/bobbing) in columns 5-6, then 2 "stunned" frames (dizzy stars, dazed expression) in columns 7-8.

Row 2 — WALK UP (facing away): 4 walk-cycle frames in columns 1-4, then 2 throw-windup frames (arm pulled back with food) in columns 5-6, then 2 throw-release frames (arm forward, releasing food) in columns 7-8.

Row 3 — WALK LEFT: 4 walk-cycle frames in columns 1-4, then 2 "hit reaction" frames (knocked back, ouch face) in columns 5-6, then 2 "picking up item" frames (bending down) in columns 7-8.

Row 4 — WALK RIGHT: 4 walk-cycle frames in columns 1-4, then 2 "holding food" idle frames (food visible in hand) in columns 5-6, then 2 "victory celebration" frames (jumping, arms up) in columns 7-8.

Make it charming and goofy — this is a cafeteria food fight game. The blue should be a bright medium blue (roughly #3399FF).
```

### Prompt for Player 2 (Red) — save as `assets/sprites/players/player_red.png`

```
Create a pixel art sprite sheet for a 2D top-down school kid character. The image should be exactly 256x128 pixels, arranged as an 8-column by 4-row grid of 32x32 pixel cells. Transparent background. Whimsical, chunky pixel art style with dark outlines — like a cute SNES-era character.

The character is a scrappy kid wearing a RED jersey/t-shirt, slightly different build than blue player — maybe spikier hair, more aggressive stance. Oversized round head, big expressive eyes, stubby limbs.

Row 1 (top) — WALK DOWN (facing camera): 4 walk-cycle frames in columns 1-4, then 2 idle frames (slight breathing/bobbing) in columns 5-6, then 2 "stunned" frames (dizzy stars, dazed expression) in columns 7-8.

Row 2 — WALK UP (facing away): 4 walk-cycle frames in columns 1-4, then 2 throw-windup frames (arm pulled back with food) in columns 5-6, then 2 throw-release frames (arm forward, releasing food) in columns 7-8.

Row 3 — WALK LEFT: 4 walk-cycle frames in columns 1-4, then 2 "hit reaction" frames (knocked back, ouch face) in columns 5-6, then 2 "picking up item" frames (bending down) in columns 7-8.

Row 4 — WALK RIGHT: 4 walk-cycle frames in columns 1-4, then 2 "holding food" idle frames (food visible in hand) in columns 5-6, then 2 "victory celebration" frames (jumping, arms up) in columns 7-8.

Make it charming and goofy — this is a cafeteria food fight game. The red should be a bright warm red (roughly #FF4D4D).
```

### Prompt for Player 3 (Green) — save as `assets/sprites/players/player_green.png`

```
Create a pixel art sprite sheet for a 2D top-down school kid character. The image should be exactly 256x128 pixels, arranged as an 8-column by 4-row grid of 32x32 pixel cells. Transparent background. Whimsical, chunky pixel art style with dark outlines — like a cute SNES-era character.

The character is a nerdy kid wearing a GREEN sweater vest, glasses, maybe a little shorter and rounder. Oversized round head, big expressive eyes behind glasses, stubby limbs.

Row 1 (top) — WALK DOWN (facing camera): 4 walk-cycle frames in columns 1-4, then 2 idle frames (slight breathing/bobbing) in columns 5-6, then 2 "stunned" frames (dizzy stars, dazed expression) in columns 7-8.

Row 2 — WALK UP (facing away): 4 walk-cycle frames in columns 1-4, then 2 throw-windup frames (arm pulled back with food) in columns 5-6, then 2 throw-release frames (arm forward, releasing food) in columns 7-8.

Row 3 — WALK LEFT: 4 walk-cycle frames in columns 1-4, then 2 "hit reaction" frames (knocked back, ouch face) in columns 5-6, then 2 "picking up item" frames (bending down) in columns 7-8.

Row 4 — WALK RIGHT: 4 walk-cycle frames in columns 1-4, then 2 "holding food" idle frames (food visible in hand) in columns 5-6, then 2 "victory celebration" frames (jumping, arms up) in columns 7-8.

Make it charming and goofy — this is a cafeteria food fight game. The green should be a bright lime green (roughly #4DFF4D).
```

### Prompt for Player 4 (Yellow) — save as `assets/sprites/players/player_yellow.png`

```
Create a pixel art sprite sheet for a 2D top-down school kid character. The image should be exactly 256x128 pixels, arranged as an 8-column by 4-row grid of 32x32 pixel cells. Transparent background. Whimsical, chunky pixel art style with dark outlines — like a cute SNES-era character.

The character is a tall lanky kid wearing a YELLOW athletic jacket, maybe a headband or cap. Oversized round head, big expressive eyes, long stubby limbs.

Row 1 (top) — WALK DOWN (facing camera): 4 walk-cycle frames in columns 1-4, then 2 idle frames (slight breathing/bobbing) in columns 5-6, then 2 "stunned" frames (dizzy stars, dazed expression) in columns 7-8.

Row 2 — WALK UP (facing away): 4 walk-cycle frames in columns 1-4, then 2 throw-windup frames (arm pulled back with food) in columns 5-6, then 2 throw-release frames (arm forward, releasing food) in columns 7-8.

Row 3 — WALK LEFT: 4 walk-cycle frames in columns 1-4, then 2 "hit reaction" frames (knocked back, ouch face) in columns 5-6, then 2 "picking up item" frames (bending down) in columns 7-8.

Row 4 — WALK RIGHT: 4 walk-cycle frames in columns 1-4, then 2 "holding food" idle frames (food visible in hand) in columns 5-6, then 2 "victory celebration" frames (jumping, arms up) in columns 7-8.

Make it charming and goofy — this is a cafeteria food fight game. The yellow should be a bright golden yellow (roughly #FFE633).
```

---

## 2. NPC SPRITE SHEETS (3 separate images)

### Teacher — save as `assets/sprites/npcs/teacher.png`

Image: **224x112 pixels** — 7 columns x 4 rows of **32x32** cells. Transparent background.

```
Create a pixel art sprite sheet for a 2D top-down school TEACHER NPC character. The image should be exactly 224x112 pixels, arranged as a 7-column by 4-row grid (but only the first 3 rows used) of 32x32 pixel cells. Transparent background. Whimsical, chunky pixel art style with dark outlines.

The teacher is a frumpy, slightly annoyed middle-aged woman with a TAN/KHAKI outfit — think corduroy blazer, sensible shoes, reading glasses perched on nose, hair in a bun. Medium build, about 28x28 pixels of actual character within the 32x32 cell.

Row 1 — PATROLLING (normal state, tan/khaki colors): walk-down (2 frames), walk-up (2 frames), walk-left (1 frame), walk-right (1 frame), idle/looking around (1 frame).

Row 2 — SUSPICIOUS (yellow-tinted, alert expression, one eyebrow raised): same 7 poses as row 1, but the teacher looks more alert and slightly yellow-tinted. Maybe squinting eyes, hand on hip.

Row 3 — CHASING (red-tinted, angry expression, running): same 7 poses but running animation, face is angry/red, arms pumping. The teacher means business.

Row 4 — EXTRAS: catching pose (grabbing gesture, 2 frames), pointing accusingly (2 frames), "I see you!" alert exclamation (1 frame), returning to patrol (relaxing, 2 frames).

She should look like a classic stern-but-caring teacher who does NOT approve of food fights. Detection angle is narrow (60 degrees) — she's focused but not omniscient.
```

### Principal — save as `assets/sprites/npcs/principal.png`

Image: **224x128 pixels** — 7 columns x 4 rows of **32x32** cells. Transparent background.

```
Create a pixel art sprite sheet for a 2D top-down school PRINCIPAL NPC character. The image should be exactly 224x128 pixels, arranged as a 7-column by 4-row grid of 32x32 pixel cells. Transparent background. Whimsical, chunky pixel art style with dark outlines.

The principal is a large, imposing, bald man in a DARK BLUE suit with a red tie. Big round body, tiny legs, stern expression, maybe a mustache. He's SLOW but has a wide field of view (90 degrees). About 32x32 pixels filling the cell — he's the biggest NPC.

Row 1 — PATROLLING (normal state, dark blue suit): walk-down (2 frames), walk-up (2 frames), walk-left (1 frame), walk-right (1 frame), idle/arms-crossed (1 frame).

Row 2 — SUSPICIOUS (yellow-tinted, narrowed eyes): same 7 poses but looking suspicious, maybe adjusting glasses, turning head. Yellow highlight/tint.

Row 3 — CHASING (red-tinted, furious): same 7 poses but "power walking" — fists clenched, face beet red, tie flapping. He's slow but terrifying.

Row 4 — EXTRAS: catching pose (firm grip, 2 frames), "DETENTION!" pose pointing (2 frames), discovery alert (1 frame), returning walk (straightening tie, 2 frames).

He should be intimidating in a comedic way. He gives the longest stun penalty (5 seconds) when he catches you. Make him look like the final boss of school authority.
```

### Lunch Lady — save as `assets/sprites/npcs/lunch_lady.png`

Image: **192x96 pixels** — 6 columns x 3 rows of **32x32** cells. Transparent background.

```
Create a pixel art sprite sheet for a 2D top-down school LUNCH LADY NPC character. The image should be exactly 192x96 pixels, arranged as a 6-column by 3-row grid of 32x32 pixel cells. Transparent background. Whimsical, chunky pixel art style with dark outlines.

The lunch lady is a stout, stern woman in a PINK apron and hairnet, standing behind the lunch counter. She holds a big serving ladle as a weapon. She does NOT move — she's stationary but has a wide 180-degree detection arc. About 28x28 pixels of character within the 32x32 cell.

Row 1 — NORMAL (pink apron, standing at counter): idle with ladle (2 frames of subtle animation — maybe stirring), looking left (1 frame), looking right (1 frame), looking forward sternly (1 frame), ladle raised threateningly (1 frame).

Row 2 — SUSPICIOUS (yellow-tinted, squinting): same 6 poses but alert — squinting eyes, ladle gripped tighter, leaning forward suspiciously. She sees EVERYTHING in her half-circle.

Row 3 — ATTACKING/CHASING (red-tinted, enraged): ladle swing left (2 frames), ladle swing right (2 frames), "caught you!" (grabbing with ladle, 2 frames). She can't move but she can reach you if you get close.

She should look like a cafeteria lunch lady who has SEEN THINGS and will not tolerate food being wasted as ammunition. The ladle is her scepter of power.
```

---

## 3. FOOD ITEMS SPRITE SHEET (single image)

Save as `assets/sprites/food/food_items.png`

Image: **128x128 pixels** — 8 columns x 8 rows of **16x16** cells. Transparent background.

```
Create a pixel art sprite sheet of CAFETERIA FOOD ITEMS for a 2D food fight game. The image should be exactly 128x128 pixels, arranged as an 8-column by 8-row grid of 16x16 pixel cells. Transparent background. Whimsical, chunky pixel art style — food should look cartoonishly appetizing (or disgusting).

Each food type gets ONE ROW (8 rows total). Within each row:
- Column 1: food item sitting on ground (resting, static)
- Column 2: food item slightly tilted (pickup sparkle)
- Columns 3-4: food spinning in flight (2 rotation frames)
- Column 5: food at peak of arc (for arc-trajectory foods — slightly stretched upward)
- Column 6: food impact frame (slightly squished/deforming on hit)
- Column 7: food splatter (partially exploded, bits flying)
- Column 8: food remains/splat on ground (aftermath stain)

Row 1 — PIZZA SLICE (14x14 within cell): Classic triangular pizza slice, orange-yellow cheese, red sauce visible, golden crust edge. Gooey and delicious looking.

Row 2 — MEATBALL (10x10 within cell, centered): Round brown meatball, slightly lumpy texture, maybe a tiny bit of sauce. Small but hefty-looking. This one arcs through the air.

Row 3 — JELLO (12x12 within cell): Bright green jiggly jello cube, translucent-looking with a highlight. Wobbly animation in flight. This one BOUNCES — show the squish on impact frames.

Row 4 — GRAPE (6x6 within cell, centered, tiny): Small purple grape, round, with a tiny stem. The smallest projectile — fast and hard to see. Impact shows purple juice splash.

Row 5 — MILK CARTON (12x16 within cell, taller): White milk carton with red/blue school milk branding, slightly dented. In flight it tumbles end over end. Splatter is white milk explosion.

Row 6 — SPAGHETTI (14x8 within cell, wide): Tangled pile of golden spaghetti noodles with red sauce. Wide and stringy. In flight, noodles trail behind. Splat is a messy noodle puddle.

Row 7 — BANANA PEEL (12x8 within cell): Classic comedy yellow banana peel, slightly curved. In flight it spins like a boomerang. Splat shows it flattened on ground (slip hazard!).

Row 8 — MYSTERY MEAT (16x16, fills cell): Suspicious gray-brown lump of unidentifiable cafeteria meat. Slightly greenish tinge. The heaviest, most damaging food item. Impact is a gross splat. Make it look questionable and slightly menacing.

Make all the food look whimsical and cartoony — exaggerated shapes, visible "action lines" during flight frames. This is a silly food fight, not fine dining.
```

---

## 4. LAUNCHERS SPRITE SHEET (single image)

Save as `assets/sprites/launchers/launchers.png`

Image: **160x80 pixels** — 5 columns x 4 rows of **32x20** cells (wider cells for launcher shapes). Transparent background.

Actually, let's use uniform 32x32 cells for simplicity with Bevy's TextureAtlas:

Image: **160x160 pixels** — 5 columns x 5 rows of **32x32** cells. Transparent background.

```
Create a pixel art sprite sheet of FOOD FIGHT WEAPON LAUNCHERS for a 2D school cafeteria game. The image should be exactly 160x160 pixels, arranged as a 5-column by 5-row grid of 32x32 pixel cells. Transparent background. Whimsical, chunky pixel art style.

Each launcher type gets ONE ROW (5 rows). Within each row:
- Column 1: launcher on ground (pickup item, with subtle glow/sparkle)
- Column 2: launcher equipped/held (as seen when a player has it)
- Column 3: launcher firing frame 1 (recoil/windup)
- Column 4: launcher firing frame 2 (projectile leaving)
- Column 5: launcher empty/depleted (grayed out, broken look)

Row 1 — SLINGSHOT (16x16 actual, brown #996633): Classic Y-shaped wooden slingshot with rubber band. Balanced weapon — 10 shots. When firing, rubber band stretches back then snaps forward. Simple and iconic.

Row 2 — KETCHUP GUN (18x12 actual, red #E61A1A): A squeeze ketchup bottle modified into a rapid-fire gun. Bright red with a white cap/nozzle. When firing, ketchup squirts out rapidly. 40 shots — pew pew pew! Show ketchup droplets on fire frames.

Row 3 — SPORK LAUNCHER (14x14 actual, silver-blue #B3B3CC): A mechanical spork-flinging device — like a miniature crossbow made from sporks and rubber bands. Silver-blue metallic look. 15 shots, very fast projectiles. Fire frames show spork being launched.

Row 4 — LUNCH TRAY CATAPULT (20x16 actual, gray #808066): A cafeteria tray rigged as a catapult/trebuchet. Gray plastic tray with a bent spoon as the arm. This one CHARGES UP — column 3 shows start of charge, column 4 shows full charge (glowing, vibrating). 5 powerful shots. Make it look over-engineered and ridiculous.

Row 5 — STRAW BLOWGUN (20x6 actual, yellow-tan #E6E680): A long drinking straw used as a blowgun. Simple, long, narrow. 50 tiny shots — very rapid fire but weak. Fire frames show cheeks puffing and tiny projectile leaving the straw. Make it look hilariously inefficient but fun.

All launchers should look like they were improvised from cafeteria supplies — duct tape, rubber bands, bent utensils. Scrappy and creative, like a kid built them during study hall.
```

---

## 5. EFFECTS SPRITE SHEET (single image)

Save as `assets/sprites/effects/effects.png`

Image: **192x96 pixels** — 6 columns x 3 rows of **32x32** cells. Transparent background.

```
Create a pixel art sprite sheet of IMPACT EFFECTS AND SPLATS for a 2D food fight game. The image should be exactly 192x96 pixels, arranged as a 6-column by 3-row grid of 32x32 pixel cells. Transparent background. Whimsical, chunky pixel art style with bright colors.

Row 1 — HIT FLASH EFFECTS (when food hits a player):
- Col 1: Small white star/flash burst (start of impact)
- Col 2: Medium expanding impact ring with food bits
- Col 3: Large starburst flash with "POW" comic-style effect
- Col 4: Impact dissipating, bits flying outward
- Col 5: Tiny sparkles remaining
- Col 6: Empty/fully faded (for animation end frame)

Row 2 — GROUND SPLAT DECALS (food remains on the ground, various colors):
- Col 1: Red sauce splat (pizza/spaghetti/meatball) — circular splatter pattern
- Col 2: Green splat (jello) — wobbly/jiggly shape
- Col 3: Purple splat (grape) — small juice stain
- Col 4: White splat (milk) — puddle shape
- Col 5: Yellow splat (banana/general) — smear shape
- Col 6: Brown/gray splat (mystery meat) — gross lumpy splatter

Row 3 — MISC EFFECTS:
- Col 1: Pickup sparkle (3-pointed star, yellow)
- Col 2: Question mark (NPC suspicious indicator)
- Col 3: Exclamation mark (NPC alert/chase indicator, red)
- Col 4: Dizzy stars (player stunned indicator, rotating)
- Col 5: Speed lines (for NPC chasing)
- Col 6: Shield/immunity bubble (for future power-up)

Make effects punchy and readable at small sizes. Comic-book style — bold outlines, bright colors, exaggerated action. The splats should look genuinely messy and satisfying.
```

---

## 6. UI ELEMENTS SPRITE SHEET (single image)

Save as `assets/sprites/ui/ui_elements.png`

Image: **256x128 pixels** — 8 columns x 4 rows of **32x32** cells. Transparent background.

```
Create a pixel art sprite sheet of UI ELEMENTS for a 2D school cafeteria food fight game. The image should be exactly 256x128 pixels, arranged as an 8-column by 4-row grid of 32x32 pixel cells. Transparent background. Whimsical pixel art style matching a school/cafeteria theme.

Row 1 — FOOD ICONS (for inventory/HUD, each item as a clean icon):
- Col 1: Pizza slice icon
- Col 2: Meatball icon
- Col 3: Jello cube icon
- Col 4: Grape icon
- Col 5: Milk carton icon
- Col 6: Spaghetti icon
- Col 7: Banana peel icon
- Col 8: Mystery meat icon

Row 2 — LAUNCHER ICONS (for HUD when equipped):
- Col 1: Slingshot icon
- Col 2: Ketchup gun icon
- Col 3: Spork launcher icon
- Col 4: Lunch tray catapult icon
- Col 5: Straw blowgun icon
- Col 6: Ammo counter frame (empty box with border)
- Col 7: Cooldown indicator (circular pie timer, half filled)
- Col 8: Charge meter frame (vertical bar outline)

Row 3 — STATUS ICONS:
- Col 1: Heart (health, red)
- Col 2: Heart empty (lost health, dark/gray)
- Col 3: Star (score point, gold)
- Col 4: Trophy (winner, gold)
- Col 5: Skull (KO/eliminated, comical)
- Col 6: Clock (timer, for stunned duration)
- Col 7: Eye (detection/suspicious, for NPC awareness)
- Col 8: Boot (speed, for potential power-ups)

Row 4 — BUTTON PROMPTS / MISC:
- Col 1: "A" button prompt (green circle)
- Col 2: "B" button prompt (red circle)
- Col 3: "X" button prompt (blue circle)
- Col 4: "Y" button prompt (yellow circle)
- Col 5: "RT" trigger prompt
- Col 6: Arrow cursor/pointer
- Col 7: Checkmark (ready indicator, green)
- Col 8: X mark (not ready, red)

School cafeteria aesthetic — maybe elements look like they're drawn on notebook paper or a chalkboard. Keep icons clear and readable at 32x32.
```

---

## 7. MAP TILES (optional, for future tilemap)

Save as `assets/sprites/map/tiles.png`

Image: **128x128 pixels** — 8 columns x 8 rows of **16x16** cells. No transparency (floor tiles).

```
Create a pixel art TILESET for a school cafeteria floor and furniture in a 2D top-down game. The image should be exactly 128x128 pixels, arranged as an 8-column by 8-row grid of 16x16 pixel cells. Whimsical pixel art style.

Row 1 — FLOOR TILES:
- Cols 1-4: Cafeteria floor variations (checkered linoleum in cream/beige, slight dirty variations for visual interest). These tile seamlessly.
- Cols 5-8: Kitchen floor variations (slightly different color — gray tile, with grout lines).

Row 2 — WALL TILES:
- Cols 1-2: Horizontal wall (cinder block texture, gray-brown)
- Cols 3-4: Vertical wall (same texture, rotated)
- Col 5: Wall corner (top-left)
- Col 6: Wall corner (top-right)
- Col 7: Wall corner (bottom-left)
- Col 8: Wall corner (bottom-right)

Row 3 — TABLE TILES (cafeteria long tables, tan/beige):
- Col 1: Table top-left corner
- Col 2: Table top-right corner
- Col 3: Table bottom-left corner
- Col 4: Table bottom-right corner
- Col 5: Table horizontal edge (top)
- Col 6: Table horizontal edge (bottom)
- Col 7: Table vertical edge (left)
- Col 8: Table center fill

Row 4 — COUNTER TILES (lunch counter, beige/steel):
- Cols 1-4: Counter top segments (glass sneeze guard visible, steel surface)
- Cols 5-8: Counter front segments (wooden front panel variations)

Row 5 — FURNITURE & OBJECTS:
- Col 1: Chair (top-down, small)
- Col 2: Trash can (cylindrical, gray)
- Col 3: Tray return window (dark opening)
- Col 4: Vending machine (top-down, rectangular)
- Col 5: Pillar/column (round, gray)
- Col 6: Door (brown, with window)
- Col 7: Food serving station (steam tray)
- Col 8: Cash register area

Row 6 — DECORATIVE:
- Col 1: Bulletin board
- Col 2: Clock on wall
- Col 3: "TODAY'S MENU" sign
- Col 4: Exit sign (green/red)
- Col 5: Wet floor sign (yellow)
- Col 6: Spilled food stain (pre-placed decor)
- Col 7: Backpack on ground
- Col 8: Lunch tray abandoned

Rows 7-8: Reserved/blank for expansion.

Make it look like a classic American school cafeteria — institutional but with personality. Linoleum floors, cinder block walls, plastic furniture.
```

---

## Generation Tips

1. **Generate one image at a time** — Claude handles single focused images better than batch requests.

2. **If the grid alignment is off**, try adding: "Make sure each sprite is precisely contained within its grid cell. Draw thin 1px guide lines between cells if needed, I can remove them later."

3. **For consistency**, generate all players in one session so they share the same art style. Same for NPCs.

4. **If images come out too large**, you can ask Claude to scale them down, or resize them in an image editor. The game uses `ImagePlugin::default_nearest()` so nearest-neighbor scaling preserves pixel art crispness.

5. **Alpha transparency** is important — ask for "transparent background, PNG format" in every prompt. If Claude generates with a background, you may need to remove it manually.

6. **After saving**, rename files exactly as specified and place in the paths listed. The metadata JSON files (in the same directories) tell the game engine how to read the sprite sheets.
