use crate::{components::Position, map_gen::generator::Level, Monster, Name, Viewshed};
use bracket_lib::prelude::*;
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        WriteExpect<'a, Level>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut level, player_pos, mut viewshed, monster, name, mut pos) = data;

        for (viewshed, _monster, name, pos) in (&mut viewshed, &monster, &name, &mut pos).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(&format!("{} ponders existence", name.name));

                let path = a_star_search(
                    level.maps[0].xy_to_idx(pos.x as u32, pos.y as u32),
                    level.maps[0].xy_to_idx(player_pos.x as u32, player_pos.y as u32),
                    &mut level.maps[0],
                );

                if path.success && path.steps.len() > 1 {
                    pos.x = (path.steps[1] as u32 % level.maps[0].dimensions.x) as i32;
                    pos.y = (path.steps[1] as u32 / level.maps[0].dimensions.x) as i32;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
