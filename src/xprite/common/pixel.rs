use xprite::Color;
use std::hash::{Hash, Hasher};
use lyon_geom::euclid::Point2D;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Pixel {
    pub point: Point2D<u32>,
    pub color: Option<Color>,
}

impl Pixel {
    pub fn from_tuple((x,y): (u32, u32), color: Option<Color>) -> Self {
        let point = Point2D::new(x, y);
        Pixel {
            point,
            color
        }
    }
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
            color: Some(Color::red()),
        }
    };
}
