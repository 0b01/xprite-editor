use crate::prelude::*;
use crate::rendering::{Renderer, MouseCursorType};
use img::{DynamicImage, Rgba};
use img::GenericImage;

pub struct ImageRenderer {
    w: u32,
    h: u32,
    pub image: image::DynamicImage,
    draw_list: Pixels,
}

#[allow(unused)]
impl Renderer for ImageRenderer {

    fn width(&self) -> u32 { self.w }

    fn height(&self) -> u32 { self.h }

    fn circ(&mut self, p0:[f32;2], r:f32, color:[f32;4], filled: bool) { }

    fn bezier(&mut self, p0:[f32;2], cp1:[f32;2], cp2: [f32;2], p1:[f32;2], color:[f32;4], thickness: f32) { }

    fn rect(&mut self, p0:[f32;2], p1:[f32;2], color:[f32;4], filled: bool) {
        self.draw_list.push(pixel!(p0[0], p0[1], color.into()));
    }

    fn line(&mut self, p0:[f32;2], p1:[f32;2], color:[f32;4]) { }

    fn set_mouse_cursor(&mut self, cursor_type: MouseCursorType) { }

    fn render(&mut self) {
        for Pixel{point, color} in self.draw_list.iter() {
            let color = {
                Rgba { data: [color.r, color.g, color.b, color.a] }
            };
            self.image.put_pixel(
                point.x as u32,
                point.y as u32,
                color
            );
        }

    }
}

impl ImageRenderer {
    pub fn new(art_w: f32, art_h: f32) -> Self {
        let w = art_w as u32;
        let h = art_h as u32;
        let image = DynamicImage::new_rgba8(w, h);
        let draw_list = Pixels::new();
        Self {
            w,
            h,
            image,
            draw_list,
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