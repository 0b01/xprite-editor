use crate::prelude::*;
use indexmap::IndexMap;
use std::borrow::Cow;

type PaletteGroup = IndexMap<&'static str, Color>;

#[derive(Default, Debug)]
pub struct PaletteManager {
    pub palettes: IndexMap<String, PaletteGroup>,
}

impl PaletteManager {
    pub fn new() -> Self {
        let mut palettes = IndexMap::new();
        palettes.insert("pico8".to_owned(), pico8());
        Self {
            palettes,
        }
    }
    pub fn get(&self, name: &str) -> Option<&PaletteGroup> {
        self.palettes.get(name)
    }
}

fn pico8() -> PaletteGroup {
    let mut colors = IndexMap::new();
    colors.insert("black", Color{r: 0, g: 0, b: 0, a: 255});
    colors.insert("dark-blue", Color{r: 29, g: 43, b: 83, a: 255});
    colors.insert("dark-purple", Color{r: 126, g: 37, b: 83, a: 255});
    colors.insert("dark-green", Color{r: 0, g: 135, b: 81, a: 255});
    colors.insert("brown", Color{r: 171, g: 82, b: 54, a: 255});
    colors.insert("dark-gray", Color{r: 95, g: 87, b: 79, a: 255});
    colors.insert("light-gray", Color{r: 194, g: 195, b: 199, a: 255});
    colors.insert("white", Color{r: 255, g: 241, b: 232, a: 255});
    colors.insert("red", Color{r: 255, g: 0, b: 77, a: 255});
    colors.insert("orange", Color{r: 255, g: 163, b: 0, a: 255});
    colors.insert("yellow", Color{r: 255, g: 236, b: 39, a: 255});
    colors.insert("green", Color{r: 0, g: 228, b: 54, a: 255});
    colors.insert("blue", Color{r: 41, g: 173, b: 255, a: 255});
    colors.insert("indigo", Color{r: 131, g: 118, b: 156, a: 255});
    colors.insert("pink", Color{r: 255, g: 119, b: 168, a: 255});
    colors.insert("peach", Color{r: 255, g: 204, b: 170, a: 255});
    colors
}
