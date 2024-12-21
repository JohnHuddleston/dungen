#[allow(unused)]
use crate::map_gen::{
    abstract_map::{MapType, TileMap},
    abstract_tiles::AbstractMapTiles,
    bsp_gen::build_bsp_dungeon,
    hauberk_gen::build_hauberk_dungeon,
};
use crate::palettes::Palette;
use glam::UVec2;
use specs::{prelude::*, Component};

pub struct LevelBuilder {
    map_type: MapType,
    palette: Palette,
    dimensions: UVec2,
    n_maps: u8,
}

#[derive(Component)]
pub struct Level {
    pub map_type: MapType,
    pub palette: Palette,
    pub dimensions: UVec2,
    pub maps: Vec<TileMap>,
}

#[allow(unused)]
impl LevelBuilder {
    pub fn new() -> Self {
        LevelBuilder {
            map_type: MapType::Base,
            palette: Palette::Default,
            dimensions: UVec2 { x: 80, y: 50 },
            n_maps: 1,
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

    pub fn with_n_maps(mut self, n: u8) -> LevelBuilder {
        self.n_maps = n;
        self
    }

    pub fn with_palette(mut self, palette: Palette) -> LevelBuilder {
        self.palette = palette;
        self
    }

    pub fn build(&self) -> Option<Level> {
        let mut maps: Vec<TileMap> = Vec::new();
        for _ in 0..self.n_maps {
            let mut new_map = TileMap::new(UVec2 { x: 79, y: 49 }, self.palette);
            //build_bsp_dungeon(&mut new_map, &self.dimensions);
            build_hauberk_dungeon(&mut new_map);
            maps.push(new_map);
        }
        Some(Level {
            map_type: self.map_type,
            palette: self.palette,
            dimensions: self.dimensions,
            maps,
        })
    }
}
