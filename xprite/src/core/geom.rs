use std::hash::{Hash, Hasher};
use std::cmp::Ordering;

/// represents a 2D vector
#[derive(Debug, Copy, Clone, PartialOrd, Serialize, Deserialize, Default)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}

impl Ord for Vec2D {
    fn cmp(&self, other: &Vec2D) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Vec2D {}

impl PartialEq for Vec2D {
    fn eq(&self, other: &Vec2D) -> bool {
        (self.x as i32 == other.x as i32) &&
        (self.y as i32 == other.y as i32)
    }
}


impl Hash for Vec2D {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as i32).hash(state);
        (self.y as i32).hash(state);
    }
}

impl Vec2D {
    /// create a new point
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

impl From<Vec2D> for [f32;2] {
    fn from(p: Vec2D) -> Self {
        [p.x, p.y]
    }
}


#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CubicBezierSegment {
    pub from: Vec2D,
    pub ctrl1: Vec2D,
    pub ctrl2: Vec2D,
    pub to: Vec2D,
}

impl CubicBezierSegment {
    pub fn sample(&self, t: f32) -> Vec2D {
        let t2 = t * t;
        let t3 = t2 * t;
        let one_t = 1. - t;
        let one_t2 = one_t * one_t;
        let one_t3 = one_t2 * one_t;

        let x = self.from.x * one_t3 +
            self.ctrl1.x * 3. * one_t2 * t +
            self.ctrl2.x * 3. * one_t * t2 +
            self.to.x * t3;

        let y = self.from.y * one_t3 +
            self.ctrl1.y * 3. * one_t2 * t +
            self.ctrl2.y * 3. * one_t * t2 +
            self.to.y * t3;

        Vec2D::new(x, y)
    }
}
