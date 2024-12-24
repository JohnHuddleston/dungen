use crate::{map_gen::generator::Level, Player, Position, Viewshed};
use bracket_lib::prelude::{field_of_view, Point};
use specs::prelude::*;

pub struct VisiblitySystem {}

impl<'a> System<'a> for VisiblitySystem {
    type SystemData = (
        WriteExpect<'a, Level>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if viewshed.dirty {
                viewshed.dirty = false;
                viewshed.visible_tiles.clear();
                viewshed.visible_tiles =
                    field_of_view(Point::new(pos.x, pos.y), viewshed.range, &map.maps[0]);
                viewshed.visible_tiles.retain(|p| {
                    p.x >= 0
                        && p.x < map.dimensions.x as i32
                        && p.y >= 0
                        && p.y < map.dimensions.y as i32
                });

                let p: Option<&Player> = player.get(ent);
                if let Some(_) = p {
                    for t in map.maps[0].visible.iter_mut() {
                        *t = false;
                    }
                    for vis in viewshed.visible_tiles.iter() {
                        let idx = (vis.y * map.dimensions.x as i32) + vis.x;
                        map.maps[0].discovered[idx as usize] = true;
                        map.maps[0].visible[idx as usize] = true;
                    }
                }
            }
        }
    }
}
