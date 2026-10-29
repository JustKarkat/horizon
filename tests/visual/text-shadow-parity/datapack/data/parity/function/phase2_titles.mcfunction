# Phase 2 — action bar, title and subtitle with shadow colors.
# /function parity:phase2_titles
title @s actionbar {"text":"Action bar red shadow","color":"yellow","shadow_color":[1.0,0.0,0.0,1.0]}
title @s title {"text":"TITLE","color":"aqua","shadow_color":[0.0,0.4,1.0,1.0]}
title @s subtitle {"text":"subtitle half-alpha shadow","color":"white","shadow_color":[1.0,1.0,1.0,0.5]}
tellraw @s {"text":"[parity] action bar + title: each text shows its own shadow colour (red / blue / half-alpha white), not the default 25% darkening.","color":"aqua"}
tellraw @s {"text":"[parity] chat line WITH shadow_color for comparison","color":"green","shadow_color":[1.0,0.0,1.0,1.0]}
tellraw @s {"text":"[parity] done — run /function parity:phase3_displays", "color":"green"}
