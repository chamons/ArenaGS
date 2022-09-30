# Get UI Ready
- Targeting

# Start of Gameplay 
- Taking turns (Double hitting move before animation complete glitches)
- Movement
- Basic Skills
- Save/Load
- AI
- See https://github.com/chamons/ArenaGS/commits/master?after=0d3c8d4ebc818198b21a8c99dc853286cc16b7c2+139&branch=master&qualified_name=refs%2Fheads%2Fmaster for more bits 

# Minor Features
- Projectiles fade over a few frames instead of disappearing
- Character Overlay for status effects

# Minor Refactorings
- Look into moving AnimationState somewhere else
- Split Animation into specific states so we can filter in the ECS

# Minor Bugs
- Idle animation plays during movement
- Animations speed up when alt+tabed away for awhile
- Window shrinks to tiny size when closing laptop lid then restore