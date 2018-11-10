use imgui::*;

use xprite::rendering::Renderer;

pub struct ImguiRenderer<'ui> {
    pub ui: &'ui Ui<'ui>,
}

impl<'ui> Renderer for ImguiRenderer<'ui> {

    fn width(&self) -> u32 {
        self.ui.get_window_size().0 as u32
    }

    fn height(&self) -> u32 {
        self.ui.get_window_size().1 as u32
    }

    fn rect(&self, p0:[f32;2], p1:[f32;2], color:[f32;4], filled: bool) {
        let draw_list = self.ui.get_window_draw_list();
        draw_list
            .add_rect(p0, p1, color)
            .filled(filled)
            .build();
    }

    fn line(&self, p0:[f32;2], p1:[f32;2], color:[f32;4]) {
        let draw_list = self.ui.get_window_draw_list();
        draw_list
            .add_line(p0, p1, color)
            .build();
    }

}

impl<'ui> ImguiRenderer<'ui> {
    pub fn new(ui: &'ui Ui) -> Self {
        Self { ui }
    }
}
