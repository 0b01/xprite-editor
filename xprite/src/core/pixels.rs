use crate::prelude::*;
use std::collections::BTreeSet;
use std::slice::Iter;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Error};

#[derive(Copy, Clone, Eq, PartialOrd)]
pub struct Pixel {
    pub point: Vec2D,
    pub color: Color,
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
        write!(f, "@({},{},{:#?})", self.point.x, self.point.y, self.color)
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
pub struct Pixels(pub Vec<Pixel>, pub BTreeSet<Pixel>);

impl PartialEq for Pixels {
    fn eq(&self, other: &Self) -> bool {
        self.1.is_subset(&other.1)
        &&
        other.1.is_subset(&self.1)
    }
}

impl Pixels {

    pub fn new() -> Self {
        Pixels(Vec::new(), BTreeSet::new())
    }

    pub fn from_slice(slice: &[Pixel]) -> Self {
        let mut vec = Vec::new();
        let mut set = BTreeSet::new();
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
        for i in &other.1 {
            self.1.replace(*i);
        }
        self.0 = self.1.iter().cloned().collect();
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
        let color = *color;
        self.0 = self.0
            .iter()
            .map(|Pixel {point,..}| { Pixel{ point: *point, color } })
            .collect::<Vec<_>>();
    }

    pub fn with_color(&mut self, color: &Color) -> &Self {
        let color = *color;
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
        self.1.len()
    }
}


mod tests {

    #[test]
    fn test_extend() {
        use crate::prelude::*;
        let mut v1 = Pixels::from_slice(&vec![
            pixel!(0.,0., Color::red()),
            pixel!(0.,1., Color::red())]
        );
        let v2 = Pixels::from_slice(&vec![
            pixel!(0.,1., Color::red())
        ]);
        v1.extend(&v2);
        assert_eq!(vec![
            pixel!(0.,0., Color::red()),
            pixel!(0.,1., Color::red()),
        ], v1.0);
    }

    #[test]
    fn test_extend_dup() {
        use crate::prelude::*;
        let mut v1 = Pixels::from_slice(&vec![
            pixel!(0.,0., Color::red()),
            pixel!(0.,1., Color::red())]
        );
        let v2 = Pixels::from_slice(&vec![
            pixel!(0.,1., Color::blue())
        ]);
        v1.extend(&v2);
        assert_eq!(vec![
            pixel!(0.,0., Color::red()),
            pixel!(0.,1., Color::blue()),
        ], v1.0);
    }
}