# Coloured hearts extra: proof screenshots

Captured from this branch (merged up to main) on 2026-10-29 with
`horizon-client --offscreen --test-terrain` and `horizon-ctl screenshot`,
GUI scale 3, 1280x720.  The heart row is above the hotbar.

1-4: `coloredHearts` off — vanilla rows, unchanged (3 rows at 60 HP, one
absorption row), and the poison, wither and frozen heart sprites.
5-6: `coloredHearts` on — one row of ten: the current row's colour (gold for
row 3), then the absorption row in `absorptionColors[0]`.
7-8: a resource pack that replaces `hud/heart/full.png` loads (7) and the
extra recolours the pack's own sprite to the row colour (8).
