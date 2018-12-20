#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("rgba({}, {}, {}, {})",
            self.r,
            self.g,
            self.b,
            self.a,
        )
    }
}

/// [f32;4] = [0.,0.,0.,0.9];
impl From<Color> for [f32; 4] {
    fn from(c: Color) -> Self {
        [c.r as f32 / 255.,
         c.g as f32 / 255.,
         c.b as f32 / 255.,
         c.a as f32/ 255.,
        ]
    }
}

/// [f32;4] = [0.,0.,0.,0.9];
impl From<[f32;4]> for Color {
    fn from(c: [f32;4]) -> Self {
        Color {
            r: (c[0]*255.).floor() as u8,
            g: (c[1]*255.).floor() as u8,
            b: (c[2]*255.).floor() as u8,
            a: (c[3]*255.).floor() as u8,
        }
    }
}


impl Color {
    pub fn new(r:u8, g:u8, b:u8) -> Color {
        Color {
            r, g, b, a:255,
        }
    }

    pub fn red() -> Color {
        Color {
            r: 255, g: 0, b: 0, a: 255,
        }
    }

    pub fn black() -> Color {
        Color {
            r: 0, g: 0, b: 0, a: 255,
        }
    }


    pub fn blue() -> Color {
        Color {
            r: 0, g: 0, b: 255, a: 255,
        }
    }

    pub fn orange() -> Color {
        Color {
            r: 255, g: 128, b: 0, a: 255,
        }
    }

    pub fn grey() -> Color {
        Color {
            r: 1, g: 1, b: 1, a: 255,
        }
    }
}
