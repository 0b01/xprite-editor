use crate::prelude::*;
use crate::rendering::{MouseCursorType, Renderer};
use img::GenericImage;
use img::{DynamicImage, Rgba};

pub struct ImageRenderer {
    color: Color,
    w: f64,
    h: f64,
    pub image: image::DynamicImage,
    draw_list: Pixels,
}

#[allow(unused)]
impl Renderer for ImageRenderer {
    fn width(&self) -> f64 {
        self.w
    }

    fn height(&self) -> f64 {
        self.h
    }

    fn rect(&mut self, p0: [f64; 2], p1: [f64; 2], color: [f32; 4], filled: bool) {}

    fn pixel(&mut self, x: f64, y: f64, color: [f32; 4], filled: bool) {
        self.draw_list.push(pixel!(y, x, color.into()));
    }

    fn circ(&mut self, p0: [f64; 2], r: f64, color: [f32; 4], filled: bool) {}

    fn line(&mut self, p0: [f64; 2], p1: [f64; 2], color: [f32; 4]) {}

    fn bezier(&mut self, p0: [f64; 2], cp1: [f64; 2], cp2: [f64; 2], p1: [f64; 2], color: [f32; 4], thickness: f64) {}

    fn set_mouse_cursor(&mut self, cursor_type: MouseCursorType) {}

    fn render(&mut self, xpr: Option<&Xprite>) -> Option<()> {
        for Pixel { point, color } in self.draw_list.iter() {
            let color: Rgba<u8> = {
                Rgba {
                    data: color.to_rgba(xpr)?.into(),
                }
            };
            if !oob(point.x, point.y, self.w, self.h) {
                let x = point.x as u32;
                let y = point.y as u32;
                self.image.put_pixel(x, y, color);
            }
        }
        Some(())
    }
}

impl ImageRenderer {
    pub fn new(color: Color, art_w: f64, art_h: f64) -> Self {
        let w = art_w;
        let h = art_h;
        let image = DynamicImage::new_rgba8(w as u32, h as u32);
        let draw_list = Pixels::new();
        Self { w, h, image, draw_list, color }
    }

    pub fn as_img(&self) -> &DynamicImage {
        &self.image
    }

    pub fn into_img(self) -> DynamicImage {
        self.image
    }

    pub fn fill_canvas(&mut self) {
        use img::Pixel;
        let rgba = unsafe { self.color.as_rgba() };
        let color = Rgba::from_channels(rgba.r, rgba.g, rgba.b, 255);

        for (_i, p) in self.image.as_mut_rgba8().unwrap().pixels_mut().enumerate() {
            *p = color;
        }
    }
}

pub fn save_img(path: &str, im: &DynamicImage) {
    info!("writing file to {}", path);
    // let mut f = std::fs::File::create(path).unwrap();
    im.save(path).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_img_render() {
        use super::*;
        let mut rdr = ImageRenderer::new(Color::orange(), 10., 10.);
        rdr.rect([0., 0.], [0., 0.], [1., 0., 0., 1.], true);
        let path = "test.png";
        save_img(path, rdr.as_img());
        std::fs::remove_file(path).unwrap();
    }
}
