# ideas that may or may not be implemented one day

- mutex on individual tile regions instead of whole world
	- i see a high potential for deadlocks
- >255 player world by only informing clients of nearby clients and swapping clients out as necessary
- similarly, >400 item drops per world, >200 npcs per world, >8000 chests, etc.
- run some sort of profiler to see what functions take the most time and optimize them
- optimize all the things! (mainly lookups)
- world generation
	- super large worlds
- skip the whole terraria physics simulation and do the bare minimum to get everything working
