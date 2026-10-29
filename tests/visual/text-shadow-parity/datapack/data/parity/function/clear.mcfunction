# Reset everything the phases created.
# /function parity:clear
scoreboard objectives setdisplay sidebar
scoreboard objectives remove parity_shadow
scoreboard objectives remove parity_noshadow
scoreboard objectives remove parity_plain
team remove parity_red
title @s clear
kill @e[type=minecraft:text_display]
clear @s
tellraw @s {"text":"[parity] cleared", "color":"green"}
