use crate::prelude::*;
use crate::render::imgui::ImguiRenderer;
use std::borrow::Cow;
use std::fs::File;
use xprite::image::GenericImageView;
use xprite::rendering::image_renderer::ImageRenderer;

use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
pub enum PreviewWindowMode {
    Fill,
    OneX,
    TwoX,
}

impl PreviewWindowMode {
    pub fn as_str(&self) -> &str {
        match self {
            PreviewWindowMode::Fill => "Fill",
            PreviewWindowMode::OneX => "1x",
            PreviewWindowMode::TwoX => "2x",
        }
    }

    pub const VARIANTS: [PreviewWindowMode; 3] = [
        PreviewWindowMode::Fill,
        PreviewWindowMode::OneX,
        PreviewWindowMode::TwoX,
    ];
}

impl FromStr for PreviewWindowMode {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, ()> {
        match string {
            "Fill" => Ok(PreviewWindowMode::Fill),
            "1x" => Ok(PreviewWindowMode::OneX),
            "2x" => Ok(PreviewWindowMode::TwoX),
            _ => panic!("impossible"),
        }
    }
}


pub struct PreviewWindowState {
    pub mode: PreviewWindowMode,
}

impl Default for PreviewWindowState {
    fn default() -> Self {
        Self {
            mode: PreviewWindowMode::Fill,
        }
    }
}


pub struct BrushState {
    pub sz: [i32; 2],
}

impl Default for BrushState {
    fn default() -> Self {
        Self {
            sz: [1, 0],
        }
    }
}

pub struct FilePopupState {
    pub show_file_popup: bool,
    pub open_file_name: ImString,
    pub show_file_is_save: bool,
}

impl Default for FilePopupState {
    fn default() -> Self {
        Self {
            show_file_popup: false,
            show_file_is_save: true,
            open_file_name: ImString::new("./sample_files/1.ase"),
        }
    }
}

pub struct PaletteWindowState<'a> {
    pub palette_color_name: Option<Cow<'a, str>>,
    pub palette_idx: i32,
}

impl<'a> Default for PaletteWindowState<'a> {
    fn default() -> Self {
        Self {
            palette_color_name: None,
            palette_idx: 0,
        }
    }
}

pub struct State<'a> {
    pub xpr: Xprite,
    pub file_popup: FilePopupState,
    pub inputs: InputState,
    pub hotkeys: HotkeyController,
    pub palette_window: PaletteWindowState<'a>,
    pub preview_window_state: PreviewWindowState,

    pub show_console: bool,
    pub script_fname: Option<String>,

    pub cols_per_row: i32,

    pub rename_layer: Option<(usize, usize)>,
    pub rename_group: Option<usize>,
    pub brush: BrushState,

    pub texture: Option<usize>,
}

impl<'a> Default for State<'a> {
    fn default() -> Self {
        Self {
            file_popup: Default::default(),
            xpr: Default::default(),
            palette_window: Default::default(),
            hotkeys: HotkeyController::new(),
            inputs: InputState::default(),
            preview_window_state: Default::default(),

            brush: Default::default(),
            show_console: false,
            script_fname: None,
            texture: None,
            cols_per_row: 8,
            rename_layer: None,
            rename_group: None,
        }
    }
}

impl<'a> State<'a> {
    pub fn new(xpr: Xprite) -> State<'a> {
        State {
            xpr,
            ..Default::default()
        }
    }

    pub fn update_preview(&mut self, rdr: &mut ImguiRenderer) {
        let mut img_rdr = ImageRenderer::new(self.xpr.canvas.art_w, self.xpr.canvas.art_h);
        img_rdr.fill_canvas();
        self.xpr.preview(&mut img_rdr).unwrap();
        img_rdr.render();
        let img = img_rdr.as_img();
        if let Some(id) = self.texture {
            rdr.replace_img(img.to_owned(), image::RGBA(0), id);
        } else {
            self.texture = Some(rdr.add_img(img.to_owned(), image::RGBA(0)));
        }
    }

    /// checks if texture needs to be updated.
    /// redraw texture
    pub fn redraw_pixels(&mut self, rdr: &mut ImguiRenderer) -> Result<(), String> {
        if self.xpr.redraw || self.texture.is_none() {
            self.update_preview(rdr);
            self.xpr.redraw = false;
        }
        Ok(())
    }

    pub fn toggle_hotkeys(&mut self) {
        debug!("Toggle hotkeys");
        self.hotkeys.toggle();
    }

    pub fn save_img(&mut self, img_path: &str) {
        let mut rdr = ImageRenderer::new(self.xpr.canvas.art_w, self.xpr.canvas.art_h);
        self.xpr.export(&mut rdr).unwrap();
        rdr.render();
        let im = rdr.as_img();
        info!("writing file to {}", img_path);
        im.save(img_path).unwrap();
    }

    pub fn load_img(&mut self, png_path: &str) {
        info!("loading png file {}", png_path);
        let img = xprite::image::open(png_path).unwrap();
        let (w, h) = img.dimensions();
        let mut xpr = Xprite::new(w as f64, h as f64);
        xpr.current_layer_mut().unwrap().content = img.into();
        self.xpr = xpr; // TODO: create a new tab for file
    }

    pub fn save_ase(&mut self, file_path: &str) {
        info!("saving ase file to {}", file_path);
        let mut f = File::create(file_path).unwrap();
        let aseprite = self.xpr.as_ase();
        aseprite.write(&mut f).unwrap();
    }

    pub fn load_ase(&mut self, file_path: &str) {
        info!("loading ase file {}", file_path);
        let mut f = File::open(file_path).unwrap();
        let ase = xprite::ase::Aseprite::from_read(&mut f).unwrap();
        self.xpr = Xprite::from_ase(&ase);
    }

    pub fn execute(&mut self, bind: Bind) -> Result<(), String> {
        use self::Bind::*;
        match bind {
            Redo => self.xpr.redo(),
            Undo => self.xpr.undo(),
            PushTool(tool) => self.xpr.change_tool(tool)?,
            PopTool => self.xpr.toolbox.pop_tool(),
            ToggleConsole => {
                self.show_console = !self.show_console;
            }
            Load => {
                self.toggle_hotkeys();
                self.file_popup.show_file_popup = true;
                self.file_popup.show_file_is_save = false;
            }
            Save => {
                self.toggle_hotkeys();
                self.file_popup.show_file_popup = true;
                self.file_popup.show_file_is_save = true;
            }

            RunScript => {
                unimplemented!();
            }
            Unmapped => (),
        }
        Ok(())
    }
}
