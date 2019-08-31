use crate::prelude::*;
use image::{self, GenericImageView};
use indexmap::IndexMap;
use natord;
use std::{fs, io, path};

type PaletteGroup = IndexMap<String, Color>;

#[derive(Debug)]
pub struct PaletteManager {
    pub palettes: IndexMap<String, PaletteGroup>,
}

impl PaletteManager {
    pub fn new() -> io::Result<Self> {
        let mut palettes = IndexMap::new();
        palettes.insert("pico8".to_owned(), pico8());

        if cfg!(not(wasm32)) {
            let dir = "config/palettes";
            let dir_entries = fs::read_dir(dir);
            if dir_entries.is_err() {
                return Ok(Self { palettes });
            }
            let mut entries: Vec<_> = dir_entries?.map(|r| r.unwrap()).collect();
            entries.sort_by(|dir1, dir2| natord::compare(dir1.path().to_str().unwrap(), dir2.path().to_str().unwrap()));
            for entry in &entries {
                let path = entry.path();
                let palname = path.file_stem().expect("filestem").to_str().expect("filestem to_str").to_owned();
                let pal = match path.extension().expect("extension").to_str().expect("to_str") {
                    "hex" => get_palette_hex(&path)?,
                    "png" => get_palette_png(&path)?,
                    _ => continue,
                };
                palettes.insert(palname, pal);
            }
        }

        Ok(Self { palettes })
    }
    pub fn get(&self, name: &str) -> Option<&PaletteGroup> {
        self.palettes.get(name)
    }
    pub fn get_mut(&mut self, name: &str) -> Option<&mut PaletteGroup> {
        self.palettes.get_mut(name)
    }
}

fn pico8() -> PaletteGroup {
    let mut colors = IndexMap::new();
    colors.insert("black".to_owned(), Color { r: 0, g: 0, b: 0, a: 255 });
    colors.insert("dark-blue".to_owned(), Color { r: 29, g: 43, b: 83, a: 255 });
    colors.insert("dark-purple".to_owned(), Color { r: 126, g: 37, b: 83, a: 255 });
    colors.insert("dark-green".to_owned(), Color { r: 0, g: 135, b: 81, a: 255 });
    colors.insert("brown".to_owned(), Color { r: 171, g: 82, b: 54, a: 255 });
    colors.insert("dark-gray".to_owned(), Color { r: 95, g: 87, b: 79, a: 255 });
    colors.insert(
        "light-gray".to_owned(),
        Color {
            r: 194,
            g: 195,
            b: 199,
            a: 255,
        },
    );
    colors.insert(
        "white".to_owned(),
        Color {
            r: 255,
            g: 241,
            b: 232,
            a: 255,
        },
    );
    colors.insert("red".to_owned(), Color { r: 255, g: 0, b: 77, a: 255 });
    colors.insert("orange".to_owned(), Color { r: 255, g: 163, b: 0, a: 255 });
    colors.insert("yellow".to_owned(), Color { r: 255, g: 236, b: 39, a: 255 });
    colors.insert("green".to_owned(), Color { r: 0, g: 228, b: 54, a: 255 });
    colors.insert("blue".to_owned(), Color { r: 41, g: 173, b: 255, a: 255 });
    colors.insert(
        "indigo".to_owned(),
        Color {
            r: 131,
            g: 118,
            b: 156,
            a: 255,
        },
    );
    colors.insert(
        "pink".to_owned(),
        Color {
            r: 255,
            g: 119,
            b: 168,
            a: 255,
        },
    );
    colors.insert(
        "peach".to_owned(),
        Color {
            r: 255,
            g: 204,
            b: 170,
            a: 255,
        },
    );
    colors
}

fn get_palette_hex(p: &path::PathBuf) -> io::Result<PaletteGroup> {
    let mut colors = IndexMap::new();
    let cols = fs::read_to_string(p)?;
    for col in cols.lines() {
        let color = Color::from_hex(&col[1..]).expect(&format!("Cannot decode hex in file {:?}", p));
        colors.insert(col.to_owned(), color);
    }
    Ok(colors)
}

fn get_palette_png(p: &path::PathBuf) -> io::Result<PaletteGroup> {
    let mut colors = IndexMap::new();
    let img = image::open(p).unwrap();
    for pix in img.pixels() {
        let color = pix.2;
        let mycolor = Color::new(color[0], color[1], color[2]);
        colors.insert(format!("color##{},{}", pix.0, pix.1), mycolor);
    }
    Ok(colors)
}
