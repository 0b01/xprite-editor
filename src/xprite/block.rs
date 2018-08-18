use xprite::Color;
use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Block {
    pub x: u32,
    pub y: u32,
    pub color: Color,
}

impl Hash for Block {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}
