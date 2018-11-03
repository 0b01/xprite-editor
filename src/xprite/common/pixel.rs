use xprite::*;
use std::hash::{Hash, Hasher};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Pixel {
    pub point: Point2D<u32>,
    pub color: ColorOption,
}

impl Hash for Pixel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.point.hash(state);
    }
}

macro_rules! pixel {
    ($i:expr, $j: expr) => {
        Pixel {
            point: Point2D::new($i as u32, $j as u32),
            color: ColorOption::Unset,
        }
    };
}
