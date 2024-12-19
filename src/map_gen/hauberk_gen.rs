use crate::map_gen::{abstract_map::TileMap, abstract_tiles::AbstractMapTiles};
use bracket_lib::prelude::Rect;
use rand::prelude::*;

const MAX_PLACEMENT_TRIES: u16 = 500;
const BASE_ROOM_MAX: u32 = 4;

pub fn build_hauberk_dungeon(tilemap: &mut TileMap) {
    let mut rooms: Vec<Rect> = Vec::new();

    let mut rng = rand::thread_rng();
    for _ in 0..MAX_PLACEMENT_TRIES {
        let size = rng.gen_range(1..=BASE_ROOM_MAX) * 2 + 1;
        let extension = ((rng.gen_range(0..1) + size) / 2) * 2;
        let mut width = size;
        let mut height = size;

        let coin_flip = rng.gen_range(0..=1);
        if coin_flip == 0 {
            width += extension;
        } else {
            height += extension;
        }

        let x = rng.gen_range(0..((tilemap.dimensions.x - width) / 2)) * 2 + 1;
        let y = rng.gen_range(0..((tilemap.dimensions.y - height) / 2)) * 2 + 1;

        let new_rect = Rect::with_size(x, y, width, height);

        let mut intersects = false;
        for room in rooms.iter() {
            if new_rect.intersect(&room) {
                intersects = true;
                break;
            }
        }
        if intersects {
            continue;
        }

        new_rect.for_each(|p| {
            tilemap.tilemap[(p.y as u32 * tilemap.dimensions.x + p.x as u32) as usize] =
                AbstractMapTiles::FLOOR
        });
        rooms.push(new_rect);
    }

    tilemap.player_spawn.x = (rooms[0].x1 + rooms[0].width() / 2) as u32;
    tilemap.player_spawn.y = (rooms[0].y1 + rooms[0].height() / 2) as u32;
}
