use crate::abstract_tiles::AbstractMapTiles;
//use bracket_lib::prelude::BaseMap;
use glam::UVec2;

#[allow(unused)]
enum MapType {
    Base,
    Dungeon,
    Cave,
    Overworld,
    ForestCamp,
    Tower,
    Labyrinth,
}

type TileMap = Vec<AbstractMapTiles>;

struct MapBuilder {
    map_type: MapType,
    dimensions: UVec2,
}

impl MapBuilder {
    fn new() -> Self {
        MapBuilder {
            map_type: MapType::Base,
            dimensions: UVec2 { x: 80, y: 50 },
        }
    }

    fn of_type(&mut self, map_type: MapType) {
        self.map_type = map_type;
    }

    fn with_dimensions(&mut self, x: u32, y: u32) {
        self.dimensions.x = x;
        self.dimensions.y = y;
    }

    fn build(&self) -> TileMap {
        let mut new_map =
            vec![AbstractMapTiles::FLOOR; self.dimensions.y as usize * self.dimensions.x as usize];
        for x in 0..self.dimensions.x {
            new_map[x as usize] = AbstractMapTiles::WALL;
            new_map[((self.dimensions.y - 1) * self.dimensions.x) as usize + x as usize];
        }
        for y in 1..self.dimensions.y - 1 {
            new_map[(y * self.dimensions.x) as usize] = AbstractMapTiles::WALL;
            new_map[(y * self.dimensions.x + self.dimensions.x - 1) as usize] =
                AbstractMapTiles::WALL;
        }
        new_map
    }
}
