use std::fmt;

#[allow(unused)]
#[derive(Eq, PartialEq, Clone, Copy, Hash)]
pub enum AbstractMapTiles {
    ABYSS,   // Empty cell
    FLOOR,   // Walkable indoor cell
    GROUND,  // Walkable outdoor cell
    WALL,    // Wall cell
    PIT,     // Lava, water, etc.
    UNKNOWN, // Should'nt be used
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
            ' ' => AbstractMapTiles::ABYSS,
            '.' => AbstractMapTiles::FLOOR,
            ',' => AbstractMapTiles::GROUND,
            '#' => AbstractMapTiles::WALL,
            '_' => AbstractMapTiles::PIT,
            _ => AbstractMapTiles::UNKNOWN,
        }
    }
}

impl AbstractMapTiles {
    pub fn as_char(&self) -> char {
        match *self {
            AbstractMapTiles::ABYSS => ' ',
            AbstractMapTiles::FLOOR => '.',
            AbstractMapTiles::GROUND => ',',
            AbstractMapTiles::WALL => '#',
            AbstractMapTiles::PIT => '_',
            AbstractMapTiles::UNKNOWN => '?',
        }
    }
}
