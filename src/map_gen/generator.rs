#[allow(unused)]
use crate::map_gen::{
    abstract_map::{MapType, TileMap},
    abstract_tiles::AbstractMapTiles,
    bsp_gen::build_bsp_dungeon,
    hauberk_gen::build_hauberk_dungeon,
};
use crate::palettes::Palette;
use glam::UVec2;

pub struct LevelBuilder {
    map_type: MapType,
    palette: Palette,
    dimensions: UVec2,
    n_levels: u8,
}

#[allow(unused)]
impl LevelBuilder {
    pub fn new() -> Self {
        LevelBuilder {
            map_type: MapType::Base,
            palette: Palette::Default,
            dimensions: UVec2 { x: 80, y: 50 },
            n_levels: 1,
        }
    }

    pub fn of_type(mut self, map_type: MapType) -> LevelBuilder {
        self.map_type = map_type;
        self
    }

    pub fn with_dimensions(mut self, x: u32, y: u32) -> LevelBuilder {
        self.dimensions.x = x;
        self.dimensions.y = y;
        self
    }

    pub fn with_n_levels(mut self, n: u8) -> LevelBuilder {
        self.n_levels = n;
        self
    }

    pub fn with_palette(mut self, palette: Palette) -> LevelBuilder {
        self.palette = palette;
        self
    }

    pub fn build(&self) -> Option<Vec<TileMap>> {
        let mut levels: Vec<TileMap> = Vec::new();
        for _ in 0..self.n_levels {
            let mut new_map = TileMap::new(UVec2 { x: 79, y: 49 }, self.palette);
            //build_bsp_dungeon(&mut new_map, &self.dimensions);
            build_hauberk_dungeon(&mut new_map);
            levels.push(new_map);
        }
        Some(levels)
    }
}
