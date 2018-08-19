use xprite::Color;
use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Block {
    pub x: u32,
    pub y: u32,
    pub color: Color,
}

impl Block {
    pub fn from_tuple((x,y): (u32, u32), color: Color) -> Self {
        Block {
            x,
            y,
            color
        }
    }
}

impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

macro_rules! blocks {
    ($i:expr, $j: expr) => {
        Block {
            x: $i,
            y: $j,
            color: Color::red(),
        }
    };
}
