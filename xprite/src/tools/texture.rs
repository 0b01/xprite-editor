use crate::algorithms::rect::*;
use crate::core::outline::outline_rect;
use crate::tools::*;
use wfc_image::*;
use std::num::NonZeroU32;

#[derive(Clone, Default)]
pub struct Texture {
    is_mouse_down: Option<InputItem>,
    cursor_pos: Option<Pixel>,
    start_pos: Option<Pixel>,
    pub blocksize: i32,
    pub overlap: i32,
    pub tex: Option<(usize, img::DynamicImage)>,
}

impl Texture {
    pub fn new() -> Self {
        Texture {
            is_mouse_down: None,
            start_pos: None,
            cursor_pos: None,
            blocksize: 12,
            overlap: 6,
            tex: None,
        }
    }

    pub fn finalize(
        &mut self,
        xpr: &mut Xprite,
    ) -> Result<img::DynamicImage, String> {
        self.quilt_img(xpr)
    }

    pub fn get_bb(&self) -> Option<Rect> {
        let (p0, p1) = (self.start_pos?, self.cursor_pos?);
        let bb = Rect(p0.point, p1.point);
        Some(bb)
    }

    fn quilt_img(
        &mut self,
        xpr: &mut Xprite,
    ) -> Result<img::DynamicImage, String> {
        let mut pixs = get_rect(self.start_pos, self.cursor_pos, true)?;
        xpr.history.enter()?;
        pixs.set_color(xpr.color());
        let content = &mut xpr.current_layer_mut().unwrap().content;
        let intersection = content.intersection(&pixs);
        let bb = self.get_bb().unwrap(); // safe to unwrap because of above
        let img = intersection.as_image(bb);
        let _width = xpr.canvas.art_w as u32;
        let _height = xpr.canvas.art_h as u32;

        let orientation = orientation::ALL;
        let pattern_size = NonZeroU32::new(3)
            .expect("pattern size may not be zero");
        let output_size = Size::new(100, 100);
        let res = generate_image(
            &img,
            pattern_size,
            output_size,
            &orientation,
            wrap::WrapXY,
            retry::NumTimes(10),
        )
        .map_err(|_| "Too many contradictions".to_owned())?;

        Ok(res)
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

    fn mouse_down(
        &mut self,
        xpr: &Xprite,
        p: Vec2f,
        button: InputItem,
    ) -> Result<(), String> {
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
        if let Ok(marq) = outline_rect(self.start_pos, self.cursor_pos) {
            xpr.add_marquee(&marq);
            return Ok(true);
        }
        Ok(false)
    }

    fn set(
        &mut self,
        _xpr: &Xprite,
        option: &str,
        value: &str,
    ) -> Result<(), String> {
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
