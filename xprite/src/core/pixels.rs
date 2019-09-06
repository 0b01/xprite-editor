use crate::algorithms::{
    connected_components::connected_components,
    perimeter::{find_outline, find_perimeter},
    pixel_perfect::{pixel_antiperfect, pixel_perfect},
    rotsprite::rotsprite,
    selective_antialias::selective_antialias,
    sorter::sort_path,
};
use crate::prelude::*;
use fnv::FnvBuildHasher;
use img::GenericImageView;
use indexmap::{set::Iter, IndexSet};
use std::cmp::Ordering;
use std::f64;
use std::fmt::{Debug, Error, Formatter};
use std::hash::{Hash, Hasher};
use std::iter::FromIterator;
use std::ops::{Index, Sub};

#[cfg_attr(feature = "python-scripting", pyclass)]
#[derive(Copy, Clone, Eq, Default)]
pub struct Pixel {
    pub point: Vec2f,
    pub color: Color,
}

impl PartialOrd for Pixel {
    fn partial_cmp(&self, other: &Pixel) -> Option<Ordering> {
        self.point.partial_cmp(&other.point)
    }
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

impl Pixel {
    pub fn as_channel_r(&self, xpr: Option<&Xprite>) -> Option<Pixel> {
        let mut ret = *self;
        let mut col = ret.color.to_rgba(xpr)?;
        col.g = 0;
        col.b = 0;
        ret.color = Color::Rgba(col);
        Some(ret)
    }

    pub fn as_channel_g(&self, xpr: Option<&Xprite>) -> Option<Pixel> {
        let mut ret = *self;
        let mut col = ret.color.to_rgba(xpr)?;
        col.r = 0;
        col.b = 0;
        ret.color = Color::Rgba(col);
        Some(ret)
    }

    pub fn as_channel_b(&self, xpr: Option<&Xprite>) -> Option<Pixel> {
        let mut ret = *self;
        let mut col = ret.color.to_rgba(xpr)?;
        col.r = 0;
        col.g = 0;
        ret.color = Color::Rgba(col);
        Some(ret)
    }

    pub fn rotate(&self, pivot: Vec2f, angle: f64) -> Pixel {
        let c = angle.cos();
        let s = angle.sin();

        // dbg!((c,s));
        let px = self.point.x - pivot.x + 0.5;
        let py = self.point.y - pivot.y + 0.5;
        // dbg!((px, py));
        // dbg!(pivot);

        let mut point = Vec2f {
            x: c * px + s * py,
            y: -s * px + c * py,
        };

        // dbg!(point);
        point += pivot;
        point.x = point.x.floor();
        point.y = point.y.floor();

        return Pixel { point, color: self.color };
    }

    #[deprecated]
    pub fn with_color(&self, color: Color) -> Pixel {
        let mut ret = *self;
        ret.color = color;
        ret
    }

    #[allow(mutable_transmutes)]
    pub fn set_color(&self, color: Color) {
        unsafe {
            let mut p = ::std::mem::transmute::<&Pixel, &mut Pixel>(self);
            p.color = color;
        }
    }

    pub fn dir(&self, other: &Pixel) -> bool {
        if self.point.x == other.point.x {
            return self.point.y > other.point.y;
        } else {
            return self.point.x > other.point.x;
        }
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
        if false {
            write!(f, "@({},{}){:?}", self.point.y, self.point.x, self.color)
        } else {
            write!(f, "@({},{})", self.point.y, self.point.x)
        }
    }
}

#[macro_export]
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

    ($vec2f: expr, $k: expr) => {
        Pixel { point: $vec2f, color: $k }
    };
}

#[macro_export]
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

#[macro_export]
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

#[derive(Clone, Eq)]
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

impl Default for Pixels {
    fn default() -> Self {
        Self::new()
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

    /// query pixel in the collection
    /// returns None if it does not exist
    pub fn get_pixel(&self, y: isize, x: isize) -> Option<Pixel> {
        if y < 0 || x < 0 {
            return None;
        }
        self.0.get(&pixel!(y, x, Color::red())).cloned()
    }

    pub fn rotate(&self, pivot: Vec2f, angle: f64) -> Pixels {
        Pixels(self.0.iter().map(|p| p.rotate(pivot, angle)).collect())
    }

    pub fn rotsprite(&self, pivot: Vec2f, angle: f64) -> Pixels {
        rotsprite(&self, angle, pivot)
    }

    pub fn extend<'a, 'b>(&'a mut self, other: &'b Pixels) {
        for i in other.0.iter() {
            self.0.replace(*i);
        }
    }

    pub fn sub_mut(&mut self, other: &Pixels) {
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

    pub fn set_color(&mut self, color: Color) {
        for pix in self.0.iter() {
            pix.set_color(color);
        }
    }

    #[deprecated]
    pub fn with_color(&mut self, color: Color) -> &Self {
        self.set_color(color);
        self
    }

    pub fn pixel_perfect(&mut self) {
        pixel_perfect(self);
    }

    pub fn pixel_antiperfect(&mut self) {
        *self = pixel_antiperfect(self);
    }

    pub fn monotonic_sort(&mut self) {
        *self = sort_path(self).unwrap();
    }

    pub fn selective_antialias(&mut self, k: f64, alt_color: Color) {
        selective_antialias(self, k, alt_color)
    }

    pub fn connected_components(&self, w: usize, h: usize) -> Vec<Pixels> {
        connected_components(self, w, h)
    }

    pub fn perimeter(&self, w: usize, h: usize) -> Pixels {
        find_perimeter(w, h, self)
    }

    pub fn outline(&self) -> Vec<MarqueePixel> {
        let bb = self.bounding_rect();
        find_outline(bb, self)
    }

    pub fn bounding_rect(&self) -> Rect {
        let mut min_x = f64::MAX;
        let mut max_x = f64::MIN;
        let mut min_y = f64::MAX;
        let mut max_y = f64::MIN;
        for Pixel { point: Vec2f { x, y }, .. } in self.iter() {
            min_x = min_x.min(*x);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            max_y = max_y.max(*y);
        }
        Rect(Vec2f { x: min_x, y: min_y }, Vec2f { x: max_x, y: max_y })
    }

    pub fn separate_rgb(&self, xpr: Option<&Xprite>) -> Option<[Pixels; 3]> {
        let mut r = Pixels::new();
        let mut g = Pixels::new();
        let mut b = Pixels::new();
        for p in self.iter() {
            r.push(p.as_channel_r(xpr)?);
            g.push(p.as_channel_g(xpr)?);
            b.push(p.as_channel_b(xpr)?);
        }
        Some([r, g, b])
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
            pixs.push(pixel_xy!(p.0, p.1, p.2.into()))
        }
        pixs
    }
}

impl FromIterator<Pixel> for Pixels {
    fn from_iter<I: IntoIterator<Item = Pixel>>(iter: I) -> Self {
        let mut c = Pixels::new();
        for i in iter {
            c.push(i);
        }
        c
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
            let Pixel { point: Vec2f { x, y }, .. } = p;
            if oob(*x, *y, w as f64, h as f64) {
                continue;
            }
            arr[*y as usize][*x as usize] = Some(*p);
        }
        arr
    }

    pub fn retain_in_rect_mut(&mut self, bb: Rect) {
        let w = bb.w();
        let h = bb.h();
        self.0 = self // TODO: use IndexMap::retain
            .0
            .iter()
            .filter_map(|p| {
                let Pixel { point: Vec2f { x, y }, .. } = p;
                if oob(*x - bb.0.x, *y - bb.0.y, w as f64, h as f64) {
                    None
                } else {
                    Some(*p)
                }
            })
            .collect();
    }

    pub fn retain_in_bound_mut(&mut self, w: usize, h: usize) {
        self.0 = self // TODO: use IndexMap::retain
            .0
            .iter()
            .filter_map(|p| {
                let Pixel { point: Vec2f { x, y }, .. } = p;
                if oob(*x, *y, w as f64, h as f64) {
                    None
                } else {
                    Some(*p)
                }
            })
            .collect();
    }

    /// shift all pixels in the matrix in accordance to bounding box
    pub fn as_mat_bb(&self, bb: Rect) -> Vec<Vec<Option<Pixel>>> {
        let w = bb.w();
        let h = bb.h();
        let mut arr = vec![vec![None; w as usize]; h as usize];
        for p in self.0.iter() {
            let Pixel { point, color } = p;
            let Vec2f { x, y } = point;
            let y_ = (*y - bb.0.y) as usize;
            let x_ = (*x - bb.0.x) as usize;
            arr[y_][x_] = Some(pixel!(y_, x_, *color));
        }
        arr
    }

    pub fn shifted(&self, d: Vec2f) -> Pixels {
        let mut ret = Pixels::new();
        for i in self.iter() {
            let mut shifted_i = i.clone();
            shifted_i.point.x += d.x;
            shifted_i.point.y += d.y;
            ret.push(shifted_i);
        }
        ret
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn to_rgba(&self, xpr: Option<&Xprite>) -> Option<Pixels> {
        self.iter()
            .map(|p| {
                let mut ret = *p;
                p.color.to_rgba(xpr).map(|c| {
                    ret.color = Color::Rgba(c);
                    ret
                })
            })
            .collect()
    }

    pub fn as_image(&self, bb: Rect, xpr: Option<&Xprite>) -> Option<img::DynamicImage> {
        let w = bb.w() as f64;
        let h = bb.h() as f64;
        let origin = bb.0;

        let bg = xpr.map(|i| i.canvas.bg).unwrap_or(Color::grey());
        let mut rdr = ImageRenderer::new(bg, w, h);
        for pix in &self.0 {
            let Pixel { point: Vec2f { x, y }, color } = pix;
            if oob(*x - origin.x, *y - origin.y, w as f64, h as f64) {
                continue;
            }
            rdr.pixel(*x - origin.x, *y - origin.y, (*color).to_rgba(xpr)?.into(), true);
        }
        rdr.render(xpr)?;
        Some(rdr.image)
    }

    pub fn save(&self, output: &str, xpr: Option<&Xprite>) {
        let bb = self.bounding_rect();
        if let Some(img) = self.as_image(bb, xpr) {
            img.save(output).unwrap();
        }
    }
}

impl Pixels {
    pub fn to_ase_pixels(&self, xpr: Option<&Xprite>) -> Option<ase::Pixels> {
        let bb = self.bounding_rect();

        self.as_mat_bb(bb)
            .into_iter()
            .flatten()
            .map(|op| match op {
                Some(Pixel { color: c, .. }) => c.to_rgba(xpr).map(|rgba| rgba.into()),
                None => Some(XpriteRgba::default().into()),
            })
            .collect::<Option<Vec<_>>>()
            .map(|v| ase::Pixels::RGBA(v))
    }

    pub fn from_ase_pixels(ase_pixs: &ase::Pixels, bb: Rect) -> Self {
        let x0 = bb.0.x as i32;
        let y0 = bb.0.y as i32;
        let h = bb.w() as i32; // TODO: BUG: reverse this
        let w = bb.h() as i32;
        let mut pixs = Pixels::new();
        if let ase::Pixels::RGBA(vec) = ase_pixs {
            assert_eq!(vec.len() as i32, w * h);
            for (color, i) in vec.iter().zip(0..) {
                if color.a == 0 {
                    continue;
                } // skip transparent pixels
                let nth_row = i / w;
                let nth_col = i % w;
                let y = y0 + nth_row;
                let x = x0 + nth_col;
                pixs.push(pixel!(y, x, (*color).into()));
            }
        }

        pixs
    }
}

mod tests {

    #[test]
    fn test_extend() {
        use super::*;
        let mut v1 = Pixels::from_slice(&vec![pixel!(0., 0., Color::blue()), pixel!(0., 1., Color::blue())]);
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
        let mut v1 = Pixels::from_slice(&vec![pixel!(0., 0., Color::red()), pixel!(0., 1., Color::red())]);
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

        assert_eq!(pixs.as_bool_mat(2, 2), vec![vec![true, true], vec![false, true]]);
    }

    #[test]
    fn test_sub() {
        use super::*;
        let mut v1 = Pixels::from_slice(&vec![pixel!(0., 0., Color::red()), pixel!(0., 1., Color::red())]);
        v1.sub_mut(&Pixels::from_slice(&vec![pixel!(0., 1., Color::blue())]));
        assert_eq!(Pixels::from_slice(&vec![pixel!(0., 0., Color::red())]), v1);
    }

    #[test]
    fn test_intersection() {
        use super::*;
        let v1 = Pixels::from_slice(&vec![pixel!(0., 0., Color::red()), pixel!(0., 1., Color::red())]);
        let intersection = v1.intersection(&Pixels::from_slice(&vec![pixel!(0., 1., Color::blue())]));
        assert_eq!(Pixels::from_slice(&vec![pixel!(0., 1., Color::red())]), intersection);
    }

    #[test]
    fn test_to_strip() {
        use super::*;
        let pixs = pixels!(pixel!(1, 1, Color::red()));
        let strips = pixs.to_strips(3, 3);
        assert_eq!(strips, vec![(1, (1, 2), Color::red())]);

        let pixs = pixels!(pixel!(0, 0, Color::red()), pixel!(0, 1, Color::red()), pixel!(0, 2, Color::red()));
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
        assert_eq!(strips, vec![(0, (0, 3), Color::red()), (1, (0, 2), Color::red()),]);
    }

    #[test]
    fn test_as_mat_bb() {
        use super::*;
        let pixs = pixels!(pixel!(0, 1, Color::red()), pixel!(0, 2, Color::red()));
        let bb = pixs.bounding_rect();
        let ret = pixs.as_mat_bb(bb);

        assert_eq!(vec![vec![Some(pixel!(0, 0, Color::red())), Some(pixel!(0, 1, Color::red())),]], ret);
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
        assert_eq!(Rect(Vec2f { x: 0.0, y: 0.0 }, Vec2f { x: 2.0, y: 1.0 }), bb);
    }

    #[test]
    fn test_as_image() {
        use super::*;
        let pixs = pixels!(pixel!(0., 0., Color::red()), pixel!(1., 1., Color::red()));
        let img = pixs.as_image(Rect(Vec2f { x: 0., y: 0. }, Vec2f { x: 2., y: 2. }), None).unwrap();
        assert_eq!(
            img.raw_pixels(),
            vec![255, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255, 0, 0, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_as_image_one_pixel() {
        use super::*;
        let pixs = pixels!(pixel!(1, 1, Color::red()));
        let bb = pixs.bounding_rect();
        let img = pixs.as_image(bb, None).unwrap();
        assert_eq!(img.raw_pixels(), vec![255, 0, 0, 255]);
    }

    #[test]
    fn test_separate_rgb() {
        use super::*;
        let pixs = pixels!(pixel!(0, 0, Color::white()));
        let ret = pixs.separate_rgb(None).unwrap();
        assert_eq!(
            ret,
            [
                pixels!(pixel!(0, 0, Color::red())),
                pixels!(pixel!(0, 0, Color::green())),
                pixels!(pixel!(0, 0, Color::blue())),
            ]
        );
    }

    #[test]
    fn test_pixel_rotation() {
        use super::*;
        macro_rules! test_rotated {
            (
                ($a:expr, $b:expr), // from
                ($c:expr, $d:expr), // pivot
                $e:expr,            // angle
                ($f:expr, $g:expr)  // to
            ) => {
                assert_eq!(
                    pixel!($a, $b, Color::red()).rotate(
                        Vec2f {
                            y: $c as f64,
                            x: $d as f64
                        },
                        $e
                    ),
                    pixel!($f, $g, Color::red())
                );
            };
        }

        test_rotated!((0, 2), (0, 1), -PI / 2., (1, 0));
        test_rotated!((0, 3), (0, 1), -PI / 2., (2, 0));
        test_rotated!((1, 2), (1, 1), -PI / 2., (2, 0));
        test_rotated!((1, 3), (1, 2), -PI / 2., (2, 1));
        test_rotated!((3, 5), (3, 3), -PI, (2, 0));

        test_rotated!((3, 3), (2, 3), PI, (0, 2));
        test_rotated!((3, 3), (2, 3), -PI, (0, 2));
    }

    #[test]
    fn test_get_pixel() {
        use super::*;
        let pixs = pixels!(pixel!(0, 0, Color::black()), pixel!(0, 1, Color::blue()));
        assert_eq!(pixs.get_pixel(0, 0), Some(pixel!(0, 0, Color::black())));
        assert_eq!(pixs.get_pixel(0, 0).map(|i| i.color), Some(Color::black()));
        assert_eq!(pixs.get_pixel(0, 1), Some(pixel!(0, 1, Color::blue())));
        assert_eq!(pixs.get_pixel(0, 1).map(|i| i.color), Some(Color::blue()));
    }

    #[test]
    fn test_retain_in_rect() {
        use super::*;
        let mut pixs = pixels!(pixel!(0, 0, Color::black()), pixel!(0, 1, Color::blue()), pixel!(1, 1, Color::blue()));

        pixs.retain_in_rect_mut(Rect(Vec2f { x: 0., y: 0. }, Vec2f { x: 1., y: 1. }));
        assert_eq!(pixs, pixels!(pixel!(0, 0, Color::black()), pixel!(0, 1, Color::blue()), pixel!(1, 1, Color::blue())));

        pixs.retain_in_rect_mut(Rect(Vec2f { x: 0., y: 0. }, Vec2f { x: 0., y: 0. }));
        assert_eq!(pixs, pixels!(pixel!(0, 0, Color::black())));

    }
}
