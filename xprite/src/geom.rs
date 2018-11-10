use std::ops::Add;
use std::hash::{Hash, Hasher};

/// represents a point in a 2D cartesian plane
#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Point2D<S: Copy> {
    pub x: S,
    pub y: S,
}

impl Eq for Point2D<f32> {
}


impl Hash for Point2D<f32> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as i32).hash(state);
        (self.y as i32).hash(state);
    }
}

impl<S: Copy + Add<Output=S>> Point2D<S> {
    /// create a new point
    pub fn new(x: S, y: S) -> Self {
        Self { x, y }
    }
}

impl Point2D<u32>  {
    pub fn as_f32(&self) -> Point2D<f32> {
        let x = self.x as f32;
        let y = self.y as f32;
        Point2D {x, y}
    }
}

impl Point2D<i32>  {
    pub fn as_f32(&self) -> Point2D<f32> {
        let x = self.x as f32;
        let y = self.y as f32;
        Point2D {x, y}
    }
}


impl Point2D<u32>  {
    pub fn as_i32(&self) -> Point2D<i32> {
        let x = self.x as i32;
        let y = self.y as i32;
        Point2D {x, y}
    }
}

impl Point2D<f32>  {
    pub fn as_i32(&self) -> Point2D<i32> {
        let x = self.x as i32;
        let y = self.y as i32;
        Point2D {x, y}
    }
}

impl<T: Copy + Add<Output=T>> Add for Point2D<T> {
    type Output = Point2D<T>;
    /// add a vector
    fn add(self, size: Point2D<T>) -> Point2D<T> {
        Point2D {
            x: self.x + size.x,
            y: self.y + size.y,
        }
    }
}

impl From<Point2D<u32>> for Point2D<f32> {
    fn from(p: Point2D<u32>) -> Self {
        Point2D {
            x: p.x as f32,
            y: p.y as f32,
        }
    }
}

#[derive(Debug)]
pub struct CubicBezierSegment<S: Copy> {
    pub from: Point2D<S>,
    pub ctrl1: Point2D<S>,
    pub ctrl2: Point2D<S>,
    pub to: Point2D<S>,
}

impl CubicBezierSegment<f32> {
    pub fn sample(&self, t: f32) -> Point2D<f32> {
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

        Point2D::new(x, y)
    }
}
