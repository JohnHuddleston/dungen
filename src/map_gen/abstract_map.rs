use crate::map_gen::abstract_tiles::AbstractMapTiles;
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
    pub rooms: Vec<Rect>,
    pub discovered: Vec<bool>,
    pub visible: Vec<bool>,
    pub dimensions: UVec2,
    pub player_spawn: UVec2,
    pub exits: Vec<UVec2>,
}

#[allow(unused)]
impl TileMap {
    pub fn new(dimensions: UVec2) -> Self {
        TileMap {
            tilemap: vec![AbstractMapTiles::WALL; dimensions.x as usize * dimensions.y as usize],
            rooms: Vec::new(),
            discovered: vec![false; dimensions.x as usize * dimensions.y as usize],
            visible: vec![false; dimensions.x as usize * dimensions.y as usize],
            dimensions,
            player_spawn: UVec2 { x: 0, y: 0 },
            exits: Vec::new(),
        }
    }

    pub fn draw(&self, ctx: &mut BTerm, palette: &[RGBA]) {
        for x in 0..self.dimensions.x {
            for y in 0..self.dimensions.y {
                let tile_data = self.tilemap[y as usize * self.dimensions.x as usize + x as usize]
                    .render_info();
                let point: Point = Point::new(x, y);
                let idx: usize = (y * self.dimensions.x + x) as usize;
                if self.discovered[idx] {
                    if self.visible[idx] {
                        ctx.set(
                            x,
                            y,
                            palette[tile_data.fg_index],
                            palette[tile_data.bg_index],
                            tile_data.chars[(x % 2) as usize],
                        );
                    } else {
                        ctx.set(
                            x,
                            y,
                            palette[7],
                            palette[8],
                            tile_data.chars[(x % 2) as usize],
                        );
                    }
                } else {
                    ctx.set(x, y, palette[0], palette[0], ' ');
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
