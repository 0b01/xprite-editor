use crate::algorithms::{
    connected_components::connected_components, perimeter::find_perimeter,
    pixel_perfect::pixel_perfect, sorter::sort_path,
};
use crate::prelude::*;
use img::GenericImageView;
use indexmap::{set::Iter, IndexSet};
use std::cmp::Ordering;
use std::fmt::{Debug, Error, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Index, Sub};
use std::f64;
use fnv::FnvBuildHasher;

#[cfg_attr(feature = "python-scripting", pyclass)]
#[derive(Copy, Clone, Eq, PartialOrd, Serialize, Deserialize, Default)]
pub struct Pixel {
    pub point: Vec2f,
    pub color: Color,
}

#[cfg(feature = "python-scripting")]
use pyo3::class::basic::PyObjectProtocol;
#[cfg(feature = "python-scripting")]
#[pyproto]
impl PyObjectProtocol for Pixel {
    fn __repr__(&'p self) -> PyResult<String> {
        Ok(format!("Pixel<({:?}), ({:?})>", self.point, self.color))
    }
}

#[cfg(feature = "python-scripting")]
#[pymethods]
impl Pixel {
    #[new]
    fn __new__(obj: &PyRawObject, point: Vec2f, color: Color) -> PyResult<()> {
        obj.init(|_| Pixel { point, color })
    }
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

impl Debug for Pixel {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "@({},{})", self.point.y, self.point.x)
    }
}

macro_rules! pixel {
    ($y:expr, $x: expr, $k: expr) => {
        Pixel {
            point: Vec2f {
                y: ($y) as f64,
                x: ($x) as f64,
            },
            color: $k,
        }
    };
}

macro_rules! pixel_xy {
    ($x:expr, $y: expr, $k: expr) => {
        Pixel {
            point: Vec2f {
                x: ($x) as f64,
                y: ($y) as f64,
            },
            color: $k,
        }
    };
}

macro_rules! pixels {
    ($($i: expr),*) => {
        {
            let mut pixs = Pixels::new();
            $(
                pixs.push($i);
            )*
            pixs
        }
    };
}

#[derive(Clone, Eq, Serialize, Deserialize, Default)]
pub struct Pixels(pub IndexSet<Pixel, FnvBuildHasher>);

impl Hash for Pixels {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for i in self.0.iter() {
            i.point.hash(state);
            i.color.hash(state);
        }
    }
}

impl PartialEq for Pixels {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Pixels {
    pub fn new() -> Self {
        Pixels(IndexSet::with_hasher(Default::default()))
    }

    pub fn from_slice(slice: &[Pixel]) -> Self {
        let mut set = IndexSet::with_hasher(Default::default());
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
    }

    pub fn sub_(&mut self, other: &Pixels) {
        self.0 = self.0.sub(&other.0)
    }

    pub fn intersection(&self, other: &Pixels) -> Pixels {
        let common: Vec<_> = self.0.intersection(&other.0).cloned().collect();
        Pixels::from_slice(&common)
    }

    pub fn push(&mut self, px: Pixel) {
        self.0.replace(px);
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

    #[allow(mutable_transmutes)]
    pub fn set_color(&mut self, color: Color) {
        for pix in self.0.iter() {
            unsafe{
                let mut p = ::std::mem::transmute::<&Pixel, &mut Pixel>(pix);
                p.color = color;
            }
        }
    }

    pub fn with_color(&mut self, color: Color) -> &Self {
        self.set_color(color);
        self
    }

    pub fn pixel_perfect(&mut self) {
        *self = pixel_perfect(self);
    }

    pub fn monotonic_sort(&mut self) {
        *self = sort_path(self).unwrap();
    }

    pub fn connected_components(&self, w: usize, h: usize) -> Vec<Pixels> {
        connected_components(self, w, h)
    }

    pub fn perimeter(&self, w: usize, h: usize) -> Pixels {
        find_perimeter(w, h, self)
    }

    pub fn bounding_rect(&self) -> Rect {
        let mut min_x = f64::MAX;
        let mut max_x = f64::MIN;
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;
        for Pixel{point: Vec2f{x, y}, ..} in self.iter() {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }
        Rect(
            Vec2f {x: min_x, y: min_y},
            Vec2f {x: max_x, y: max_y},
        )
    }

    #[deprecated]
    pub fn to_strips(&self, w: usize, h: usize) -> Vec<(usize, (usize, usize), Color)> {
        let ccs = self.connected_components(w, h);
        let mut rect_list = vec![];
        for cc in &ccs {
            let fill_col = cc[0].color;
            let mat = cc.as_bool_mat(w, h);
            for (y, row) in mat.iter().enumerate() {
                let mut init = None;
                for (x, pix) in row.iter().enumerate() {
                    match (*pix, init) {
                        (true, Some(_)) => continue,
                        (true, None) => {
                            init = Some(x);
                        }
                        (false, None) => continue,
                        (false, Some(_)) => {
                            rect_list.push((y, (init.unwrap(), x), fill_col));
                            init = None;
                        }
                    }
                }
                if let Some(init) = init {
                    rect_list.push((y, (init, w), fill_col));
                }
            }
        }
        rect_list
    }
}

impl Index<usize> for Pixels {
    type Output = Pixel;
    fn index(&self, idx: usize) -> &Pixel {
        self.0.get_index(idx).unwrap()
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

impl From<img::DynamicImage> for Pixels {
    fn from(im: img::DynamicImage) -> Pixels {
        let mut pixs = Pixels::new();
        for p in im.pixels() {
            pixs.push(pixel!(p.0, p.1, p.2.into()))
        }
        pixs
    }
}

impl Pixels {
    pub fn as_bool_mat(&self, w: usize, h: usize) -> Vec<Vec<bool>> {
        let mut arr = vec![vec![false; w]; h];
        for p in self.0.iter() {
            let Pixel { point, .. } = p;
            let Vec2f { x, y } = point;
            arr[*y as usize][*x as usize] = true;
        }
        arr
    }
    pub fn as_mat(&self, w: usize, h: usize) -> Vec<Vec<Option<Pixel>>> {
        let mut arr = vec![vec![None; w as usize]; h as usize];
        for p in self.0.iter() {
            let Pixel { point, .. } = p;
            let Vec2f { x, y } = point;
            if oob(*x, *y, w as f64, h as f64) {
                continue;
            }
            arr[*y as usize][*x as usize] = Some(*p);
        }
        arr
    }


    // // TODO:
    // pub fn as_mat(&self, bb: Rect) -> Vec<Vec<Option<Pixel>>> {
    //     let w = bb.w();
    //     let h = bb.h();
    //     let mut arr = vec![vec![None; w as usize]; h as usize];
    //     for p in self.0.iter() {
    //         let Pixel { point, .. } = p;
    //         let Vec2f { x, y } = point;
    //         if oob(*x, *y, w, h) {
    //             continue;
    //         }
    //         arr[(*y - bb.0.y) as usize][(*x - bb.0.x) as usize] = Some(*p);
    //     }
    //     arr
    // }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn as_image(&self, w: f64, h: f64, origin: (f64, f64)) -> img::DynamicImage {
        let mut rdr = ImageRenderer::new(w, h);
        for pix in &self.0 {
            let Pixel {
                point: Vec2f { x, y },
                color,
            } = pix;
            if oob(*x - origin.0, *y - origin.1, w as f64, h as f64) {
                continue;
            }
            rdr.rect(
                [*x - origin.0, *y - origin.1],
                [0., 0.],
                (*color).into(),
                true,
            );
        }
        rdr.render();
        rdr.image
    }
}

mod tests {

    #[test]
    fn test_extend() {
        use super::*;
        let mut v1 = Pixels::from_slice(&vec![
            pixel!(0., 0., Color::blue()),
            pixel!(0., 1., Color::blue()),
        ]);
        let v2 = Pixels::from_slice(&vec![pixel!(0., 1., Color::blue())]);
        v1.extend(&v2);
        let mut expected = IndexSet::new();
        expected.insert(pixel!(0., 0., Color::blue()));
        expected.insert(pixel!(0., 1., Color::blue()));
        assert_eq!(expected, v1.0);
    }

    #[test]
    fn test_extend_dup() {
        use super::*;
        let mut v1 = Pixels::from_slice(&vec![
            pixel!(0., 0., Color::red()),
            pixel!(0., 1., Color::red()),
        ]);
        let v2 = Pixels::from_slice(&vec![pixel!(0., 1., Color::blue())]);
        v1.extend(&v2);
        let mut expected = IndexSet::new();
        expected.insert(pixel!(0., 0., Color::red()));
        expected.insert(pixel!(0., 1., Color::blue()));
        assert_eq!(expected, v1.0);
    }

    #[test]
    fn test_bool_mat() {
        use super::*;
        let pixs = pixels! {
            pixel!(0,0,Color::red()),
            pixel!(0,1,Color::red()),
            // pixel!(1,0,Color::red()),
            pixel!(1,1,Color::red())
        };

        assert_eq!(
            pixs.as_bool_mat(2, 2),
            vec![vec![true, true], vec![false, true]]
        );
    }

    #[test]
    fn test_sub() {
        use super::*;
        let mut v1 = Pixels::from_slice(&vec![
            pixel!(0., 0., Color::red()),
            pixel!(0., 1., Color::red()),
        ]);
        v1.sub_(&Pixels::from_slice(&vec![pixel!(0., 1., Color::blue())]));
        assert_eq!(Pixels::from_slice(&vec![pixel!(0., 0., Color::red())]), v1);
    }

    #[test]
    fn test_intersection() {
        use super::*;
        let v1 = Pixels::from_slice(&vec![
            pixel!(0., 0., Color::red()),
            pixel!(0., 1., Color::red()),
        ]);
        let intersection =
            v1.intersection(&Pixels::from_slice(&vec![pixel!(0., 1., Color::blue())]));
        assert_eq!(
            Pixels::from_slice(&vec![pixel!(0., 1., Color::red())]),
            intersection
        );
    }

    #[test]
    fn test_to_strip() {
        use super::*;
        let pixs = pixels!(pixel!(1, 1, Color::red()));
        let strips = pixs.to_strips(3, 3);
        assert_eq!(strips, vec![(1, (1, 2), Color::red())]);

        let pixs = pixels!(
            pixel!(0, 0, Color::red()),
            pixel!(0, 1, Color::red()),
            pixel!(0, 2, Color::red())
        );
        let strips = pixs.to_strips(3, 3);
        assert_eq!(strips, vec![(0, (0, 3), Color::red())]);

        /*
         *   ###
         *   ##.
         */
        let pixs = pixels!(
            pixel!(0, 0, Color::red()),
            pixel!(0, 1, Color::red()),
            pixel!(0, 2, Color::red()),
            pixel!(1, 0, Color::red()),
            pixel!(1, 1, Color::red())
        );
        let strips = pixs.to_strips(3, 3);
        assert_eq!(
            strips,
            vec![(0, (0, 3), Color::red()), (1, (0, 2), Color::red()),]
        );
    }

    #[test]
    fn test_bounding_box() {
        use super::*;
        /*
         *   ###
         *   ##.
         */
        let pixs = pixels!(
            pixel!(0, 0, Color::red()),
            pixel!(0, 1, Color::red()),
            pixel!(0, 2, Color::red()),
            pixel!(1, 0, Color::red()),
            pixel!(1, 1, Color::red())
        );

        let bb = pixs.bounding_rect();
        assert_eq!(Rect(
            Vec2f {
                x: 0.0,
                y: 0.0
            },
            Vec2f {
                x: 2.0,
                y: 1.0
            }
        ), bb);
    }
}
