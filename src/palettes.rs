use bracket_lib::color::RGBA;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use std::fs;

#[derive(Debug, Serialize, Deserialize)]
struct ColorPalette {
    name: String,
    colors: Vec<String>,
}

#[allow(unused)]
pub struct ColorPaletteRGBA {
    pub name: String,
    pub colors: Vec<RGBA>,
}

pub struct PaletteManager {
    pub palettes: Vec<ColorPaletteRGBA>,
    pub current: usize,
}

impl PaletteManager {
    pub fn new() -> Self {
        let mut palettes: Vec<ColorPaletteRGBA> = Vec::new();
        let palette_files = fs::read_dir("resources/palettes")
            .expect("Reading palettes directory failed.")
            .map(|f| f.unwrap().path())
            .filter_map(|f| {
                let f = fs::File::open(f).expect("Failed to open palette file.");
                match from_reader(f) {
                    Ok(x) => Some(x),
                    Err(_) => None,
                }
            })
            .collect::<Vec<ColorPalette>>();

        let default_colors: Vec<&str> = vec![
            "#000000ff",
            "#000080ff",
            "#008000ff",
            "#008080ff",
            "#800000ff",
            "#800080ff",
            "#808000ff",
            "#c0c0c0ff",
            "#808080ff",
            "#0000ffff",
            "#00ff00ff",
            "#00ffffff",
            "#ff0000ff",
            "#ff00ffff",
            "#ffff00ff",
            "#ffffffff",
        ];

        palettes.push(ColorPaletteRGBA {
            name: "Default".to_string(),
            colors: default_colors
                .iter()
                .map(|c| RGBA::from_hex(*c).unwrap())
                .collect(),
        });
        println!("[PaletteManager] Loaded 'Default' color palette (index 0).");

        let mut converted_colors: Vec<RGBA> = Vec::new();

        for palette in palette_files {
            converted_colors.clear();
            converted_colors = palette
                .colors
                .iter()
                .map(|c| RGBA::from_hex(c).unwrap())
                .collect();
            println!(
                "[PaletteManager] Loaded '{}' color palette. (index {})",
                palette.name,
                palettes.len()
            );
            palettes.push(ColorPaletteRGBA {
                name: palette.name,
                colors: converted_colors.clone(),
            });
        }

        PaletteManager {
            palettes,
            current: 0,
        }
    }
}

pub fn cycle_palette(ecs: &mut World) {
    let mut palette_man = ecs.fetch_mut::<PaletteManager>();
    palette_man.current = (palette_man.current + 1) % palette_man.palettes.len();
    println!(
        "[PaletteManager] Switched to '{}' palette.",
        palette_man.palettes[palette_man.current].name
    );
}
