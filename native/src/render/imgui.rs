use crate::image::{self, GenericImageView};
use glium::{
    backend::Facade,
    texture::{ClientFormat, RawImage2d},
    Texture2d,
};
use imgui::*;
use std::borrow::Cow;
use std::f32;
use xprite::rendering::{ MouseCursorType, Renderer };

pub struct ImguiRenderer<'ui> {
    pub ui: &'ui Ui<'ui>,
    pub gl_ctx: &'ui Facade,
    pub textures: &'ui mut Textures<Texture2d>,
    pub art_w: f32,
    pub art_h: f32,
}

impl<'ui> Renderer for ImguiRenderer<'ui> {
    fn width(&self) -> f32 {
        self.ui.get_window_size().0
    }

    fn height(&self) -> f32 {
        self.ui.get_window_size().1
    }

    fn circ(&mut self, p0: [f32; 2], r: f32, color: [f32; 4], filled: bool) {
        let draw_list = self.ui.get_window_draw_list();
        draw_list.add_circle(p0, r, color).filled(filled).build();
    }

    fn bezier(
        &mut self,
        p0: [f32; 2],
        cp1: [f32; 2],
        cp2: [f32; 2],
        p1: [f32; 2],
        color: [f32; 4],
        thickness: f32,
    ) {
        let draw_list = self.ui.get_window_draw_list();
        draw_list
            .add_bezier_curve(p0, cp1, cp2, p1, color)
            // .filled(filled)
            .thickness(thickness)
            .build();
    }

    fn rect(&mut self, p0: [f32; 2], p1: [f32; 2], color: [f32; 4], filled: bool) {
        // self.draw_list
        //     .insert((canonicalize(p0), canonicalize(p1)), (color, filled));

        let draw_list = self.ui.get_window_draw_list();
        draw_list
            .add_rect(p0, p1, color)
            .filled(filled)
            .build();
    }

    fn pixel(&mut self, _x: f32, _y: f32, _color: [f32; 4], _filled: bool) {
        ()
    }

    fn line(&mut self, p0: [f32; 2], p1: [f32; 2], color: [f32; 4]) {
        let draw_list = self.ui.get_window_draw_list();
        draw_list.add_line(p0, p1, color).build();
    }

    fn set_mouse_cursor(&mut self, cursor_type: MouseCursorType) {
        let c = match cursor_type {
            MouseCursorType::Hand => ImGuiMouseCursor::Hand,
        };
        self.ui.imgui().set_mouse_cursor(c);
    }

    fn add_img(&mut self, img: image::DynamicImage, format: image::ColorType) -> usize {
        let gl_texture = self.to_gl_texture(img, format);
        self.textures.insert(gl_texture).id()
    }

    fn render(&mut self) {
        ()
    }

    fn reset(&mut self) {
    }
}

impl<'ui> ImguiRenderer<'ui> {
    pub fn new(ui: &'ui Ui, gl_ctx: &'ui Facade, textures: &'ui mut Textures<Texture2d>, art_w: f32, art_h: f32) -> Self {
        Self {
            art_w,
            art_h,
            ui,
            gl_ctx,
            textures,
        }
    }

    pub fn replace_img(&mut self, img: image::DynamicImage, format: image::ColorType, texture_id: usize) {
        let gl_texture = self.to_gl_texture(img, format);
        self.textures.replace(ImTexture::from(texture_id), gl_texture);
    }

    fn to_gl_texture(&self, img: image::DynamicImage, format: image::ColorType,) -> Texture2d {
        let format = match format {
            image::ColorType::RGBA(_) => ClientFormat::U8U8U8U8,
            image::ColorType::RGB(_) => ClientFormat::U8U8U8,
            _ => unimplemented!("Color type"),
        };
        let (width, height) = img.dimensions();
        let img = RawImage2d {
            data: Cow::Owned(img.raw_pixels()),
            width,
            height,
            format,
        };
        Texture2d::new(self.gl_ctx, img).unwrap()
    }
}