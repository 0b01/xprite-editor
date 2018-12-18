use image::GenericImage;
use num_traits::Float;
use std::cmp::Ordering;
use std::ops::{Add, AddAssign};

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
pub struct OrderedFloat<F> where F: Float {
    val: F
}

impl<F> OrderedFloat<F> where F: Float {
    pub fn as_float(&self) -> F { self.val }

    /// Try converting a Float into an OrderedFloat.
    pub fn try_from(val: F) -> Result<OrderedFloat<F>, ()> {
        if val.is_nan() { Err(()) }
        else { Ok(OrderedFloat { val: val }) }
    }
}

impl<F> Eq for OrderedFloat<F> where F: Float { }

impl<F> Ord for OrderedFloat<F> where F: Float {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self.val.is_nan(), other.val.is_nan()) {
            (false, false) => self.partial_cmp(other).unwrap(),
            _ => panic!("OrderedFloat is NaN")
        }
    }
}

impl<F> Add for OrderedFloat<F> where F: Float {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        OrderedFloat { val: self.val + other.val }
    }
}

impl<F> AddAssign for OrderedFloat<F> where F: Float {
    fn add_assign(&mut self, other: Self) {
        self.val = self.val + other.val;
    }
}

#[derive(Debug, Clone)]
/// Describes a square patch in the source image
pub struct Patch {
    /// Coordinates of the bottom-left corner of the patch
    pub coords: (u32, u32),
    /// Size of the patch in pixels
    pub size: u32
}

#[derive(Debug)]
/// Describes a rectangle in an image
pub struct Rect {
    pub coords: (u32, u32),
    pub size: (u32, u32)
}

pub fn blit_rect<I>(bottom: &mut I, top: &I, rect: &Rect, buf_coords: (u32, u32))
    where I: GenericImage
{
    for x in 0..rect.size.0 {
        for y in 0..rect.size.1 {
            bottom.put_pixel(buf_coords.0 + x, buf_coords.1 + y,
                             top.get_pixel(x + rect.coords.0, y + rect.coords.1));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryInto;

    #[test]
    fn test_ordered_float_tryfrom() {
        let f = 72.;
        let of: OrderedFloat<_> = f.try_into().unwrap();
        assert!(of.val == f);
    }
}
