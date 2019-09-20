use crate::prelude::*;
use crate::render::imgui::ImguiRenderer;
use xprite::rendering::image_renderer::ImageRenderer;
use std::collections::BTreeMap;

pub mod brush_state;
pub mod filepopup_state;
pub mod preview_window;

pub struct State {
    pub xprs: Vec<Xprite>,
    pub xpr_idx: usize,

    pub file_popup: filepopup_state::FilePopupState,
    pub inputs: InputState,
    pub hotkeys: HotkeyController,
    pub preview_window_state: preview_window::PreviewWindowState,
    pub exporter: xprite::core::exporter::Exporter,

    pub show_exporter: bool,
    pub show_console: bool,
    pub show_brush: bool,
    pub show_symmetry: bool,
    pub script_fname: Option<String>,

    pub cols_per_row: i32,

    pub rename_layer: Option<(usize, usize)>,
    pub rename_group: Option<usize>,
    pub brush: brush_state::BrushState,

    /// rendered texture
    pub texture: Option<usize>,

    pub icons: BTreeMap<&'static str, usize>,
    icons_initialized: bool,

}

impl Default for State {
    fn default() -> Self {
        Self {
            file_popup: Default::default(),
            xprs: vec![Default::default()],
            xpr_idx: 0,
            hotkeys: HotkeyController::new(),
            inputs: InputState::default(),
            preview_window_state: Default::default(),
            exporter: Default::default(),

            brush: Default::default(),
            show_exporter: false,
            show_console: false,
            show_brush: false,
            show_symmetry: false,
            script_fname: None,
            texture: None,
            cols_per_row: 8,
            rename_layer: None,
            rename_group: None,

            icons: BTreeMap::new(),
            icons_initialized: false,
        }
    }
}

impl State {
    pub fn xpr(&self) -> &Xprite {
        &self.xprs[self.xpr_idx]
    }

    pub fn xpr_mut(&mut self) -> &mut Xprite {
        &mut self.xprs[self.xpr_idx]
    }

    pub fn close_xpr(&mut self, idx: usize) {
        self.xprs.remove(idx);
    }

    pub fn new(xpr: Xprite) -> State {
        State {
            xprs: vec![xpr],
            ..Default::default()
        }
    }

    pub fn add_icon(rdr: &mut ImguiRenderer, src: &[u8]) -> usize {
        let img = image::load_from_memory(src).unwrap();
        let texture_id = rdr.add_img(img, image::ColorType::RGBA(0));
        texture_id
    }

    pub fn load_icons(&mut self, rdr: &mut ImguiRenderer) {
        if self.icons_initialized {
            return;
        }
        self.icons.insert("color_picker", Self::add_icon(rdr, include_bytes!("../../assets/colorpicker.png")));
        self.icons.insert("button_up", Self::add_icon(rdr, include_bytes!("../../assets/up.png")));
        self.icons.insert("button_hold", Self::add_icon(rdr, include_bytes!("../../assets/hold.png")));
        self.icons.insert("button_down", Self::add_icon(rdr, include_bytes!("../../assets/down.png")));
        self.icons.insert(ToolType::Pencil.as_str(), Self::add_icon(rdr, include_bytes!("../../assets/pencil.png")));
        self.icons.insert(ToolType::Line.as_str(), Self::add_icon(rdr, include_bytes!("../../assets/line.png")));
        self.icons.insert(ToolType::PaintBucket.as_str(), Self::add_icon(rdr, include_bytes!("../../assets/paint.png")));
        self.icons.insert(ToolType::Vector.as_str(), Self::add_icon(rdr, include_bytes!("../../assets/vector.png")));
        self.icons.insert(ToolType::Eraser.as_str(), Self::add_icon(rdr, include_bytes!("../../assets/eraser.png")));
        self.icons.insert(ToolType::Rect.as_str(), Self::add_icon(rdr, include_bytes!("../../assets/rect.png")));
        self.icons.insert(ToolType::Texture.as_str(), Self::add_icon(rdr, include_bytes!("../../assets/texture.png")));
        self.icons.insert(ToolType::Ellipse.as_str(), Self::add_icon(rdr, include_bytes!("../../assets/ellipse.png")));
        self.icons.insert(ToolType::Marquee.as_str(), Self::add_icon(rdr, include_bytes!("../../assets/marquee.png")));
        self.icons.insert(ToolType::AutoShade.as_str(), Self::add_icon(rdr, include_bytes!("../../assets/autoshade.png")));
        self.icons_initialized = true;
    }

    /// checks if texture needs to be updated.
    /// redraw texture
    pub fn redraw_pixels(&mut self, rdr: &mut ImguiRenderer) -> Result<(), String> {
        if self.xpr().redraw() || self.texture.is_none() {
            self.update_preview(rdr);
            unsafe {
                self.xpr_mut().override_redraw(false);
            }
        }
        Ok(())
    }

    fn update_preview(&mut self, rdr: &mut ImguiRenderer) -> Option<()> {
        let mut img_rdr = ImageRenderer::new(self.xpr().canvas.bg, self.xpr().canvas.art_w, self.xpr().canvas.art_h);
        img_rdr.fill_canvas();
        self.xpr().preview(&mut img_rdr).unwrap();
        img_rdr.render(Some(self.xpr()))?;
        let img = img_rdr.as_img();
        if let Some(id) = self.texture {
            rdr.replace_img(img.to_owned(), image::RGBA(0), id);
        } else {
            self.texture = Some(rdr.add_img(img.to_owned(), image::RGBA(0)));
        }
        Some(())
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
        self.show_exporter = !self.show_exporter;
    }

    pub fn toggle_symmetry(&mut self) {
        self.show_symmetry = !self.show_symmetry;
    }

    pub fn push_xpr(&mut self, xpr: Xprite) {
        self.xprs.push(xpr);
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
                self.file_popup.show_file_popup = true;
                self.file_popup.show_file_is_save = false;
            }
            Save => {
                self.file_popup.show_file_popup = true;
                self.file_popup.show_file_is_save = true;
            }
            NewXpr => {
                // TODO: some sort of prompt here
                debug!("New xpr");
                let xpr = Xprite::new("New Sprite".to_owned(), 100., 100.);
                self.push_xpr(xpr);
            }
            CloseXpr(idx) => {
                // TODO: some sort of prompt here
                self.close_xpr(idx)
            }
            RunScript => {
                unimplemented!();
            }
            SetPaletteIndex(i) => {
                self.xpr_mut().palette.current_palette_mut().idx = i;
            }
            ToggleSymmetryPanel => self.toggle_symmetry(),
            ToggleExporterPanel => self.toggle_exporter(),
            Unmapped => (),
        }
        Ok(())
    }

    pub fn export(&self) {
        self.exporter.run_export(&self.xpr());
    }

    pub fn set_brush_for_tool(&mut self, brush: BrushType, tool_type: ToolType) {
        if self.brush.sz[0] < 1 {
            self.brush.sz[0] = 1;
        }
        macro_rules! tool {
            ($tool: expr) => {
                match brush {
                    BrushType::Circle(_) | BrushType::Square(_) => {
                        let sz = self.brush.sz[0];
                        $tool.set(self.xpr_mut(), "brush", &format!("{}{}", brush.as_str(), sz)).unwrap();
                    }
                    BrushType::Line(_, _) => {
                        let sz0 = self.brush.sz[0];
                        let sz1 = self.brush.sz[1];
                        $tool.set(self.xpr_mut(), "brush", &format!("{}{},{}", brush.as_str(), sz0, sz1)).unwrap();
                    }
                };
            };
        }

        match tool_type {
            ToolType::Pencil => tool!(self.xpr_mut().toolbox.pencil.clone().borrow_mut()),
            ToolType::Vector => tool!(self.xpr_mut().toolbox.vector.clone().borrow_mut()),
            ToolType::Eraser => tool!(self.xpr_mut().toolbox.eraser.clone().borrow_mut()),
            _ => return,
        }
    }
}
