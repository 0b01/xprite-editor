use imgui::*;

use xprite::rendering::{Renderer, MouseCursorType};

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

    fn circ(&self, p0:[f32;2], r:f32, color:[f32;4], filled: bool) {
        let draw_list = self.ui.get_window_draw_list();
        draw_list
            .add_circle(p0, r, color)
            .filled(filled)
            .build();
    }

    fn bezier(&self, p0:[f32;2], cp1:[f32;2], cp2: [f32;2], p1:[f32;2], color:[f32;4]) {
        let draw_list = self.ui.get_window_draw_list();
        draw_list
            .add_bezier_curve(p0, cp1, cp2, p1, color)
            // .filled(filled)
            .thickness(1.)
            .build();
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

    fn set_mouse_cursor(&self, cursor_type: MouseCursorType) {
        let c = match cursor_type {
            MouseCursorType::Move => ImGuiMouseCursor::Move,
        };
        self.ui.imgui().set_mouse_cursor(c);
    }

}

impl<'ui> ImguiRenderer<'ui> {
    pub fn new(ui: &'ui Ui) -> Self {
        Self { ui }
    }
}
