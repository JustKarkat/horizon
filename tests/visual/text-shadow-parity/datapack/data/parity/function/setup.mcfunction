# Flat day scene with everything the phases place things on.
# Run from anywhere: /function parity:setup
time set noon
weather clear
gamerule doDaylightCycle false
gamerule doWeatherCycle false
kill @e[type=!minecraft:player]
setblock ~ ~-1 ~ minecraft:smooth_stone
tellraw @s {"text":"[parity] setup done — now run /function parity:phase1_sidebar", "color":"green"}
