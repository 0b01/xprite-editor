use crate::prelude::*;
use imgui::*;
use xprite::rendering::{Renderer, MouseCursorType};
use cairo::{ImageSurface, Context, Format};
use std::f64;
use glium::{
    backend::Facade,
    texture::{ClientFormat, RawImage2d},
    Texture2d,
};
use std::borrow::Cow;

#[allow(unused)]
pub struct ImguiCairoRenderer<'ui> {
    w: i32,
    h: i32,
    pub surface: ImageSurface,
    pub cr: Option<Context>,
    pub ui: &'ui Ui<'ui>,
    pub gl_ctx: &'ui Facade,
    pub textures: &'ui mut Textures<Texture2d>,

}

#[allow(unused)]
impl<'ui> Renderer for ImguiCairoRenderer<'ui> {

    fn width(&self) -> u32 {
        self.ui.get_window_size().0 as u32
    }

    fn height(&self) -> u32 {
        self.ui.get_window_size().1 as u32
    }

    fn circ(&mut self, p0:[f32;2], r:f32, color:[f32;4], filled: bool) {
        // let draw_list = self.ui.get_window_draw_list();
        // draw_list
        //     .add_circle(p0, r, color)
        //     .filled(filled)
        //     .build();
    }

    fn bezier(&mut self, p0:[f32;2], cp1:[f32;2], cp2: [f32;2], p1:[f32;2], color:[f32;4], thickness: f32) {
        // let draw_list = self.ui.get_window_draw_list();
        // draw_list
        //     .add_bezier_curve(p0, cp1, cp2, p1, color)
        //     // .filled(filled)
        //     .thickness(thickness)
        //     .build();
    }


    fn rect(&mut self, p0:[f32;2], p1:[f32;2], color:[f32;4], filled: bool) {

        self.cr.as_ref().unwrap().set_source_rgba(
            f64::from(color[0]),
            f64::from(color[1]),
            f64::from(color[2]),
            f64::from(color[3])
        );
        self.cr.as_ref().unwrap().rectangle(
            f64::from(p0[0]),
            f64::from(p0[1]),
            f64::from(p1[0] - p0[0]),
            f64::from(p1[1] - p0[1])
        );
        self.cr.as_ref().unwrap().fill();


        // let draw_list = self.ui.get_window_draw_list();
        // draw_list
        //     .add_rect(p0, p1, color)
        //     .filled(filled)
        //     .build();
    }

    fn line(&mut self, p0:[f32;2], p1:[f32;2], color:[f32;4]) {
        // let draw_list = self.ui.get_window_draw_list();
        // draw_list
        //     .add_line(p0, p1, color)
        //     .build();
    }

    fn set_mouse_cursor(&mut self, cursor_type: MouseCursorType) {
        // let c = match cursor_type {
        //     MouseCursorType::Hand => ImGuiMouseCursor::Hand,
        // };
        // self.ui.imgui().set_mouse_cursor(c);
    }

    fn render(&mut self) {
        let w = self.width() ;
        let h = self.height();
        if self.w != w as i32|| self.h != h as i32 { return }

        self.cr = None;
        let data = self.surface.get_data().expect("Cannot get data");
        let image = RawImage2d {
            data: Cow::Borrowed(&*data),
            width: w,
            height: h,
            format: ClientFormat::U8U8U8U8,
        };
        let gl_texture = Texture2d::new(self.gl_ctx, image).unwrap();
        let texture_id = self.textures.insert(gl_texture);
        // println!("cairo rerender");
        drop(data);

        self.ui.image(texture_id, [w as f32, h as f32]).build();
    }

}

#[allow(unused)]
impl<'ui> ImguiCairoRenderer<'ui> {
    pub fn new<F>(ui: &'ui Ui, gl_ctx: &'ui F, textures: &'ui mut Textures<Texture2d>,
        state: &State,
    ) -> Self
    where
        F: Facade
    {
        let w = state.xpr.canvas.canvas_w as i32;
        let h = state.xpr.canvas.canvas_h as i32;

        let mut surface = ImageSurface::create(Format::ARgb32, w, h).expect("Cannot create surface.");
        let cr = Context::new(&surface);
        // cr.set_source_rgb(1.0, 1.0, 1.0);
        // cr.paint();

        let cr = Some(cr);


        Self {
            w, h,
            surface,
            ui,
            cr,
            gl_ctx,
            textures,
        }
    }
}
