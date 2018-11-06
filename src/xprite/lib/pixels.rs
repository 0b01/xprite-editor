use xprite::prelude::*;

use std::collections::HashSet;
use std::collections::hash_set::Iter;

#[derive(Clone, Debug)]
pub struct Pixels(pub HashSet<Pixel>);
impl Pixels {
    pub fn new() -> Self {
        Pixels(HashSet::new())
    }
    pub fn from_slice(slice: &[Pixel]) -> Self {
        let mut set = HashSet::new();
        for i in slice.iter() {
            set.insert(*i);
        }
        Pixels(set)
    }
    pub fn extend(&mut self, other: &Pixels) {
        self.0.extend(&other.0)
    }
    pub fn remove(&mut self, px: &Pixel) {
        self.0.remove(px);
    }
    pub fn insert(&mut self, px: Pixel) {
        self.0.insert(px);
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
            .collect::<HashSet<_>>();
    }
}
