# Phase 4 — styled item names: tooltips, the hotbar name popup and the inventory.
# /function parity:phase4_items
give @s minecraft:stick[item_name={"text":"Bold red item_name","bold":true,"color":"red"}] 1
give @s minecraft:stick[custom_name={"text":"Bold aqua custom_name","bold":true,"color":"aqua","italic":false}] 1
give @s minecraft:stick[custom_name={"text":"§6§lLegacy name"}] 1
give @s minecraft:stick[item_name={"text":"Shadowed name","color":"white","shadow_color":[1.0,0.0,0.0,1.0]}] 1
give @s minecraft:diamond_sword 1
loot give @s loot minecraft:blocks/grass_block
tellraw @s {"text":"[parity] four sticks in the hotbar. Hover each for the tooltip, then switch slots 1-4 for the name popup: bold+red, bold+aqua, legacy gold-bold, and a red-shadowed name.","color":"aqua"}
tellraw @s {"text":"[parity] The unstyled diamond sword (slot 5) keeps its vanilla look: white name, no italic.","color":"gray"}
tellraw @s {"text":"[parity] done — compare with /function parity:clear to reset", "color":"green"}
