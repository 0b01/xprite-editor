use imgui::*;
use xprite::rendering::{Renderer, MouseCursorType};
use xprite::image::GenericImage;
use glium::{ backend::Facade, Texture2d, texture::{RawImage2d, ClientFormat} };
use std::borrow::Cow;
use xprite::indexmap::IndexMap;
use std::f32;

type DrawListPoint1 = [u32;2];
type DrawListPoint2 = [u32;2];
type DrawListColor = [f32;4];
type DrawListFill = bool;

pub struct ImguiRenderer<'ui> {
    pub ui: &'ui Ui<'ui>,
    pub gl_ctx: &'ui Facade,
    pub textures: &'ui mut Textures<Texture2d>,
    draw_list: IndexMap<(DrawListPoint1, DrawListPoint2), (DrawListColor, DrawListFill)>,
}

impl<'ui> Renderer for ImguiRenderer<'ui> {

    fn width(&self) -> f32 {
        self.ui.get_window_size().0
    }

    fn height(&self) -> f32 {
        self.ui.get_window_size().1
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
        self.draw_list.insert(
            (canonicalize(p0), canonicalize(p1)),
            (color, filled)
        );
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

    fn render(&mut self) {
        let imgui_draw_list = self.ui.get_window_draw_list();
        for ((p0,p1),(color,filled)) in self.draw_list.clone().into_iter() {
            imgui_draw_list
                .add_rect(
                    uncanonicalize(p0),
                    uncanonicalize(p1),
                    color
                )
                .filled(filled)
                .build();
        }
    }

    fn reset(&mut self) {
        self.draw_list.clear();
    }
}

impl<'ui> ImguiRenderer<'ui> {
    pub fn new(ui: &'ui Ui, gl_ctx: &'ui Facade, textures: &'ui mut Textures<Texture2d>) -> Self {
        let draw_list = IndexMap::new();
        Self {
            ui,
            gl_ctx,
            textures,
            draw_list,
        }
    }

}

#[inline(always)]
fn canonicalize(p: [f32;2]) -> [u32;2] {
    unsafe {
        [
            ::std::mem::transmute::<f32, u32>(p[0]),
            ::std::mem::transmute::<f32, u32>(p[1]),
        ]
    }
}

#[inline(always)]
fn uncanonicalize(p: [u32;2]) -> [f32;2] {
    [
        f32::from_bits(p[0]),
        f32::from_bits(p[1]),
    ]
}