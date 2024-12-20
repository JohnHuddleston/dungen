// (Roughly) translated to Rust from https://github.com/munificent/hauberk/blob/db360d9efa714efb6d937c31953ef849c7394a39/lib/src/content/dungeon.dart

use std::{
    collections::{HashMap, HashSet},
    usize,
};

use crate::map_gen::{abstract_map::TileMap, abstract_tiles::AbstractMapTiles};
use bracket_lib::prelude::Rect;
use rand::prelude::*;

const MAX_PLACEMENT_TRIES: u16 = 200;
const BASE_ROOM_MAX: u32 = 4;
const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];
const WINDING_FACTOR: i32 = 10;

pub fn build_hauberk_dungeon(tilemap: &mut TileMap) {
    let mut current_region = -1;
    let mut regions: Vec<i32> = vec![-1; (tilemap.dimensions.x * tilemap.dimensions.y) as usize];
    let mut rooms: Vec<Rect> = Vec::new();

    let mut rng = rand::thread_rng();

    // add rooms
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

        current_region += 1;
        new_rect.for_each(|p| {
            carve(
                tilemap,
                (p.x as u32, p.y as u32),
                &mut regions,
                current_region,
            );
        });
        rooms.push(new_rect);
    }

    // add maze
    for y in (1..tilemap.dimensions.y).step_by(2) {
        for x in (1..tilemap.dimensions.x).step_by(2) {
            if tilemap.tilemap[(y as u32 * tilemap.dimensions.x + x as u32) as usize]
                == AbstractMapTiles::WALL
            {
                let start = (x, y);
                let mut cells: Vec<(u32, u32)> = Vec::new();
                let mut last_dir: (i32, i32) = (0, 0);

                current_region += 1;
                carve(tilemap, start, &mut regions, current_region);

                cells.push(start.clone());
                let mut unmade_cells: Vec<(i32, i32)> = Vec::new();
                while !cells.is_empty() {
                    let cell = cells.last().unwrap();
                    unmade_cells.clear();
                    for direction in DIRECTIONS.iter() {
                        if can_carve(&tilemap, &cell, &direction) {
                            unmade_cells.push(direction.clone());
                        }
                    }

                    if !unmade_cells.is_empty() {
                        let dir: (i32, i32);
                        if unmade_cells.contains(&last_dir)
                            && rng.gen_range(1..=100) > WINDING_FACTOR
                        {
                            dir = last_dir;
                        } else {
                            dir = unmade_cells[rng.gen_range(0..unmade_cells.len())];
                        }
                        let one_out: (u32, u32) = (
                            (cell.0 as i32 + dir.0) as u32,
                            (cell.1 as i32 + dir.1) as u32,
                        );
                        let two_out: (u32, u32) = (
                            (cell.0 as i32 + dir.0 * 2) as u32,
                            (cell.1 as i32 + dir.1 * 2) as u32,
                        );
                        carve(tilemap, one_out, &mut regions, current_region);
                        carve(tilemap, two_out, &mut regions, current_region);

                        cells.push(two_out);
                        last_dir = dir;
                    } else {
                        cells.pop();
                        last_dir = (0, 0);
                    }
                }
            }
        }
    }

    // connect regions
    let mut connector_regions: HashMap<(u32, u32), HashSet<i32>> = HashMap::new();
    for y in 1..(tilemap.dimensions.y - 1) {
        for x in 1..(tilemap.dimensions.x - 1) {
            let index = y * tilemap.dimensions.x + x;
            if tilemap.tilemap[index as usize] != AbstractMapTiles::WALL {
                continue;
            }

            let mut region_set: HashSet<i32> = HashSet::new();
            for direction in DIRECTIONS.iter() {
                let neighbor_point = (x as i32 + direction.0, y as i32 + direction.1);
                if (neighbor_point.0 >= 0
                    && neighbor_point.0 <= tilemap.dimensions.x as i32
                    && neighbor_point.1 >= 0
                    && neighbor_point.1 < tilemap.dimensions.y as i32)
                    && regions[(neighbor_point.1 as u32 * tilemap.dimensions.x
                        + neighbor_point.0 as u32) as usize]
                        != -1
                {
                    region_set.insert(
                        regions[(neighbor_point.1 as u32 * tilemap.dimensions.x
                            + neighbor_point.0 as u32) as usize],
                    );
                }
            }
            if region_set.len() >= 2 {
                connector_regions.insert((x, y), region_set);
            }
        }
    }

    let mut connectors: Vec<&(u32, u32)> = connector_regions.keys().collect();
    let mut merged: HashMap<i32, i32> = HashMap::new();
    let mut open_regions: HashSet<i32> = HashSet::new();

    for i in 0..=current_region {
        merged.insert(i, i);
        open_regions.insert(i);
    }

    while open_regions.len() > 1 {
        let random_connector = connectors[rng.gen_range(0..connectors.len())];
        tilemap.tilemap
            [(random_connector.1 * tilemap.dimensions.x + random_connector.0) as usize] =
            AbstractMapTiles::FLOOR;

        let mut affected_regions = connector_regions
            .get(&random_connector)
            .expect("Tried to get a connector not in the connector_regions map.")
            .iter()
            .map(|r| merged.get(r).unwrap());
        let dest = *affected_regions.next().unwrap();
        let sources: Vec<i32> = affected_regions.map(|v| *v).collect();

        for i in 0..=current_region {
            if sources.contains(&merged.get(&i).unwrap()) {
                merged.insert(i, dest);
            }
        }

        sources.iter().for_each(|s| {
            open_regions.remove(s);
        });
        //let reduced_connectors: Vec<&(u32, u32)> = connectors
        //    .iter()
        //    .cloned()
        //    .filter(|c| {
        //        let distance = f64::sqrt(
        //            (random_connector.0.abs_diff(c.0).pow(2)
        //                + random_connector.1.abs_diff(c.1).pow(2)) as f64,
        //        ) as i32;
        //        if distance <= 3 {
        //            ()
        //        }
        //        let region_test: HashSet<i32> = connector_regions
        //            .get(&c)
        //            .unwrap()
        //            .iter()
        //            .map(|r| *merged.get(r).unwrap())
        //            .collect();
        //        if region_test.len() >= 1 {
        //            ()
        //        }
        //        // add optional here (P=1/3 in original)
        //        true
        //    })
        //    //.map(|c| *c)
        //    .collect();
        let reduced_connectors: Vec<&(u32, u32)> = connectors
            .iter()
            .filter(|c| {
                f64::sqrt(
                    ((random_connector.0.abs_diff(c.0).pow(2))
                        + (random_connector.1.abs_diff(c.1)).pow(2)) as f64,
                ) as u32
                    > 3
            })
            .filter(|c| {
                connector_regions
                    .get(c)
                    .unwrap()
                    .iter()
                    .map(|r| *merged.get(r).unwrap())
                    .collect::<HashSet<i32>>()
                    .len()
                    > 1
            })
            .map(|c| *c)
            .collect();
        connectors = reduced_connectors;
    }

    // remove dead-ends
    let mut done = false;

    while !done {
        done = true;

        for y in 1..(tilemap.dimensions.y - 1) {
            for x in 1..(tilemap.dimensions.x - 1) {
                let index = y * tilemap.dimensions.x + x;
                if tilemap.tilemap[index as usize] == AbstractMapTiles::WALL {
                    continue;
                }

                let mut exits = 0;
                for direction in DIRECTIONS.iter() {
                    let offset_index: usize =
                        ((y as i32 + direction.1) * tilemap.dimensions.x as i32
                            + (x as i32 + direction.0)) as usize;
                    if tilemap.tilemap[offset_index] == AbstractMapTiles::FLOOR {
                        exits += 1;
                    }
                }
                if exits != 1 {
                    continue;
                }
                done = false;
                tilemap.tilemap[index as usize] = AbstractMapTiles::WALL;
            }
        }
    }

    tilemap.player_spawn.x = (rooms[0].x1 + rooms[0].width() / 2) as u32;
    tilemap.player_spawn.y = (rooms[0].y1 + rooms[0].height() / 2) as u32;
}

fn can_carve(tilemap: &TileMap, position: &(u32, u32), direction: &(i32, i32)) -> bool {
    let two_out: (i32, i32) = (
        position.0 as i32 + direction.0 * 2,
        position.1 as i32 + direction.1 * 2,
    );
    let three_out: (i32, i32) = (
        position.0 as i32 + direction.0 * 3,
        position.1 as i32 + direction.1 * 3,
    );

    if three_out.0 >= 0
        && three_out.0 < tilemap.dimensions.x as i32
        && three_out.1 >= 0
        && three_out.1 < tilemap.dimensions.y as i32
    {
        let two_out_wall = tilemap.tilemap
            [(two_out.1 as u32 * tilemap.dimensions.x + two_out.0 as u32) as usize]
            == AbstractMapTiles::WALL;
        two_out_wall
    } else {
        false
    }
}

fn carve(tilemap: &mut TileMap, position: (u32, u32), regions: &mut Vec<i32>, current_region: i32) {
    let index: usize = (position.1 * tilemap.dimensions.x + position.0) as usize;
    tilemap.tilemap[index] = AbstractMapTiles::FLOOR;
    regions[index] = current_region;
}
