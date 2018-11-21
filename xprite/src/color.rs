#[derive(Debug, Hash, Clone, Eq, Copy, PartialEq)]
pub enum ColorOption {
    Unset,
    Set(Color),
}

#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq)]
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

    pub fn grey() -> Color {
        Color {
            r: 200, g: 200, b: 200, a: 255,
        }
    }
}
