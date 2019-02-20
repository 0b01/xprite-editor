use crate::prelude::*;
use crate::render::imgui::ImguiRenderer;
use xprite::rendering::image_renderer::ImageRenderer;

const COLOR_PICKER: &'static [u8; 13807] = include_bytes!("../../colorpicker.png");

pub mod preview_window;
pub mod palette_window;
pub mod brush_state;
pub mod exporter_state;
pub mod filepopup_state;

pub struct State<'a> {
    pub xprs: Vec<Xprite>,
    pub xpr_idx: usize,

    pub file_popup: filepopup_state::FilePopupState,
    pub inputs: InputState,
    pub hotkeys: HotkeyController,
    pub palette_window: palette_window::PaletteWindowState<'a>,
    pub preview_window_state: preview_window::PreviewWindowState,
    pub exporter_state: exporter_state::ExporterState,

    pub show_console: bool,
    pub show_brush: bool,
    pub show_symmetry: bool,
    pub script_fname: Option<String>,

    pub cols_per_row: i32,

    pub rename_layer: Option<(usize, usize)>,
    pub rename_group: Option<usize>,
    pub brush: brush_state::BrushState,

    pub texture: Option<usize>,
    pub color_picker_texture: Option<usize>,
}

impl<'a> Default for State<'a> {

    fn default() -> Self {
        Self {
            file_popup: Default::default(),
            xprs: vec![Default::default()],
            xpr_idx: 0,
            palette_window: Default::default(),
            hotkeys: HotkeyController::new(),
            inputs: InputState::default(),
            preview_window_state: Default::default(),
            exporter_state: Default::default(),

            brush: Default::default(),
            show_console: false,
            show_brush: false,
            show_symmetry: false,
            script_fname: None,
            texture: None,
            cols_per_row: 8,
            rename_layer: None,
            rename_group: None,
            color_picker_texture: None,
        }
    }
}

impl<'a> State<'a> {

    pub fn xpr(&self) -> &Xprite {
        &self.xprs[self.xpr_idx]
    }

    pub fn xpr_mut(&mut self) -> &mut Xprite {
        &mut self.xprs[self.xpr_idx]
    }

    pub fn close_xpr(&mut self, idx: usize) {
        self.xprs.remove(idx);
    }

    pub fn new(xpr: Xprite) -> State<'a> {
        State {
            xprs: vec![xpr],
            ..Default::default()
        }
    }

    pub fn load_icons(&mut self, rdr: &mut ImguiRenderer) {
        if self.color_picker_texture.is_some() { return; }
        let color_picker = COLOR_PICKER;
        let img = image::load_from_memory(color_picker).unwrap();
        let texture_id = rdr.add_img(img, image::ColorType::RGBA(0));
        self.color_picker_texture = Some(texture_id);
    }

    /// checks if texture needs to be updated.
    /// redraw texture
    pub fn redraw_pixels(
        &mut self,
        rdr: &mut ImguiRenderer,
    ) -> Result<(), String> {
        if self.xpr().redraw || self.texture.is_none() {
            self.update_preview(rdr);
            self.xpr_mut().redraw = false;
        }
        Ok(())
    }

    fn update_preview(&mut self, rdr: &mut ImguiRenderer) {
        let mut img_rdr =
            ImageRenderer::new(self.xpr().canvas.art_w, self.xpr().canvas.art_h);
        img_rdr.fill_canvas();
        self.xpr().preview(&mut img_rdr).unwrap();
        img_rdr.render();
        let img = img_rdr.as_img();
        if let Some(id) = self.texture {
            rdr.replace_img(img.to_owned(), image::RGBA(0), id);
        } else {
            self.texture = Some(rdr.add_img(img.to_owned(), image::RGBA(0)));
        }
    }

    pub fn toggle_hotkeys(&mut self) {
        debug!("Toggle hotkeys");
        self.hotkeys.toggle();
    }

    pub fn toggle_console(&mut self) {
        self.show_console = !self.show_console;
    }

    pub fn toggle_brush(&mut self) {
        self.show_brush = !self.show_brush;
    }

    pub fn toggle_exporter(&mut self) {
        self.exporter_state.show = !self.exporter_state.show;
    }

    pub fn toggle_symmetry(&mut self) {
        self.show_symmetry = !self.show_symmetry;
    }

    pub fn execute(&mut self, bind: Bind) -> Result<(), String> {
        use self::Bind::*;
        match bind {
            Redo => self.xpr_mut().redo(),
            Undo => self.xpr_mut().undo(),
            PushTool(tool) => self.xpr_mut().change_tool(tool)?,
            PopTool => self.xpr_mut().toolbox.pop_tool(),
            ToggleConsole => {
                self.toggle_console();
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

    pub fn export(&self) {
        self.exporter_state.run_export(&self.xpr());
    }
}
