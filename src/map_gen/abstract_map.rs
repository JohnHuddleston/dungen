use crate::map_gen::abstract_tiles::AbstractMapTiles;
use crate::{Palette, Viewshed};
use bracket_lib::prelude::*;
use glam::UVec2;

// These will eventually become 'recipes' that determine number of levels, what generation should
// be done on each level, etc.  May need to pair with difficulty rating, or that can be handled on
// the population side.
#[allow(unused)]
#[derive(Clone, Copy)]
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
    pub discovered: Vec<bool>,
    pub visible: Vec<bool>,
    pub palette: Palette,
    pub dimensions: UVec2,
    pub player_spawn: UVec2,
    pub exits: Vec<UVec2>,
}

#[allow(unused)]
impl TileMap {
    pub fn new(dimensions: UVec2, palette: Palette) -> Self {
        TileMap {
            tilemap: vec![AbstractMapTiles::WALL; dimensions.x as usize * dimensions.y as usize],
            discovered: vec![false; dimensions.x as usize * dimensions.y as usize],
            visible: vec![false; dimensions.x as usize * dimensions.y as usize],
            palette,
            dimensions,
            player_spawn: UVec2 { x: 0, y: 0 },
            exits: Vec::new(),
        }
    }

    pub fn draw(&self, viewshed: &Viewshed, ctx: &mut BTerm) {
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
                let point: Point = Point::new(x, y);
                if self.discovered[(y * self.dimensions.x + x) as usize] {
                    if viewshed.visible_tiles.contains(&point) {
                        ctx.set(x, y, self.palette.fg(), self.palette.bg(), glyph);
                    } else {
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
    }
}

impl Algorithm2D for TileMap {
    fn dimensions(&self) -> Point {
        Point::new(self.dimensions.x, self.dimensions.y)
    }
}

impl BaseMap for TileMap {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tilemap[idx] == AbstractMapTiles::WALL
    }
}
