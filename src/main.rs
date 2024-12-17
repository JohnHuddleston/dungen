pub mod abstract_tiles;
mod map_gen;

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
    fg: RGB,
    bg: RGB,
}

impl Component for Renderable {
    type Storage = DenseVecStorage<Self>;
}

struct State {
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        //ctx.cls();
        ctx.cls_bg(RGB::from(DARKSLATEGRAY));
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
        .with_title("Roguelike Tutorial")
        .build()?;
    let mut gs = State { ecs: World::new() };
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs
        .create_entity()
        .with(Position { x: 5, y: 10 })
        .with(Renderable {
            glyph: to_cp437('@'),
            fg: RGB::from(FLORALWHITE),
            bg: RGB::from(DARKSLATEGRAY),
        })
        .build();
    main_loop(context, gs)
}
