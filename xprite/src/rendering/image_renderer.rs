use crate::prelude::*;
use crate::rendering::{MouseCursorType, Renderer};
use img::GenericImage;
use img::{DynamicImage, Rgba};

pub struct ImageRenderer {
    w: f32,
    h: f32,
    pub image: image::DynamicImage,
    draw_list: Pixels,
}

#[allow(unused)]
impl Renderer for ImageRenderer {
    fn width(&self) -> f32 {
        self.w
    }

    fn height(&self) -> f32 {
        self.h
    }

    fn circ(&mut self, p0: [f32; 2], r: f32, color: [f32; 4], filled: bool) {}

    fn bezier(
        &mut self,
        p0: [f32; 2],
        cp1: [f32; 2],
        cp2: [f32; 2],
        p1: [f32; 2],
        color: [f32; 4],
        thickness: f32,
    ) {
    }

    fn rect(&mut self, p0: [f32; 2], p1: [f32; 2], color: [f32; 4], filled: bool) {
        ()
    }

    fn pixel(&mut self, x: f32, y: f32, color: [f32; 4], filled: bool) {
        self.draw_list.push(pixel!(y, x, color.into()));
    }

    fn line(&mut self, p0: [f32; 2], p1: [f32; 2], color: [f32; 4]) {}

    fn set_mouse_cursor(&mut self, cursor_type: MouseCursorType) {}

    fn render(&mut self) {
        for Pixel { point, color } in self.draw_list.iter() {
            let color = {
                Rgba {
                    data: [color.r, color.g, color.b, color.a],
                }
            };
            if !oob(point.x, point.y, self.w, self.h) {
                let x = point.x as u32;
                let y = point.y as u32;
                self.image.put_pixel(x, y, color);
            }
        }
    }
}

impl ImageRenderer {
    pub fn new(art_w: f32, art_h: f32) -> Self {
        let w = art_w;
        let h = art_h;
        let image = DynamicImage::new_rgba8(w as u32, h as u32);
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
    // let mut f = std::fs::File::create(path).unwrap();
    im.save(path).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_img_render() {
        use super::*;
        let mut rdr = ImageRenderer::new(10., 10.);
        rdr.rect([0., 0.], [0., 0.], [1., 0., 0., 1.], true);
        let path = "test.png";
        save_img(path, rdr.img());
        std::fs::remove_file(path).unwrap();
    }
}
