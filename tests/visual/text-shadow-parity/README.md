# Text `shadow_color` / styled item names — visual parity scene

For [issue #3531](https://github.com/horizon-rust/horizon/issues/3531): components with a
`shadow_color` did not render their shadow in the HUD (sidebar, titles, action bar, chat),
and item name components combining `bold` and a `color` lost their styling.

Run the same steps in **Horizon** and in **vanilla 26.3** and compare side by side. Everything
below must look identical.

## Setup (both games)

1. Create a flat world (creative, cheats on).
2. Copy `datapack/` into the world's `datapacks/` folder, then `/reload`.
3. `/function parity:setup`

## What to compare

### Phase 1 — sidebar (`/function parity:phase1_sidebar`)

- The header reads **Shadow** in bold gold **with a red shadow** (a `shadow_color` shows a
  shadow even though the sidebar draws without the default one — vanilla
  `Font.PreparedTextBuilder.getShadowColor` honours the style's own colour regardless).
- Rows `Alpha` / `Bravo` / `Charlie` with their scores. `Bravo` is red (its team colour) with
  a green-shadowed `▲` suffix.
- `/scoreboard objectives setdisplay sidebar parity_noshadow`: the title shows **no shadow at
  all** (`shadow_color: 0` is `NO_SHADOW`, it suppresses the default one too).
- `/scoreboard objectives setdisplay sidebar parity_plain`: the plain gold title keeps
  vanilla's default darkened shadow.

### Phase 2 — titles and chat (`/function parity:phase2_titles`)

- Action bar `Action bar red shadow` (yellow text, red shadow), title `TITLE` (aqua text,
  blue shadow), subtitle with a half-alpha white shadow.
- The chat line with a magenta `shadow_color`.

### Phase 3 — floating text displays (`/function parity:phase3_displays`)

Five billboarded displays in a row: red shadow, green shadow, **no** shadow, a legacy
`§6§l` name with a blue shadow (legacy codes keep the style's `shadow_color`), and a plain
gold one with vanilla's default 25% shadow. Step back until all five are in view.

### Phase 4 — item names (`/function parity:phase4_items`)

Four sticks land in the hotbar; hover each one and switch between slots for the name popup:

1. `Bold red item_name` — bold **and** red in the tooltip, the hotbar popup and the inventory.
2. `Bold aqua custom_name` — bold aqua, **not** italic despite being a custom name
   (`getStyledHoverName` adds the italic; the component's own `italic:false` wins).
3. `§6§lLegacy name` — legacy codes styled.
4. `Shadowed name` — a red shadow behind the tooltip's first line and the popup.
5. The plain diamond sword keeps the vanilla look: white, not italic.

## Automated coverage

The same behaviour is pinned by `tools/test_text_display_render_parity.py` (its cases include
`shadow_color` int, float-list, `NO_SHADOW`, legacy and translucent forms) against the real
vanilla `TextDisplayRenderer`, plus unit tests in `horizon-client-core` and `horizon-items`.

## Cleanup

`/function parity:clear`
