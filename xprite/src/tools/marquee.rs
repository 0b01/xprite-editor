use crate::core::outline::outline_rect;
use crate::tools::*;

#[derive(Clone, Default, Debug)]
pub struct Marquee {
    is_mouse_down: Option<InputItem>,
    cursor_pos: Option<Pixel>,
    start_pos: Option<Pixel>,
}

impl Marquee {
    pub fn new() -> Self {
        Marquee {
            is_mouse_down: None,
            start_pos: None,
            cursor_pos: None,
        }
    }

    fn get_dims(&self) -> Option<(f64, f64, (f64, f64))> {
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

}

impl Tool for Marquee {
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
        if let Some(cursor) = self.cursor() { xpr.set_cursor(&cursor); }
        if let Ok(marq) = outline_rect(self.start_pos, self.cursor_pos) {
            xpr.add_marquee(&marq);
        }
        Ok(false)
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
