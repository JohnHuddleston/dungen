use crate::{
    components::{Player, Position, Viewshed},
    map_gen::{abstract_tiles::AbstractMapTiles, generator::Level},
    palettes::cycle_palette,
    RunState, State,
};
use bracket_lib::prelude::*;
use specs::prelude::*;

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let level = ecs.fetch::<Level>();

    for (_, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let destination_idx = (pos.y + delta_y) * level.dimensions.x as i32 + (pos.x + delta_x);
        if level.maps[0].tilemap[destination_idx as usize] != AbstractMapTiles::WALL {
            pos.x = pos.x + delta_x;
            pos.y = pos.y + delta_y;
            viewshed.dirty = true;
            let mut player_position = ecs.write_resource::<Point>();
            player_position.x = pos.x;
            player_position.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => return RunState::Paused,
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right | VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Down | VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Up | VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::T => {
                cycle_palette(&mut gs.ecs);
                return RunState::Paused;
            }
            _ => return RunState::Paused,
        },
    }
    RunState::Running
}
