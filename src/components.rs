use bracket_lib::prelude::*;
use specs::{prelude::*, Component};

#[derive(Component, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
pub struct Name {
    pub name: String,
}

#[derive(Component, Debug)]
pub struct Renderable {
    pub glyph: FontCharType,
    pub color_index: usize,
}

#[derive(Component, Debug)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Monster {}
