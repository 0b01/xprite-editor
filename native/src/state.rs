use xprite::prelude::*;
use crate::prelude::*;
use crate::render::cairo::CairoRenderer;

pub struct State {
    pub xpr: Xprite,
    pub show_settings: bool,
    pub show_console: bool,
    pub hotkeys: HotkeyController,
    pub inputs: InputState,
    pub cairo: CairoRenderer,
    pub script_fname: Option<String>
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

    pub fn save(&mut self) {
        self.xpr.export(&self.cairo);
        self.cairo.render();
        if let Some(im) = self.cairo.img() {
            let img_path = "1.png";
            info!("writing file to {}", img_path);
            let mut f = ::std::fs::File::create(img_path).unwrap();
            im.save(&mut f, image::ImageFormat::PNG).unwrap()
        };
        self.cairo.reset();
    }

}