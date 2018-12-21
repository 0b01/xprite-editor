use crate::prelude::*;
use xprite::rendering::{Renderer, MouseCursorType};
use cairo::{ImageSurface, Context, Format};
use xprite::image::{ImageBuffer, DynamicImage, Rgba};
use std::mem;

#[allow(unused)]
pub struct CairoRenderer {
    w: u32,
    h: u32,
    pub surface: ImageSurface,
    pub cr: Option<Context>,
    image: Option<image::DynamicImage>,
}

#[allow(unused)]
impl Renderer for CairoRenderer {

    fn width(&self) -> u32 {
        self.w
    }

    fn height(&self) -> u32 {
        self.h
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
        //     MouseCursorType::Move => ImGuiMouseCursor::Move,
        // };
        // self.ui.imgui().set_mouse_cursor(c);
    }

    fn render(&mut self) {
        let w = self.width() ;
        let h = self.height();
        if self.w != w || self.h != h { return }

        // drop cairo context which contains a reference to surface
        self.cr = None;
        let data = self.surface.get_data().expect("Cannot get data"); // ARGB
        let im = DynamicImage::ImageRgba8({
            let mut vec32: Vec<_> = unsafe { mem::transmute::<&[u8], &[u32]>(&*data) }
                .iter().map(|&i|argb2rgba(i)).collect();
            let vec8 = unsafe {
                let ratio = mem::size_of::<u32>() / mem::size_of::<u8>();
                let length = vec32.len() * ratio;
                let capacity = vec32.capacity() * ratio;
                let ptr = vec32.as_mut_ptr() as *mut u8;
                mem::forget(vec32);
                Vec::from_raw_parts(ptr, length, capacity)
            };

            ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(self.w, self.h, vec8).unwrap()
        });

        self.image = Some(im);
    }
}

#[inline(always)]
fn argb2rgba(i: u32) -> u32 {
     (i & 0xFF00_0000)        |
    ((i & 0x00FF_0000) >> 16) |
     (i & 0x0000_FF00)        |
    ((i & 0x0000_00FF) << 16)
}

#[allow(unused)]
impl CairoRenderer {
    pub fn new(art_w: f32, art_h: f32) -> Self {
        let w = art_w as u32;
        let h = art_h as u32;

        let surface = ImageSurface::create(Format::ARgb32, w as i32, h as i32).expect("Cannot create surface.");
        let cr = Context::new(&surface);
        // cr.set_source_rgb(1.0, 0.0, 1.0);
        // cr.paint();

        let cr = Some(cr);

        Self {
            w,
            h,
            surface,
            cr,
            image: None,
        }
    }

    pub fn reset(&mut self) {
        let cr = Context::new(&self.surface);
        self.cr = Some(cr);
    }

    pub fn img(&self) -> Option<&DynamicImage> {
        self.image.as_ref()
    }
}
