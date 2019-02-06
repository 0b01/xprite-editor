use crate::algorithms::rect::*;
use crate::tools::*;
use libtexsyn::{
    distance::l1,
    generators::patch::{Quilter, QuilterParams},
};

#[derive(Clone, Default, Debug)]
pub struct Texture {
    is_mouse_down: Option<InputItem>,
    cursor_pos: Option<Pixel>,
    start_pos: Option<Pixel>,
    pub blocksize: i32,
    pub overlap: i32,
    pub current_id: Option<usize>,
}

impl Texture {
    pub fn new() -> Self {
        Texture {
            is_mouse_down: None,
            start_pos: None,
            cursor_pos: None,
            blocksize: 12,
            overlap: 6,
            current_id: None,
        }
    }

    pub fn finalize(&mut self, xpr: &mut Xprite) -> Result<img::DynamicImage, String> {
        self.quilt_img(xpr)
    }

    pub fn get_dims(&self) -> Option<(f64, f64, (f64, f64))> {
        let x0 = self.start_pos?.point.x;
        let y0 = self.start_pos?.point.y;
        let x1 = self.cursor_pos?.point.x;
        let y1 = self.cursor_pos?.point.y;
        Some((
            (x1 - x0).abs(),
            (y1 - y0).abs(),
            (f64::min(x0, x1), f64::min(y0, y1)),
        ))
    }

    fn quilt_img(&mut self, xpr: &mut Xprite) -> Result<img::DynamicImage, String> {
        let mut pixs = get_rect(self.start_pos, self.cursor_pos, true)?;
        xpr.history.enter()?;
        pixs.set_color(xpr.color());
        let content = &mut xpr.current_layer_mut().unwrap().content;
        let intersection = content.intersection(&pixs);
        let (w, h, origin) = self
            .get_dims()
            .ok_or_else(|| "cannot get dimension".to_owned())?;
        let img = intersection.as_image(w, h, origin);

        let width = xpr.canvas.art_w as u32;
        let height = xpr.canvas.art_h as u32;
        let params = QuilterParams::new(
            (width, height),
            self.blocksize as u32,
            self.overlap as u32,
            None,
            None,
            l1,
        )?;
        let mut quilter = Quilter::new(img.to_rgb(), params);
        let res = quilter.quilt_image()?;
        Ok(img::DynamicImage::ImageRgb8(res))
    }
}

impl Tool for Texture {
    fn cursor(&self) -> Option<Pixels> {
        let p = self.cursor_pos?;
        Some(pixels!(p))
    }

    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        // set current cursor_pos
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        if self.is_mouse_down.is_some() {
            self.cursor_pos = Some(Pixel { point, color });
        }
        Ok(())
    }

    fn mouse_up(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel { point, color });
        // self.quilt_img(xpr)?;

        self.is_mouse_down = None;
        // self.start_pos = None;

        Ok(())
    }

    fn mouse_down(&mut self, xpr: &Xprite, p: Vec2f, button: InputItem) -> Result<(), String> {
        if InputItem::Left != button {
            return Ok(());
        }
        self.is_mouse_down = Some(button);
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.start_pos = Some(Pixel { point, color });
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        xpr.new_frame();
        if let Some(cursor) = self.cursor() {
            xpr.set_cursor(&cursor);
        }
        if let Ok(mut pixs) = get_rect(self.start_pos, self.cursor_pos, false) {
            pixs.set_color(xpr.color());
            xpr.add_pixels(&pixs);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn set(&mut self, _xpr: &Xprite, option: &str, value: &str) -> Result<(), String> {
        match option {
            "ctrl" => match value {
                _ => error!("unimpl for ctrl: {}", value),
            },
            "shift" => match value {
                _ => error!("unimpl for ctrl: {}", value),
            },
            "alt" => {
                info!("alt pressed (unimplemented)");
            }
            _ => (),
        }
        Ok(())
    }
}
