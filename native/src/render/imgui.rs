use imgui::*;
use xprite::rendering::{Renderer, MouseCursorType};
use xprite::image::GenericImage;
use glium::{ backend::Facade, Texture2d, texture::{RawImage2d, ClientFormat} };
use std::borrow::Cow;

pub struct ImguiRenderer<'ui> {
    pub ui: &'ui Ui<'ui>,
    pub gl_ctx: &'ui Facade,
    pub textures: &'ui mut Textures<Texture2d>,
}

impl<'ui> Renderer for ImguiRenderer<'ui> {

    fn width(&self) -> u32 {
        self.ui.get_window_size().0 as u32
    }

    fn height(&self) -> u32 {
        self.ui.get_window_size().1 as u32
    }

    fn circ(&mut self, p0:[f32;2], r:f32, color:[f32;4], filled: bool) {
        let draw_list = self.ui.get_window_draw_list();
        draw_list
            .add_circle(p0, r, color)
            .filled(filled)
            .build();
    }

    fn bezier(&mut self, p0:[f32;2], cp1:[f32;2], cp2: [f32;2], p1:[f32;2], color:[f32;4], thickness: f32) {
        let draw_list = self.ui.get_window_draw_list();
        draw_list
            .add_bezier_curve(p0, cp1, cp2, p1, color)
            // .filled(filled)
            .thickness(thickness)
            .build();
    }


    fn rect(&mut self, p0:[f32;2], p1:[f32;2], color:[f32;4], filled: bool) {
        let draw_list = self.ui.get_window_draw_list();
        draw_list
            .add_rect(p0, p1, color)
            .filled(filled)
            .build();
    }

    fn line(&mut self, p0:[f32;2], p1:[f32;2], color:[f32;4]) {
        let draw_list = self.ui.get_window_draw_list();
        draw_list
            .add_line(p0, p1, color)
            .build();
    }

    fn set_mouse_cursor(&mut self, cursor_type: MouseCursorType) {
        let c = match cursor_type {
            MouseCursorType::Move => ImGuiMouseCursor::Move,
        };
        self.ui.imgui().set_mouse_cursor(c);
    }

    fn add_img(&mut self, img: xprite::image::DynamicImage) -> usize {
        let (width, height) = img.dimensions();
        let img = RawImage2d {
            data: Cow::Owned(img.raw_pixels()),
            width,
            height,
            format: ClientFormat::U8U8U8,
        };

        let gl_texture = Texture2d::new(self.gl_ctx, img).unwrap();
        self.textures.insert(gl_texture).id()
    }

}

impl<'ui> ImguiRenderer<'ui> {
    pub fn new(ui: &'ui Ui, gl_ctx: &'ui Facade, textures: &'ui mut Textures<Texture2d>) -> Self {
        Self { ui, gl_ctx, textures }
    }

}
