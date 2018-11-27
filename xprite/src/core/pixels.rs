use crate::prelude::*;
use std::collections::HashSet;
use std::slice::Iter;
use std::hash::{Hash, Hasher};
use std::fmt::{Debug, Formatter, Error};

#[derive(Copy, Clone, Eq)]
pub struct Pixel {
    pub point: Point2D<f32>,
    pub color: ColorOption,
}

impl Pixel {
    pub fn with_color(&self, col: Color) -> Self {
        let mut self_ = self.clone();
        self_.color = ColorOption::Set(col);
        self_
    }
}

impl PartialEq for Pixel {
    fn eq(&self, other: &Self) -> bool {
        (self.point == other.point) &&
        (self.color == other.color)
    }
}

impl Debug for Pixel {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "@({},{},{:#?})", self.point.x, self.point.y, self.color)
    }
}

impl Hash for Pixel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.point.hash(state);
        self.color.hash(state);
    }
}

macro_rules! pixel {
    ($i:expr, $j: expr) => {
        Pixel {
            point: Point2D::new($i as f32, $j as f32),
            color: ColorOption::Unset,
        }
    };
    ($i:expr, $j: expr, $k: expr) => {
        Pixel {
            point: Point2D::new($i as f32, $j as f32),
            color: ColorOption::Set($k),
        }
    };
}


#[derive(Clone, Eq)]
/// dual repr
pub struct Pixels(pub Vec<Pixel>, pub HashSet<Pixel>);

impl PartialEq for Pixels {
    fn eq(&self, other: &Self) -> bool {
        self.1.is_subset(&other.1)
        &&
        other.1.is_subset(&self.1)
    }
}

impl Pixels {
    pub fn new() -> Self {
        Pixels(Vec::new(), HashSet::new())
    }
    pub fn from_slice(slice: &[Pixel]) -> Self {
        let mut vec = Vec::new();
        let mut set = HashSet::new();
        for p in slice.iter() {
            if set.contains(p) {
                continue;
            }
            vec.push(*p);
            set.insert(*p);
        }
        Pixels(vec, set)
    }
    pub fn extend(&mut self, other: &Pixels) {
        self.extend_vec(&other.0);
    }
    pub fn extend_vec(&mut self, pxs: &[Pixel]) {
        for p in pxs.iter() {
            if self.1.contains(&p) {
                continue;
            }
            self.0.push(*p);
            self.1.insert(*p);
        }
    }
    pub fn push(&mut self, px: Pixel) {
        if !self.1.contains(&px) {
            self.0.push(px);
            self.1.insert(px);
        }
    }
    pub fn contains(&mut self, px: &Pixel) -> bool {
        self.1.contains(px)
    }
    pub fn clear(&mut self) {
        self.0.clear();
        self.1.clear();
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
    pub fn with_color(&mut self, color: &Color) -> &Self {
        let color = ColorOption::Set(*color);
        self.0 = self.0
            .iter()
            .map(|Pixel {point,..}| { Pixel{ point: *point, color } })
            .collect::<Vec<_>>();
        self
    }
}

impl Debug for Pixels {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self.0.len() {
            0 => write!(f, "Pixels[0](empty)"),
            1 => write!(f, "Pixels[1]([{:?}])", self.0[0]),
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
            let Point2D {x, y} = point;
            arr[*x as usize][*y as usize] = true;
        }
        arr
    }

    pub fn as_arr(&self, w: usize, h: usize) -> Vec<Vec<Pixel>> {
        let mut arr = vec![];
        for i in 0..h {
            let mut row = vec![];
            for j in 0..w {
                row.push(pixel!(i as f32, j as f32));
            }
            arr.push(row);
        }
        for p in self.0.iter() {
            let Pixel{point, ..} = p;
            let Point2D {x, y} = point;
            arr[*x as usize][*y as usize] = p.clone();
        }
        arr
    }
}


mod tests {
    #[test]
    fn test_extend() {
        use crate::prelude::*;
        let mut v1 = Pixels::from_slice(&vec![pixel!(0.,0.), pixel!(0.,1.)]);
        let v2 = Pixels::from_slice(&vec![pixel!(0.,1.)]);
        v1.extend(&v2);
        assert_eq!(Pixels::from_slice(&vec![pixel!(0.,0.), pixel!(0.,1.)]), v1);
    }
}