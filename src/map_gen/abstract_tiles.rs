#![allow(unused)]

use std::fmt;

pub struct RenderInfo {
    pub chars: [char; 2],
    pub fg_index: usize,
    pub bg_index: usize,
}

#[allow(unused)]
#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub enum AbstractMapTiles {
    Abyss,  // Empty cell
    Floor,  // Walkable indoor cell
    Ground, // Walkable outdoor cell
    Wall,   // Wall cell
    Pit,    // Lava, water, etc.
    StairsUp,
    StairsDown,
    StairsBidirectional,
    DoorClosed,
    DoorOpen,
    Unknown, // Should'nt be used
}

impl fmt::Display for AbstractMapTiles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let character = self.as_char();
        write!(f, "{}", character)
    }
}

impl From<&char> for AbstractMapTiles {
    fn from(value: &char) -> Self {
        match value {
            ' ' => AbstractMapTiles::Abyss,
            '.' => AbstractMapTiles::Floor,
            ',' => AbstractMapTiles::Ground,
            '#' => AbstractMapTiles::Wall,
            '_' => AbstractMapTiles::Pit,
            '^' => AbstractMapTiles::StairsUp,
            'v' => AbstractMapTiles::StairsDown,
            'X' => AbstractMapTiles::StairsBidirectional,
            '+' => AbstractMapTiles::DoorClosed,
            '-' => AbstractMapTiles::DoorOpen,
            _ => AbstractMapTiles::Unknown,
        }
    }
}

impl AbstractMapTiles {
    pub fn as_char(&self) -> char {
        match *self {
            AbstractMapTiles::Abyss => ' ',
            AbstractMapTiles::Floor => '.',
            AbstractMapTiles::Ground => ',',
            AbstractMapTiles::Wall => '#',
            AbstractMapTiles::Pit => '_',
            AbstractMapTiles::StairsUp => '^',
            AbstractMapTiles::StairsDown => 'v',
            AbstractMapTiles::StairsBidirectional => 'X',
            AbstractMapTiles::DoorClosed => '+',
            AbstractMapTiles::DoorOpen => '-',
            AbstractMapTiles::Unknown => '?',
        }
    }

    pub fn render_info(&self) -> RenderInfo {
        match *self {
            AbstractMapTiles::Abyss => RenderInfo {
                chars: ['░', '░'],
                fg_index: 8,
                bg_index: 0,
            },
            AbstractMapTiles::Floor => RenderInfo {
                chars: ['.', ' '],
                fg_index: 15,
                bg_index: 0,
            },
            AbstractMapTiles::Ground => RenderInfo {
                chars: [',', ' '],
                fg_index: 15,
                bg_index: 0,
            },
            AbstractMapTiles::Wall => RenderInfo {
                chars: ['█', '█'],
                fg_index: 15,
                bg_index: 0,
            },
            AbstractMapTiles::Pit => RenderInfo {
                chars: ['_', '_'],
                fg_index: 7,
                bg_index: 0,
            },
            AbstractMapTiles::StairsUp => RenderInfo {
                chars: ['^', '^'],
                fg_index: 15,
                bg_index: 0,
            },
            AbstractMapTiles::StairsDown => RenderInfo {
                chars: ['v', 'v'],
                fg_index: 15,
                bg_index: 0,
            },
            AbstractMapTiles::StairsBidirectional => RenderInfo {
                chars: ['X', 'X'],
                fg_index: 15,
                bg_index: 0,
            },
            AbstractMapTiles::DoorClosed => RenderInfo {
                chars: ['+', '+'],
                fg_index: 15,
                bg_index: 0,
            },
            AbstractMapTiles::DoorOpen => RenderInfo {
                chars: ['-', '-'],
                fg_index: 15,
                bg_index: 0,
            },
            AbstractMapTiles::Unknown => RenderInfo {
                chars: ['?', '?'],
                fg_index: 14,
                bg_index: 4,
            },
        }
    }
}
