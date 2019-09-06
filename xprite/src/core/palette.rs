use crate::prelude::*;
use image::{self, GenericImageView};
use indexmap::IndexMap;
use natord;
use std::{fs, io, path};
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct PaletteGroup {
    pub colors: Rc<RefCell<IndexMap<String, Color>>>,
    pub idx: usize,
}


#[derive(Debug)]
pub struct PaletteManager {
    /// usize is for the index of the selected color within the palette
    pub palettes: IndexMap<String, PaletteGroup>,
    pub selected_palette_idx: usize,
}

impl PaletteManager {
    pub fn new() -> io::Result<Self> {
        let mut palettes = IndexMap::new();
        palettes.insert("pico8".to_owned(), pico8());

        if cfg!(not(wasm32)) {
            let dir = "config/palettes";
            let dir_entries = fs::read_dir(dir);
            if dir_entries.is_err() {
                return Ok(Self {
                    palettes,
                    selected_palette_idx: Default::default(),
                });
            }
            let mut entries: Vec<_> = dir_entries?.map(|r| r.unwrap()).collect();
            entries.sort_by(|dir1, dir2| natord::compare(dir1.path().to_str().unwrap(), dir2.path().to_str().unwrap()));
            for entry in &entries {
                let path = entry.path();
                let palette_name = path.file_stem().expect("file_stem").to_str().expect("file_stem to_str").to_owned();
                let pal = match path.extension().expect("extension").to_str().expect("to_str") {
                    "hex" => get_palette_hex(&path)?,
                    "png" => get_palette_png(&path)?,
                    _ => continue,
                };
                palettes.insert(palette_name, pal);
            }
        }

        Ok(Self {
            palettes,
            selected_palette_idx: Default::default(),
        })
    }

    fn find_color(&self, color: Color) -> Option<usize> {
        use itertools::Itertools;
        self.current_palette()
            .colors
            .borrow()
            .iter()
            .find_position(|&(_k, v)| unsafe { v.as_rgba() == color.as_rgba() })
            .map(|i| i.0)
    }

    pub fn set_color(&mut self, color: Color) {
        println!("{:#?}", color);
        let idx = match color {
            Color::Indexed(i) => i,
            Color::Rgba(_rgba) => {
                if let Some(idx) = self.find_color(color) {
                    idx
                } else {
                    let pal = self.current_palette();
                    let mut pal_ = pal.colors.borrow_mut();
                    let len = pal_.len();
                    let (idx, _) = pal_.insert_full(format!("{}", len), color);
                    idx
                }
            }
        };

        self.current_palette_mut().idx = idx;
    }

    pub fn current_palette_mut(&mut self) -> &mut PaletteGroup {
        return self.palettes.get_index_mut(self.selected_palette_idx).unwrap().1;
    }

    pub fn current_palette(&self) -> &PaletteGroup {
        return &self.palettes.get_index(self.selected_palette_idx).unwrap().1;
    }

    pub fn current_color(&self) -> (String, Color) {
        let pal = self.current_palette();
        let idx = pal.idx;
        let pal_ = pal.colors.borrow();
        let ret = pal_.get_index(idx).unwrap();

        (ret.0.to_owned(), *ret.1)
    }
}

fn pico8() -> PaletteGroup {
    let mut colors = IndexMap::new();
    colors.insert("black".to_owned(), Color::rgba(0, 0, 0, 255));
    colors.insert("dark-blue".to_owned(), Color::rgba(29, 43, 83, 255));
    colors.insert("dark-purple".to_owned(), Color::rgba(126, 37, 83, 255));
    colors.insert("dark-green".to_owned(), Color::rgba(0, 135, 81, 255));
    colors.insert("brown".to_owned(), Color::rgba(171, 82, 54, 255));
    colors.insert("dark-gray".to_owned(), Color::rgba(95, 87, 79, 255));
    colors.insert("light-gray".to_owned(), Color::rgba(194, 195, 199, 255));
    colors.insert("white".to_owned(), Color::rgba(255, 241, 232, 255));
    colors.insert("red".to_owned(), Color::rgba(255, 0, 77, 255));
    colors.insert("orange".to_owned(), Color::rgba(255, 163, 0, 255));
    colors.insert("yellow".to_owned(), Color::rgba(255, 236, 39, 255));
    colors.insert("green".to_owned(), Color::rgba(0, 228, 54, 255));
    colors.insert("blue".to_owned(), Color::rgba(41, 173, 255, 255));
    colors.insert("indigo".to_owned(), Color::rgba(131, 118, 156, 255));
    colors.insert("pink".to_owned(), Color::rgba(255, 119, 168, 255));
    colors.insert("peach".to_owned(), Color::rgba(255, 204, 170, 255));
    PaletteGroup {
        colors: Rc::new(RefCell::new(colors)),
        idx: 0
    }
}

fn get_palette_hex(p: &path::PathBuf) -> io::Result<PaletteGroup> {
    let mut colors = IndexMap::new();
    let cols = fs::read_to_string(p)?;
    for col in cols.lines() {
        let color = Color::from_hex(&col[1..]).expect(&format!("Cannot decode hex in file {:?}", p));
        colors.insert(col.to_owned(), color);
    }
    Ok(PaletteGroup{
        colors: Rc::new(RefCell::new(colors)),
        idx: 0,
    })
}

fn get_palette_png(p: &path::PathBuf) -> io::Result<PaletteGroup> {
    let mut colors = IndexMap::new();
    let img = image::open(p).unwrap();
    for pix in img.pixels() {
        let color = pix.2;
        let my_color = Color::rgba(color[0], color[1], color[2], 255);
        colors.insert(format!("color##{},{}", pix.0, pix.1), my_color);
    }
    Ok(PaletteGroup{
        colors: Rc::new(RefCell::new(colors)),
        idx: 0,
    })
}
