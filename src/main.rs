mod map_gen {
    pub mod abstract_map;
    pub mod abstract_tiles;
    pub mod bsp_gen;
    pub mod generator;
    pub mod hauberk_gen;
}
mod palettes;
mod visibility_system;

use crate::map_gen::{
    abstract_map::MapType,
    generator::{Level, LevelBuilder},
};
use crate::visibility_system::VisiblitySystem;
use bracket_lib::prelude::*;
use map_gen::abstract_tiles::AbstractMapTiles;
use palettes::PaletteManager;
use specs::{prelude::*, Component};

#[derive(Component, Debug)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Component, Debug)]
pub struct Renderable {
    glyph: FontCharType,
    color_index: usize,
}

#[derive(Component, Debug)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component)]
pub struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
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
        }
    }
}

fn cycle_palette(ecs: &mut World) {
    let mut palette_man = ecs.fetch_mut::<PaletteManager>();
    palette_man.current = (palette_man.current + 1) % palette_man.palettes.len();
    println!(
        "[PaletteManager] Switched to '{}' palette.",
        palette_man.palettes[palette_man.current].name
    );
}

fn player_input(gs: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::A => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right | VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Down | VirtualKeyCode::S => try_move_player(0, 1, &mut gs.ecs),
            VirtualKeyCode::Up | VirtualKeyCode::W => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::T => cycle_palette(&mut gs.ecs),
            _ => {}
        },
    }
}

struct State {
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let level = self.ecs.fetch::<Level>();
        //let viewsheds = self.ecs.read_storage::<Viewshed>();
        let palette_man = self.ecs.fetch::<PaletteManager>();
        let palette = &palette_man.palettes[palette_man.current].colors.as_slice();
        level.maps[0].draw(ctx, palette);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            if level.maps[0].visible[(pos.y * level.maps[0].dimensions.x as i32 + pos.x) as usize] {
                ctx.set(
                    pos.x,
                    pos.y,
                    palette[render.color_index],
                    palette[0],
                    render.glyph,
                );
            }
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisiblitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
    }
}

fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_gutter(8)
        .with_fps_cap(60.)
        .with_dimensions(160, 100)
        .with_fitscreen(true)
        .with_title("Chosen v0.0.0.1pre-alpha")
        .build()?;

    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();

    let level = LevelBuilder::new()
        .with_dimensions(79, 49)
        .of_type(MapType::Base)
        .build()
        .expect("Level failed to generate.");

    for room in level.maps[0].rooms.iter().skip(1) {
        let center_point = room.center();
        gs.ecs
            .create_entity()
            .with(Position {
                x: center_point.x,
                y: center_point.y,
            })
            .with(Renderable {
                glyph: to_cp437('g'),
                color_index: 12,
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .build();
    }

    gs.ecs
        .create_entity()
        .with(Position {
            x: level.maps[0].player_spawn.x as i32,
            y: level.maps[0].player_spawn.y as i32,
        })
        .with(Renderable {
            glyph: to_cp437('@'),
            color_index: 11,
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    gs.ecs.insert(level);

    let palette_man = PaletteManager::new();
    gs.ecs.insert(palette_man);

    main_loop(context, gs)
}
