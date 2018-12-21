use crate::prelude::*;
use crate::rendering::{Renderer, MouseCursorType};
use img::{DynamicImage, Rgba};
use img::GenericImage;

pub struct ImageRenderer {
    w: u32,
    h: u32,
    pub image: image::DynamicImage,
}

#[allow(unused)]
impl Renderer for ImageRenderer {

    fn width(&self) -> u32 { self.w }

    fn height(&self) -> u32 { self.h }

    fn circ(&mut self, p0:[f32;2], r:f32, color:[f32;4], filled: bool) { }

    fn bezier(&mut self, p0:[f32;2], cp1:[f32;2], cp2: [f32;2], p1:[f32;2], color:[f32;4], thickness: f32) { }

    fn rect(&mut self, p0:[f32;2], p1:[f32;2], color:[f32;4], filled: bool) {
        let color = {
            let c: Color = color.into();
            Rgba { data: [c.r, c.g, c.b, c.a] }
        };
        self.image.put_pixel(
            p0[0] as u32,
            p0[1] as u32,
            color
        );
    }

    fn line(&mut self, p0:[f32;2], p1:[f32;2], color:[f32;4]) { }

    fn set_mouse_cursor(&mut self, cursor_type: MouseCursorType) { }

    fn render(&mut self) { }
}

impl ImageRenderer {
    pub fn new(art_w: f32, art_h: f32) -> Self {
        let w = art_w as u32;
        let h = art_h as u32;
        let image = DynamicImage::new_rgba8(w, h);
        Self {
            w,
            h,
            image,
        }
    }

    pub fn img(&self) -> &DynamicImage {
        &self.image
    }
}

pub fn save_img(path: &str, im: &DynamicImage) {
    info!("writing file to {}", path);
    let mut f = ::std::fs::File::create(path).unwrap();
    im.save(&mut f, image::ImageFormat::PNG).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_img_render() {
        use super::*;
        let mut rdr = ImageRenderer::new(10., 10.);
        rdr.rect([0.,0.,], [0.,0.,], [1.,0.,0.,1.], true);
        let path = "test.png";
        save_img(path, rdr.img());
        ::std::fs::remove_file(path).unwrap();
    }
}