use bracket_lib::color::RGBA;

#[allow(unused)]
#[derive(Clone, Copy)]
pub enum Palette {
    Default,
    GB,
}

const PALETTE_LENGTH: u8 = 4;

const PALETTES: [[RGBA; 4]; 2] = [
    [
        RGBA {
            r: 0.553,
            g: 0.412,
            b: 0.478,
            a: 1.,
        },
        RGBA {
            r: 0.051,
            g: 0.168,
            b: 0.27,
            a: 1.,
        },
        RGBA {
            r: 1.,
            g: 0.831,
            b: 0.64,
            a: 1.,
        },
        RGBA {
            r: 0.329,
            g: 0.306,
            b: 0.408,
            a: 1.,
        },
    ],
    [
        RGBA {
            r: 0.698,
            g: 0.76,
            b: 0.49,
            a: 1.,
        },
        RGBA {
            r: 0.208 - 0.1,
            g: 0.239 - 0.1,
            b: 0.274 - 0.1,
            a: 1.,
        },
        RGBA {
            r: 0.451,
            g: 0.604,
            b: 0.337,
            a: 1.,
        },
        RGBA {
            r: 0.259,
            g: 0.4,
            b: 0.353,
            a: 1.,
        },
    ],
];

#[allow(unused)]
impl Palette {
    pub fn fg(&self) -> RGBA {
        match *self {
            Palette::Default => PALETTES[0][0],
            Palette::GB => PALETTES[1][0],
        }
    }

    pub fn bg(&self) -> RGBA {
        match *self {
            Palette::Default => PALETTES[0][1],
            Palette::GB => PALETTES[1][1],
        }
    }

    pub fn color_idx(&self, idx: u8) -> Option<RGBA> {
        if idx >= PALETTE_LENGTH {
            None
        } else {
            match *self {
                Palette::Default => Some(PALETTES[0][idx as usize]),
                Palette::GB => Some(PALETTES[1][idx as usize]),
            }
        }
    }
}
