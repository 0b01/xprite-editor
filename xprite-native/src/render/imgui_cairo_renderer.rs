use imgui::*;
use xprite::rendering::{Renderer, MouseCursorType};
use cairo::{ImageSurface, Context, Format};

pub struct ImguiCairoRenderer<'ui> {
    pub surface: ImageSurface,
    pub cr: Option<Context>,
    pub ui: &'ui Ui<'ui>,
    pub gl_ctx: &'ui Facade,
    pub textures: &'ui mut Textures<Texture2d>,

}

impl<'ui> Renderer for ImguiCairoRenderer<'ui> {

    fn width(&self) -> u32 {
        self.ui.get_window_size().0 as u32
    }

    fn height(&self) -> u32 {
        self.ui.get_window_size().1 as u32
    }

    fn circ(&self, p0:[f32;2], r:f32, color:[f32;4], filled: bool) {
        // let draw_list = self.ui.get_window_draw_list();
        // draw_list
        //     .add_circle(p0, r, color)
        //     .filled(filled)
        //     .build();
    }

    fn bezier(&self, p0:[f32;2], cp1:[f32;2], cp2: [f32;2], p1:[f32;2], color:[f32;4], thickness: f32) {
        // let draw_list = self.ui.get_window_draw_list();
        // draw_list
        //     .add_bezier_curve(p0, cp1, cp2, p1, color)
        //     // .filled(filled)
        //     .thickness(thickness)
        //     .build();
    }


    fn rect(&self, p0:[f32;2], p1:[f32;2], color:[f32;4], filled: bool) {
        // let draw_list = self.ui.get_window_draw_list();
        // draw_list
        //     .add_rect(p0, p1, color)
        //     .filled(filled)
        //     .build();
    }

    fn line(&self, p0:[f32;2], p1:[f32;2], color:[f32;4]) {
        // let draw_list = self.ui.get_window_draw_list();
        // draw_list
        //     .add_line(p0, p1, color)
        //     .build();
    }

    fn set_mouse_cursor(&self, cursor_type: MouseCursorType) {
        // let c = match cursor_type {
        //     MouseCursorType::Move => ImGuiMouseCursor::Move,
        // };
        // self.ui.imgui().set_mouse_cursor(c);
    }

    fn render(&mut self) {
        self.cr = None;
        let data = self.surface.get_data().expect("Cannot get data");
        let image = RawImage2d {
            data: Cow::Borrowed(&*data),
            width: 100,
            height: 100,
            format: ClientFormat::U8U8U8U8,
        };
        let gl_texture = Texture2d::new(self.gl_ctx, image).unwrap();
        let texture_id = self.textures.insert(gl_texture);
        println!("fuck");
        drop(data);

        self.ui.window(im_str!("Hello textures"))
            .size((400.0, 600.0), ImGuiCond::FirstUseEver)
            .build(|| {
                self.ui.text(im_str!("Hello textures!"));
                self.ui.image(texture_id, [100.,100.]).build();
            });


    }

}

use glium::{
    backend::Facade,
    texture::{ClientFormat, RawImage2d},
    Texture2d,
};

use std::borrow::Cow;

impl<'ui> ImguiCairoRenderer<'ui> {
    pub fn new<F>(ui: &'ui Ui, gl_ctx: &'ui F, textures: &'ui mut Textures<Texture2d>) -> Self
    where
        F: Facade
    {
        let mut surface = ImageSurface::create(Format::ARgb32, 100, 100).expect("Cannot create surface.");
        let cr = Context::new(&mut surface);
        cr.set_source_rgb(1.0, 1.0, 1.0);
        cr.paint();


        let cr = Some(cr);


        Self {
            surface,
            ui,
            cr,
            gl_ctx,
            textures,
        }
    }
}
