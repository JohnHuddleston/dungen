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

const ADJACENT_OFFSETS: &[(i8, i8)] = &[
    (0, -1),  // north
    (1, -1),  // northeast
    (1, 0),   // east
    (1, 1),   // southeast
    (0, 1),   // south
    (-1, 1),  // southwest
    (-1, 0),  // west
    (-1, -1), // northwest
];

#[allow(unused)]
pub struct TileMap {
    pub tilemap: Vec<AbstractMapTiles>,
    pub rooms: Vec<Rect>,
    pub discovered: Vec<bool>,
    pub visible: Vec<bool>,
    pub blocked: Vec<bool>,
    pub dimensions: UVec2,
    pub player_spawn: UVec2,
    pub exits: Vec<UVec2>,
}

#[allow(unused)]
impl TileMap {
    pub fn new(dimensions: UVec2) -> Self {
        TileMap {
            tilemap: vec![AbstractMapTiles::Wall; dimensions.x as usize * dimensions.y as usize],
            rooms: Vec::new(),
            discovered: vec![false; dimensions.x as usize * dimensions.y as usize],
            visible: vec![false; dimensions.x as usize * dimensions.y as usize],
            blocked: vec![false; dimensions.x as usize * dimensions.y as usize],
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

    fn get_idx_neighbors(&self, idx: usize) -> Vec<usize> {
        let mut valid_neighbors: Vec<usize> = Vec::with_capacity(8);
        let row = (idx / self.dimensions.x as usize);
        let col = (idx % self.dimensions.x as usize);
        let mut new_idx: (isize, isize);
        for (x_off, y_off) in ADJACENT_OFFSETS {
            new_idx = (
                col as isize + isize::from(x_off.clone()),
                row as isize + isize::from(y_off.clone()),
            );
            if new_idx.0 >= 0
                && new_idx.0 <= (self.dimensions.x - 1) as isize
                && new_idx.1 >= 0
                && new_idx.1 <= (self.dimensions.y - 1) as isize
            {
                valid_neighbors
                    .push(new_idx.1 as usize * self.dimensions.x as usize + new_idx.0 as usize)
            }
        }
        valid_neighbors
    }

    pub fn xy_to_idx(&self, x: u32, y: u32) -> u32 {
        return y * self.dimensions.x + x;
    }

    pub fn idx_to_xy(&self, idx: u32) -> UVec2 {
        UVec2 {
            x: idx as u32 % self.dimensions.x,
            y: idx as u32 / self.dimensions.x,
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
        self.tilemap[idx] == AbstractMapTiles::Wall
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let point_a = Point::new(
            idx1 % self.dimensions.y as usize,
            idx1 / self.dimensions.y as usize,
        );
        let point_b = Point::new(
            idx2 % self.dimensions.y as usize,
            idx2 / self.dimensions.y as usize,
        );
        DistanceAlg::Pythagoras.distance2d(point_a, point_b)
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits: SmallVec<[(usize, f32); 10]> = SmallVec::new();
        // let's push logic for finding valid neighbors to a new method on TileMap
        let valid_neighbors = self.get_idx_neighbors(idx);
        for neighbor_idx in valid_neighbors.iter() {
            if self.tilemap[*neighbor_idx] != AbstractMapTiles::Wall {
                exits.push((*neighbor_idx, 1.0));
            }
        }
        exits
    }
}
