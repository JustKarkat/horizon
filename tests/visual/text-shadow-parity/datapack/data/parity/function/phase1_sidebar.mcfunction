# Phase 1 — the sidebar scoreboard with a shadow-colored title (issue #3531's repro).
# /function parity:phase1_sidebar
scoreboard objectives add parity_shadow dummy [{"text":"Shadow","bold":true,"color":"gold","shadow_color":[1.0,0.0,0.0,1.0]}]
scoreboard objectives add parity_noshadow dummy {"text":"NoShadow","color":"white","shadow_color":0}
scoreboard objectives add parity_plain dummy {"text":"Plain","color":"gold"}
scoreboard players set "Alpha" parity_shadow 30
scoreboard players set "Bravo" parity_shadow 22
scoreboard players set "Charlie" parity_shadow 14
scoreboard players set "Alpha" parity_noshadow 8
scoreboard players set "Alpha" parity_plain 5
team add parity_red
team modify parity_red color red
team modify parity_red suffix {"text":" ▲","shadow_color":[0.0,1.0,0.0,1.0]}
team join parity_red Bravo
scoreboard objectives setdisplay sidebar parity_shadow
tellraw @s {"text":"[parity] sidebar: gold 'Shadow' title must show a RED shadow; Bravo's name is red with a GREEN-shadowed suffix.","color":"aqua"}
tellraw @s {"text":"[parity] switch titles with: /scoreboard objectives setdisplay sidebar parity_noshadow  (shadow must vanish entirely)  |  parity_plain  (default dark-gold shadow)","color":"gray"}
