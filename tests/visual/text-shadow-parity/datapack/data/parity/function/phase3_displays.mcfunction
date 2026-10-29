# Phase 3 — floating text displays (billboarded, in a row facing the player).
# /function parity:phase3_displays
summon minecraft:text_display ~1 ~3 ~ {billboard:"center",transformation:{translation:[0.0,0.0,0.0],left_rotation:[0.0,0.0,0.0,1.0],right_rotation:[0.0,0.0,0.0,1.0],scale:[1.0,1.0,1.0]},text:{"text":"int shadow","color":"white","shadow_color":-65536},background:0}
summon minecraft:text_display ~3 ~3 ~ {billboard:"center",text:{"text":"float shadow","color":"white","shadow_color":[0.0,1.0,0.0,1.0]},background:0}
summon minecraft:text_display ~5 ~3 ~ {billboard:"center",text:{"text":"no shadow","color":"white","shadow_color":0},background:0}
summon minecraft:text_display ~7 ~3 ~ {billboard:"center",text:{"text":"§6§l legacy + blue shadow","shadow_color":[0.0,0.0,1.0,1.0]},background:0}
summon minecraft:text_display ~9 ~3 ~ {billboard:"center",text:{"text":"default (25% shadow)","color":"gold"},background:0}
tellraw @s {"text":"[parity] five floating displays: red / green / none / legacy gold-bold with blue shadow / vanilla default. Stand back ~8 blocks and compare against vanilla 26.3.","color":"aqua"}
tellraw @s {"text":"[parity] done — run /function parity:phase4_items", "color":"green"}
