mod map_gen {
    pub mod abstract_map;
    pub mod abstract_tiles;
    pub mod bsp_gen;
    pub mod generator;
    pub mod hauberk_gen;
}
mod palettes;

use crate::map_gen::{
    abstract_map::{MapType, TileMap},
    generator::LevelBuilder,
};
use crate::palettes::Palette;
use bracket_lib::prelude::*;
use specs::prelude::*;

struct Position {
    x: i32,
    y: i32,
}

impl Component for Position {
    type Storage = DenseVecStorage<Self>;
}

struct Renderable {
    glyph: FontCharType,
    fg: RGBA,
    bg: RGBA,
}

impl Component for Renderable {
    type Storage = DenseVecStorage<Self>;
}

struct State {
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        let map = self.ecs.fetch::<Vec<TileMap>>();
        map[0].draw(ctx);
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
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

    let palette = Palette::GB;

    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();

    let levels = LevelBuilder::new()
        .with_dimensions(79, 49)
        .of_type(MapType::Base)
        .with_palette(palette)
        .build()
        .expect("Level failed to generate.");

    gs.ecs
        .create_entity()
        .with(Position {
            x: levels[0].player_spawn.x as i32,
            y: levels[0].player_spawn.y as i32,
        })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: palette.color_idx(2).expect("Palette load failed."),
            bg: palette.bg(),
        })
        .build();

    gs.ecs.insert(levels);

    main_loop(context, gs)
}
