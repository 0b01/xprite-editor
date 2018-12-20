use xprite::prelude::*;
use crate::prelude::*;
use crate::render::cairo::CairoRenderer;
use xprite::bincode::{serialize, deserialize};
use std::io::{BufWriter, Write, BufReader, Read};
use std::fs::File;

pub struct State {
    pub xpr: Xprite,
    pub show_settings: bool,
    pub show_console: bool,
    pub hotkeys: HotkeyController,
    pub inputs: InputState,
    pub cairo: CairoRenderer,
    pub script_fname: Option<String>,
}

impl State {
    pub fn new(xpr: Xprite, cairo: CairoRenderer) -> State {
        State {
            xpr,
            show_settings: false,
            show_console: false,
            hotkeys: HotkeyController::new(),
            inputs: InputState::default(),
            cairo,
            script_fname: None,
        }
    }

    pub fn export_png(&mut self, img_path: &str) {
        self.xpr.export(&mut self.cairo).unwrap();
        self.cairo.render();
        if let Some(im) = self.cairo.img() {
            info!("writing file to {}", img_path);
            let mut f = File::create(img_path).unwrap();
            im.save(&mut f, image::ImageFormat::PNG).unwrap()
        };
        self.cairo.reset();
    }

    pub fn save(&mut self, file_path: &str) {
        let encoded: Vec<u8> = serialize(&self.xpr).unwrap();
        let f = File::create(file_path).unwrap();
        let mut wtr = BufWriter::new(f);
        wtr.write_all(&encoded).unwrap();
    }

    pub fn open(&mut self, png_path: &str) {
        // let mut f = File::open(png_path).unwrap();
        // let mut wtr = BufReader::new(f);
    }

    pub fn load(&mut self, file_path: &str) {
        let f = File::open(file_path).unwrap();
        let mut reader = BufReader::new(f);

        let mut encoded = Vec::new();
        reader.read_to_end(&mut encoded).unwrap();

        let xpr: Xprite = deserialize(&encoded).unwrap();
        println!("xpr: {:#?}", xpr);
    }

}