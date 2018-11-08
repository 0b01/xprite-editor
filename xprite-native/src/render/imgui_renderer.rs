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
    fn set_fill_style_color(&self, _color: &str) {
        ()
    }
    fn fill_rect(&self, a: f32, b: f32, c: f32, d: f32) {
        let draw_list = self.ui.get_window_draw_list();
        draw_list
            .add_rect(
                (a, b),
                (c, d),
                [0.,0.,200.,255.])
            .filled(true)
            .build();
        println!("just drew rect {} {} {} {}", a, b, c, d);
    }
}

impl<'ui> ImguiRenderer<'ui> {
    pub fn new(ui: &'ui Ui) -> Self {
        Self { ui }
    }
}
