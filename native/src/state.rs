use crate::prelude::*;
use std::borrow::Cow;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use xprite::bincode::{deserialize, serialize};
use xprite::image::GenericImageView;
use xprite::rendering::image_renderer::ImageRenderer;
use crate::render::imgui::ImguiRenderer;

pub struct State<'a> {
    pub xpr: Xprite,
    pub show_settings: bool,
    pub show_console: bool,
    pub hotkeys: HotkeyController,
    pub inputs: InputState,
    pub script_fname: Option<String>,
    pub palette_color_name: Option<Cow<'a, str>>,
    pub palette_idx: i32,

    pub cols_per_row: i32,

    pub preview_texture: Option<usize>,
}

impl<'a> State<'a> {
    pub fn new(xpr: Xprite) -> State<'a> {
        State {
            xpr,
            show_settings: false,
            show_console: false,
            hotkeys: HotkeyController::new(),
            inputs: InputState::default(),
            script_fname: None,
            preview_texture: None,
            palette_color_name: None,
            palette_idx: 0,
            cols_per_row: 8,
        }
    }

    pub fn update_preview(&mut self, rdr: &mut ImguiRenderer) {
        let mut img_rdr = ImageRenderer::new(self.xpr.canvas.art_w, self.xpr.canvas.art_h);
        self.xpr.preview(&mut img_rdr).unwrap();
        img_rdr.render();
        let img = img_rdr.img();
        if let Some(id) = self.preview_texture {
            rdr.replace_img(img.to_owned(), image::RGBA(0), id);
        } else {
            self.preview_texture = Some(rdr.add_img(img.to_owned(), image::RGBA(0)));
        }
    }

    pub fn redraw_pixels(&mut self, rdr: &mut ImguiRenderer) -> Result<(), String> {
        dbg!(self.xpr.redraw);
        if self.xpr.redraw || self.preview_texture.is_none() {
            dbg!("redrawing");
            self.update_preview(rdr);
            self.xpr.redraw = false;
        }
        Ok(())
    }

    pub fn toggle_hotkeys(&mut self) {
        debug!("Toggle hotkeys");
        self.hotkeys.toggle();
    }

    pub fn save_png(&mut self, img_path: &str) {
        let mut rdr = ImageRenderer::new(self.xpr.canvas.art_w, self.xpr.canvas.art_h);
        self.xpr.export(&mut rdr).unwrap();
        rdr.render();
        let im = rdr.img();
        info!("writing file to {}", img_path);
        im.save(img_path).unwrap();
    }

    pub fn save_xpr(&mut self, file_path: &str) {
        info!("saving xpr file to {}", file_path);
        let encoded: Vec<u8> = serialize(&self.xpr).unwrap();
        let f = File::create(file_path).unwrap();
        let mut wtr = BufWriter::new(f);
        wtr.write_all(&encoded).unwrap();
    }

    pub fn load_png(&mut self, png_path: &str) {
        info!("loading png file {}", png_path);
        let img = xprite::image::open(png_path).unwrap();
        let (w, h) = img.dimensions();
        let mut xpr = Xprite::new(w as f32, h as f32);
        xpr.current_layer_mut().unwrap().content = img.into();
        self.xpr = xpr;
    }

    pub fn load_xpr(&mut self, file_path: &str) {
        info!("loading xpr file {}", file_path);
        let f = File::open(file_path).unwrap();
        let mut reader = BufReader::new(f);

        let mut encoded = Vec::new();
        reader.read_to_end(&mut encoded).unwrap();

        let xpr: Xprite = deserialize(&encoded).unwrap();
        self.xpr = xpr;
    }
}
