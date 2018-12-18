use crate::prelude::*;
use std::ops::Sub;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};
use std::fmt::{Debug, Formatter, Error};
use indexmap::{IndexSet, set::Iter};

#[derive(Copy, Clone, Eq, PartialOrd)]
pub struct Pixel {
    pub point: Vec2D,
    pub color: Color,
}

impl Hash for Pixel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.point.hash(state)
    }
}

impl Ord for Pixel {
    fn cmp(&self, other: &Pixel) -> Ordering {
        self.point.cmp(&other.point)
    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Pixel) -> bool {
        self.point == other.point
    }
}

impl Pixel {
    pub fn with_color(&self, col: Color) -> Self {
        let mut self_ = self.clone();
        self_.color = col;
        self_
    }
}

impl Debug for Pixel {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "@({},{})", self.point.x, self.point.y)
    }
}

macro_rules! pixel {
    ($i:expr, $j: expr, $k: expr) => {
        Pixel {
            point: Vec2D::new(($i) as f32, ($j) as f32),
            color: $k,
        }
    };
}


#[derive(Clone, Eq)]
/// dual repr
pub struct Pixels(pub IndexSet<Pixel>);

impl PartialEq for Pixels {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Pixels {

    pub fn new() -> Self {
        Pixels(IndexSet::new())
    }

    pub fn from_slice(slice: &[Pixel]) -> Self {
        let mut set = IndexSet::new();
        for p in slice.iter() {
            if set.contains(p) {
                continue;
            }
            set.insert(*p);
        }
        Pixels(set)
    }

    pub fn extend(&mut self, other: &Pixels) {
        for i in other.0.iter() {
            self.0.replace(*i);
        }
        self.0 = self.0.iter().cloned().collect();
    }

    pub fn sub(&mut self, other: &Pixels) {
        self.0 = self.0.sub(&other.0)
    }

    pub fn push(&mut self, px: Pixel) {
        if !self.0.contains(&px) {
            self.0.insert(px);
        }
    }

    pub fn contains(&mut self, px: &Pixel) -> bool {
        self.0.contains(px)
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn iter(&self) -> Iter<Pixel> {
        self.0.iter()
    }

    pub fn set_color(&mut self, color: &Color) {
        let color = *color;
        self.0 = self.0
            .iter()
            .map(|Pixel {point,..}| { Pixel{ point: *point, color } })
            .collect();
    }

    pub fn with_color(&mut self, color: &Color) -> &Self {
        let color = *color;
        self.0 = self.0
            .iter()
            .map(|Pixel {point,..}| { Pixel{ point: *point, color } })
            .collect();
        self
    }

}

impl Debug for Pixels {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.0.len() {
            0 => write!(f, "Pixels[0](empty)"),
            1 => write!(f, "Pixels[1]([{:?}])", self.0.iter().next().unwrap()),
            // _ => write!(f, "Pixels[{}]([{:?}..{:?}])", self.0.len(), self.0[0], self.0.last().unwrap()),
            _ => write!(f, "Pixels[{}]([{:?}])", self.0.len(), self.0),
        }
    }
}

impl From<Pixel> for Pixels {
    fn from(p: Pixel) -> Pixels {
        let mut pix = Pixels::new();
        pix.push(p);
        pix
    }
}

impl Pixels {
    pub fn as_bool_arr(&self, w: usize, h: usize) -> Vec<Vec<bool>> {
        let mut arr = vec![];
        for _i in 0..h {
            let mut row = vec![];
            for _j in 0..w {
                row.push(false);
            }
            arr.push(row);
        }
        for p in self.0.iter() {
            let Pixel{point, ..} = p;
            let Vec2D {x, y} = point;
            arr[*x as usize][*y as usize] = true;
        }
        arr
    }

    pub fn as_arr(&self, w: usize, h: usize) -> Vec<Vec<Option<Pixel>>> {
        let mut arr = vec![];
        for i in 0..h {
            let mut row = vec![];
            for j in 0..w {
                row.push(None);
            }
            arr.push(row);
        }
        for p in self.0.iter() {
            let Pixel{point, ..} = p;
            let Vec2D {x, y} = point;
            arr[*x as usize][*y as usize] = Some(p.clone());
        }
        arr
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}



mod tests {

    #[test]
    fn test_extend() {
        use super::*;
        let mut v1 = Pixels::from_slice(&vec![
            pixel!(0.,0., Color::blue()),
            pixel!(0.,1., Color::blue())]
        );
        let v2 = Pixels::from_slice(&vec![
            pixel!(0.,1., Color::blue())
        ]);
        v1.extend(&v2);
        let mut expected = IndexSet::new();
        expected.insert(pixel!(0.,0., Color::blue()));
        expected.insert(pixel!(0.,1., Color::blue()));
        assert_eq!(expected, v1.0);
    }

    #[test]
    fn test_extend_dup() {
        use super::*;
        let mut v1 = Pixels::from_slice(&vec![
            pixel!(0.,0., Color::red()),
            pixel!(0.,1., Color::red())]
        );
        let v2 = Pixels::from_slice(&vec![
            pixel!(0.,1., Color::blue())
        ]);
        v1.extend(&v2);
        let mut expected = IndexSet::new();
        expected.insert(pixel!(0.,0., Color::red()));
        expected.insert(pixel!(0.,1., Color::blue()));
        assert_eq!(expected, v1.0);
    }
}