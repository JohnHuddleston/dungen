use crate::map_gen::abstract_tiles::AbstractMapTiles;
use crate::Palette;
use bracket_lib::prelude::*;
use glam::UVec2;

// These will eventually become 'recipes' that determine number of levels, what generation should
// be done on each level, etc.  May need to pair with difficulty rating, or that can be handled on
// the population side.
#[allow(unused)]
pub enum MapType {
    Base,
    Dungeon,
    Cave,
    Overworld,
    ForestCamp,
    Tower,
    Labyrinth,
}

#[allow(unused)]
pub struct TileMap {
    pub tilemap: Vec<AbstractMapTiles>,
    pub palette: Palette,
    pub dimensions: UVec2,
    pub player_spawn: UVec2,
    pub exits: Vec<UVec2>,
}

#[allow(unused)]
impl TileMap {
    pub fn new(palette: Palette) -> Self {
        TileMap {
            tilemap: vec![AbstractMapTiles::WALL; 80 * 50],
            palette,
            dimensions: UVec2 { x: 79, y: 49 },
            player_spawn: UVec2 { x: 0, y: 0 },
            exits: Vec::new(),
        }
    }

    pub fn draw(&self, ctx: &mut BTerm) {
        for x in 0..self.dimensions.x {
            for y in 0..self.dimensions.y {
                let glyph: u16;
                match self.tilemap[y as usize * self.dimensions.x as usize + x as usize] {
                    // this mapping should probably be done automatically, we can do that in the
                    // enum or make a mapping function, especially for the move to tilesets.  Come to think of it, return type for a
                    // glyph and a tile index are probably the same (u16)
                    AbstractMapTiles::WALL => glyph = to_cp437('█'),
                    AbstractMapTiles::PIT => glyph = to_cp437('_'),
                    AbstractMapTiles::ABYSS => glyph = to_cp437(' '),
                    AbstractMapTiles::FLOOR => {
                        if x % 2 == 0 && y % 2 == 0 {
                            glyph = to_cp437('.');
                        } else {
                            glyph = to_cp437(' ');
                        }
                    }
                    AbstractMapTiles::GROUND => glyph = to_cp437(','),
                    AbstractMapTiles::UNKNOWN => glyph = to_cp437('?'),
                }
                ctx.set(
                    x,
                    y,
                    self.palette.color_idx(3).expect("Palette load failed."),
                    self.palette.bg(),
                    glyph,
                );
            }
        }
    }
}
