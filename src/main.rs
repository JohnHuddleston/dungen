mod map_gen {
    pub mod abstract_map;
    pub mod abstract_tiles;
    pub mod bsp_gen;
    pub mod generator;
    pub mod hauberk_gen;
}
mod components;
mod monster_ai_system;
mod palettes;
mod player;
mod visibility_system;

use crate::map_gen::{
    abstract_map::MapType,
    generator::{Level, LevelBuilder},
};
use crate::{
    components::{Monster, Name, Player, Position, Renderable, Viewshed},
    palettes::PaletteManager,
    player::player_input,
    visibility_system::VisiblitySystem,
};
use bracket_lib::prelude::*;
use monster_ai_system::MonsterAI;
use specs::prelude::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

struct State {
    ecs: World,
    runstate: RunState,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }

        let level = self.ecs.fetch::<Level>();
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
        let mut mob = MonsterAI {};
        mob.run_now(&self.ecs);
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

    let mut gs = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Name>();

    let level = LevelBuilder::new()
        .with_dimensions(79, 49)
        .of_type(MapType::Base)
        .build()
        .expect("Level failed to generate.");

    let mut rng = RandomNumberGenerator::new();

    for (i, room) in level.maps[0].rooms.iter().skip(1).enumerate() {
        let center_point = room.center();
        let glyph: FontCharType;
        let name: String;
        let color_index: usize;
        let roll = rng.roll_dice(1, 2);
        match roll {
            1 => {
                glyph = to_cp437('g');
                name = "Goblin".to_string();
                color_index = 10;
            }
            _ => {
                glyph = to_cp437('o');
                name = "Orc".to_string();
                color_index = 2;
            }
        }
        gs.ecs
            .create_entity()
            .with(Position {
                x: center_point.x,
                y: center_point.y,
            })
            .with(Renderable { glyph, color_index })
            .with(Name {
                name: format!("{} #{}", name, i),
            })
            .with(Viewshed {
                visible_tiles: Vec::new(),
                range: 8,
                dirty: true,
            })
            .with(Monster {})
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
        .with(Name {
            name: "Hero".to_string(),
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();
    gs.ecs.insert(Point::new(
        level.maps[0].player_spawn.x,
        level.maps[0].player_spawn.y,
    ));

    gs.ecs.insert(level);

    let palette_man = PaletteManager::new();
    gs.ecs.insert(palette_man);

    main_loop(context, gs)
}
