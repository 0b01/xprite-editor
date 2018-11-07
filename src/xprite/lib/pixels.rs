use xprite::prelude::*;
use std::slice::Iter;
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


#[derive(Clone, Debug)]
pub struct Pixels(pub Vec<Pixel>);
impl Pixels {
    pub fn new() -> Self {
        Pixels(Vec::new())
    }
    pub fn from_slice(slice: &[Pixel]) -> Self {
        let mut vec = Vec::new();
        for i in slice.iter() {
            vec.push(*i);
        }
        Pixels(vec)
    }
    pub fn extend(&mut self, other: &Pixels) {
        self.0.extend(&other.0)
    }
    pub fn push(&mut self, px: Pixel) {
        self.0.push(px);
    }
    pub fn contains(&mut self, px: &Pixel) -> bool {
        self.0.contains(px)
    }
    pub fn iter(&self) -> Iter<Pixel> {
        self.0.iter()
    }
    pub fn set_color(&mut self, color: &Color) {
        let color = ColorOption::Set(*color);
        self.0 = self.0
            .iter()
            .map(|Pixel {point,..}| { Pixel{ point: *point, color } })
            .collect::<Vec<_>>();
    }
}
