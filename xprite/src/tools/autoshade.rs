use crate::algorithms::autoshade::{autoshade, AutoshadeStepParam};
use crate::core::outline::outline_rect;
use crate::algorithms::rect::get_rect;
use crate::tools::*;

#[derive(Clone, Default, Debug)]
pub struct AutoShade {
    is_mouse_down: Option<InputItem>,
    cursor_pos: Option<Pixel>,
    start_pos: Option<Pixel>,
    pub steps: Vec<AutoshadeStepParam>,
    buf: Pixels,
}

impl AutoShade {
    pub fn new() -> Self {
        AutoShade {
            is_mouse_down: None,
            start_pos: None,
            cursor_pos: None,
            steps: vec![],
            buf: Pixels::new(),
        }
    }

    pub fn finalize(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        let pixs = get_rect(self.start_pos, self.cursor_pos, true)?;
        let content = &mut xpr.current_layer_mut().unwrap().content;
        let intersection = content.intersection(&pixs); // TODO: don't construct a rect, filter based on w, h directly
        // let _bb = intersection.bounding_rect();
        let shaded = autoshade(&intersection, true, &self.steps);
        self.buf = shaded;
        Ok(())
    }

    pub fn get_bb(&self) -> Option<Rect> {
        let (p0, p1) = (self.start_pos?, self.cursor_pos?);
        let bb = Rect(p0.point, p1.point);
        Some(bb)
    }
}

impl Tool for AutoShade {
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
        let mut ret = false;
        if !self.buf.is_empty() {
            xpr.add_pixels(&self.buf);
            ret = true;
        }
        if self.start_pos.is_some() && self.cursor_pos.is_some() {
            if let Ok(marq) = outline_rect(self.start_pos.unwrap().point, self.cursor_pos.unwrap().point) {
                xpr.add_marquee(&marq);
                ret = true;
            }
        }
        Ok(ret)
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
