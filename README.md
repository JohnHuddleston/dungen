# DunGen

This is currently just a test repository for working out the basics of a roguelike.

Primarily following [the bracket_lib roguelike tutorial](https://bfnightly.bracketproductions.com/rustbook) but making custom implementations where desired.

**Current Efforts:**
* Map Generation
  * A set of basic map types and abstract tiles have been created, these will be used by MapBuilder to generate tilemaps.
  * Eventually abstract maps will be combined with themed palettes
    * Themes may eventually also mutate the input map even further, and/or be used to seed objects and entities (both hostile & non-hostile NPCs)
  * Abstract maps will be assembled from a variety of techniques including binary space partitioning, room tree generation and analysis, wave function collapse, and prefab assembly
  * Eventually a 'map' may encompass multiple levels, this will ultimately be controlled at the generation level but supported by functionality in the map generation module
  * Tile palettes: different kinds of maps should have different colors of cells (and maybe even represent cells with different characters) -> a normal overworld area should have green grass, but maybe grass outside of a necromancer's tower has died and should brown, while an arcane wizard tower's grass may have turned purple from some wild experiment!
    * Map drawing can also be made more complex to properly draw out walls with box-drawing characters (or use special tiles for the same purpose)
* Camera & Movement
  * Current setup has a static 'camera' meaning a heavily limited map space (80x50 characters), this will need to be replaced with a player-following camera.

**To-Do:**
* Component definitions
  * Currently only handling 'Renderable' and 'Position'
  * Need to determine what properties 'action takers' should have
    * Modeling bodies in some creative way would be cool, who wouldn't want 4 arms or the body of a horse?
* Mechanic enumeration & implementation
  * Alignment (not D&D style, more 'pantheon/domain' based)
  * Combat
    * Physical, magical... psychological???
* Game loop
  * First pass should stick to the standard single-run permadeath roguelike setup, but eventually I'll move to a full game loop in which a basic plot (procgen'd would be cool) utilizes a run system inside of a save-file persistence system where run rewards can be turned into power enhancement/character modifications.
* Graphics
  * Currently only ASCII/CCSID 437 characters, but I'd love to switch to tilesets
  * The renderer uses GLSL shaders, that's an opportunity to get absolutely wild with the output
  * The shader will likely only be applied to the full screen output as a sort of post-processing step, so if I want lighting I'll have to work that out on my own (lighting system that can modify the output color of a character/the lightness of a tile)
