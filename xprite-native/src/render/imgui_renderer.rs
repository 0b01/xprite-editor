use imgui::*;

use xprite::rendering::Renderer;

pub struct ImguiRenderer<'ui> {
    pub ui: Option<&'ui Ui<'ui>>,
}

impl<'ui> Renderer for ImguiRenderer<'ui> {
    fn width(&self) -> u32 {
        if self.ui.is_none() { return 0; }
        self.ui.unwrap().get_window_size().0 as u32
    }
    fn height(&self) -> u32 {
        if self.ui.is_none() { return 0; }
        self.ui.unwrap().get_window_size().1 as u32
    }
    fn set_fill_style_color(&self, _color: &str) {
        if self.ui.is_none() { return; }
        ()
    }
    fn fill_rect(&self, a: f64, b: f64, c: f64,d: f64) {
        if self.ui.is_none() { return; }
        let draw_list = self.ui.unwrap().get_window_draw_list();
        draw_list.add_rect([a as f32,b as f32], [c as f32,d as f32], [0.,0.,0.,0.]).build();
    }
}

impl<'ui> ImguiRenderer<'ui> {
    pub fn new() -> Self {
        Self {
            ui: None
        }
    }
    pub fn init(&mut self, ui: &'ui Ui) {
        self.ui = Some(ui);
    }
}
