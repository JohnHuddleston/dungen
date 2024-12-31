# DunGen

This is currently just a test repository for working out the basics of a roguelike.

Primarily following [the bracket_lib roguelike tutorial](https://bfnightly.bracketproductions.com/rustbook) but making custom implementations where desired.

#### Immediate To-Do's:
1. Refactor level building to be fault tolerant in cases where procgen fails
    1. Could be handled internally to the generation functions, clone out the tilemap and only replace it on a success?
    2. *!! This is going to be a large refactoring effort that I'll handle while formalizing map generation !!*
2. Monster spawning & combat
    1. Something primitive for now, just basic attacks then I'll build up weapons (maybe even ranged)
    2. Worth thinking about how to organize/model attacks between weapons (melee and ranged) and magic (definitely ranged, melee too?)
3. CAMERA - 80x50 is annoyingly small, and with the odd-dimension restriction on at least the Hauberk style generation it's an even gnarlier 79x49!  Having a camera set up lets me make map of arbitrarily large sizes (within reason, generation will obviously take longer, though 5ms vs 20ms for the generation stage doesn't really matter)


**"Top-Line" Efforts:**
* Map Generation
  * A set of basic map types and abstract tiles have been created, these will be used by MapBuilder to generate tilemaps.
  * Eventually abstract maps will be combined with themed palettes
    * Themes may eventually also mutate the input map even further, and/or be used to seed objects and entities (both hostile & non-hostile NPCs)
  * Abstract maps will be assembled from a variety of techniques including binary space partitioning, room tree generation and analysis, wave function collapse, and prefab assembly
  * Eventually a 'map' may encompass multiple levels, this will ultimately be controlled at the generation level but supported by functionality in the map generation module
  * Tile palettes: different kinds of maps should have different colors of cells (and maybe even represent cells with different characters) -> a normal overworld area should have green grass, but maybe grass outside of a necromancer's tower has died and should be brown, while an arcane wizard tower's grass may have turned purple from some wild experiment!
    * Map drawing can also be made more complex to properly draw out walls with box-drawing characters (or use special tiles for the same purpose)
    * Current palette implementation is very limited, but will be extended.  Storing const [[RGBA]] is gross, too, I should load those from hex values in a RON file.
* Camera & Movement
  * Current setup has a static 'camera' meaning a heavily limited map space (80x50 characters), this will need to be replaced with a player-following camera.

**To-Do:**
* Component definitions
  * Currently only handling 'Renderable' and 'Position'
  * Need to determine what properties 'action takers' should have
    * Modeling bodies in some creative way would be cool, who wouldn't want 4 arms or the body of a horse?
        * I'm definitely stealing CoQ's mutation thing
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

**Completed:**
1. ~~Move palette definitions to a config file (RON looks nice)~~ ALL PARTS DONE!
    1. USE HEXSTRINGS, bracket_lib defining RGBA as (f32, f32, f32, f32) with elements 0.0..=1.0 is, frankly, insane, and it looks AWFUL when cargo formatted.
    2. The palette selected should be added to the World as a resource so that all parts of the game can pull from it.
    3. Move to 16-color palettes ASAP so I'm not limited to FG, BG, and 2 accents for everything.
2. ~~MOVEMENT - I need to get something simple in there, just move and get blocked by walls, this is important for the next item...~~ DONE!
