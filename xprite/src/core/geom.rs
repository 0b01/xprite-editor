#![allow(clippy::many_single_char_names)]

#[macro_export]
macro_rules! vec2f {
    ($y:expr, $x: expr) => {
        Vec2f {
            y: ($y) as f64,
            x: ($x) as f64,
        }
    };
}

#[macro_export]
macro_rules! vec2f_xy {
    ($x:expr, $y: expr) => {
        Vec2f {
            x: ($x) as f64,
            y: ($y) as f64,
        }
    };
}

use crate::prelude::*;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

impl Vec2f {
    pub fn mag(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn clear(&mut self) {
        *self = Default::default();
    }

    pub fn floor(&self) -> Self {
        let x = self.x.floor();
        let y = self.y.floor();
        Vec2f { x, y }
    }
}

impl Neg for Vec2f {
    type Output = Vec2f;

    fn neg(self) -> Vec2f {
        Vec2f { x: -self.x, y: -self.y }
    }
}

impl Add for Vec2f {
    type Output = Vec2f;

    fn add(self, rhs: Vec2f) -> Vec2f {
        Vec2f {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Vec2f {
    fn add_assign(&mut self, rhs: Vec2f) -> () {
        *self = *self + rhs;
    }
}

impl Sub for Vec2f {
    type Output = Vec2f;

    fn sub(self, rhs: Vec2f) -> Vec2f {
        self + (-rhs)
    }
}

impl SubAssign for Vec2f {
    fn sub_assign(&mut self, rhs: Vec2f) -> () {
        *self = *self - rhs;
    }
}

impl Div for Vec2f {
    type Output = Vec2f;

    fn div(self, rhs: Vec2f) -> Vec2f {
        Vec2f {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl Div<f64> for Vec2f {
    type Output = Vec2f;

    fn div(self, rhs: f64) -> Vec2f {
        Vec2f {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl DivAssign for Vec2f {
    fn div_assign(&mut self, rhs: Vec2f) -> () {
        *self = *self / rhs;
    }
}

impl Mul for Vec2f {
    type Output = Vec2f;

    fn mul(self, rhs: Vec2f) -> Vec2f {
        Vec2f {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl Mul<f64> for Vec2f {
    type Output = Vec2f;

    fn mul(self, rhs: f64) -> Vec2f {
        Vec2f {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl MulAssign for Vec2f {
    fn mul_assign(&mut self, rhs: Vec2f) -> () {
        *self = *self * rhs;
    }
}

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd, Default)]
pub struct Rect(pub Vec2f, pub Vec2f);

impl Rect {
    pub fn w(&self) -> f64 {
        (self.1.x - self.0.x + 1.).abs()
    }
    pub fn h(&self) -> f64 {
        (self.1.y - self.0.y + 1.).abs()
    }
}

/// represents a 2D vector
#[cfg_attr(feature = "python-scripting", pyclass)]
#[derive(Copy, Clone, PartialOrd, Default)]
pub struct Vec2f {
    pub x: f64,
    pub y: f64,
}

impl ::std::fmt::Debug for Vec2f {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        write!(f, "[x,y]=[{},{}]", self.x, self.y)
    }
}

#[cfg(feature = "python-scripting")]
impl<'a> pyo3::FromPyObject<'a> for Vec2f {
    fn extract(ob: &'a pyo3::types::PyObjectRef) -> PyResult<Vec2f> {
        let tup: &pyo3::types::PyTuple = ob.extract()?;
        let ret: Vec2f = Vec2f {
            y: tup.get_item(0).extract().unwrap(),
            x: tup.get_item(1).extract().unwrap(),
        };
        Ok(ret)
    }
}

impl Ord for Vec2f {
    fn cmp(&self, other: &Vec2f) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Eq for Vec2f {}

impl PartialEq for Vec2f {
    fn eq(&self, other: &Vec2f) -> bool {
        (self.x as i32 == other.x as i32) && (self.y as i32 == other.y as i32)
    }
}

impl Hash for Vec2f {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.x as i32).hash(state);
        (self.y as i32).hash(state);
    }
}

impl From<Vec2f> for [f64; 2] {
    fn from(p: Vec2f) -> Self {
        [p.x, p.y]
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct CubicBezierSegment {
    pub from: Vec2f,
    pub ctrl1: Vec2f,
    pub ctrl2: Vec2f,
    pub to: Vec2f,
}

impl CubicBezierSegment {
    pub fn sample(&self, t: f64) -> Vec2f {
        let t2 = t * t;
        let t3 = t2 * t;
        let one_t = 1. - t;
        let one_t2 = one_t * one_t;
        let one_t3 = one_t2 * one_t;

        let x = self.from.x * one_t3 + self.ctrl1.x * 3. * one_t2 * t + self.ctrl2.x * 3. * one_t * t2 + self.to.x * t3;

        let y = self.from.y * one_t3 + self.ctrl1.y * 3. * one_t2 * t + self.ctrl2.y * 3. * one_t * t2 + self.to.y * t3;

        Vec2f { x, y }
    }

    /// rasterize a single bezier curve by sampling
    pub fn rasterize(&self, sort: bool, color: Color) -> Option<Pixels> {
        let mut pixs = Pixels::new();

        let mut extrema = vec![0.];
        extrema.extend(self.extrema());
        extrema.push(1.);
        for (start, stop) in extrema.iter().zip(extrema[1..].iter()) {
            let mut monotone_seg = Pixels::new();
            let mut t = *start;
            let n_steps = 1000;
            let step = (stop - start) / n_steps as f64;
            for _ in 0..n_steps {
                let point = self.sample(t);
                let Vec2f { x, y } = Canvas::snap(point);
                let pixel = pixel!(y, x, color);
                // don't allow duplicate pixels
                if !monotone_seg.contains(&pixel) {
                    monotone_seg.push(pixel);
                }
                t += step;
            }
            monotone_seg.pixel_perfect();
            if sort {
                monotone_seg.monotonic_sort();
            }
            pixs.extend(&monotone_seg);
        }

        pixs.pixel_perfect();
        Some(pixs)
    }

    pub fn arc_len(&self, steps: usize) -> f64 {
        let mut acc = 0.;
        let mut prev = self.sample(0.);
        for i in 1..steps {
            let p = self.sample(i as f64 / steps as f64);
            acc += (p - prev).mag();
            prev = p;
        }
        acc
    }

    pub fn extrema(&self) -> Vec<f64> {
        // https://github.com/Pomax/bezierjs/blob/gh-pages/lib/bezier.js#L470

        let dims = vec![0, 1];
        let mut result = vec![vec![], vec![]];

        let mut roots = Vec::new();

        let points = vec![self.from, self.ctrl1, self.ctrl2, self.to];

        let dpoints = derive(points);

        for &dim in &dims {
            let mfn = |point: &Vec2f| {
                if dim == 0 {
                    point.x
                } else {
                    point.y
                }
            };
            let p: Vec<f64> = dpoints[0].iter().map(mfn).collect();
            result[dim] = droots(&p);
            let p: Vec<f64> = dpoints[1].iter().map(mfn).collect();
            result[dim].extend(droots(&p));
            result[dim] = result[dim].iter().filter(|&t| *t >= 0. && *t <= 1.).cloned().collect();
            result[dim].sort_by(|a: &f64, b: &f64| a.partial_cmp(b).unwrap());
            roots.extend(&result[dim]);
        }
        roots.sort_by(|a: &f64, b| a.partial_cmp(b).unwrap());
        return roots;
    }
}

/// https://github.com/Pomax/bezierjs/blob/gh-pages/lib/utils.js#L173
fn derive(points: Vec<Vec2f>) -> Vec<Vec<Vec2f>> {
    let mut dpoints = vec![];
    let mut p = points;
    let mut d = p.len();
    let mut c = d - 1;
    while d > 1 {
        let mut list = vec![];
        for j in 0..c {
            let dpt = Vec2f {
                x: c as f64 * (p[j + 1].x - p[j].x),
                y: c as f64 * (p[j + 1].y - p[j].y),
            };
            list.push(dpt);
        }
        dpoints.push(list.clone());
        p = list;
        d -= 1;
        c -= 1;
    }
    dpoints
}

fn droots(p: &[f64]) -> Vec<f64> {
    if p.len() == 3 {
        let a = p[0];
        let b = p[1];
        let c = p[2];
        let d = a - 2. * b + c;
        if d != 0. {
            let m1 = -(b * b - a * c).sqrt();
            let m2 = -a + b;
            let v1 = -(m1 + m2) / d;
            let v2 = -(-m1 + m2) / d;
            return vec![v1, v2];
        } else if b != c && d == 0. {
            return vec![(2. * b - c) / (2. * (b - c))];
        }
        return vec![];
    } else if p.len() == 2 {
        let a = p[0];
        let b = p[1];
        if a != b {
            return vec![a / (a - b)];
        }
        return vec![];
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_extrema() {
        use super::*;

        let seg = CubicBezierSegment {
            from: vec2f!(100., 25.),
            ctrl1: vec2f!(10., 90.),
            ctrl2: vec2f!(110., 100.),
            to: vec2f!(150., 195.),
        };

        let ex = seg.extrema();
        assert_eq!(vec![0.29352384841237594, 0.39285714285714285, 0.76,], ex);
    }

    #[test]
    fn test_arc_len() {
        use super::*;

        let seg = CubicBezierSegment {
            from: vec2f!(100., 25.),
            ctrl1: vec2f!(10., 90.),
            ctrl2: vec2f!(110., 100.),
            to: vec2f!(150., 195.),
        };

        let ret = seg.arc_len(10);
        assert_eq!(183.99826172389442, ret);
    }

    #[test]
    fn test_rect_dimensions() {
        use super::*;
        let rect = Rect(vec2f!(0, 0), vec2f!(1, 1));
        assert_eq!(rect.w(), 2.);
        assert_eq!(rect.h(), 2.);
    }
}
