use crate::tools::*;
use crate::algorithms::rect::*;

#[derive(Clone, Default)]
pub struct Rect {
    is_mouse_down: Option<InputItem>,
    cursor_pos: Option<Pixel>,
    start_pos: Option<Pixel>,
    snap: bool,
    symmetric: bool,
    pub filled: bool,
}

impl Rect {
    pub fn new() -> Self {
        Rect {
            is_mouse_down: None,
            cursor_pos: None,
            start_pos: None,
            snap: false,
            symmetric: false,
            filled: false,
        }
    }

    fn set_cursor(&self, xpr: &mut Xprite) -> Option<()> {
        if let Some(pix) = self.cursor_pos {
            let c = pixel!(pix.point.x, pix.point.y, Color::red());
            let mut pixels = Pixels::new();
            pixels.push(c);
            xpr.set_cursor(&pixels);
        }
        Some(())
    }

    fn get_rect(&self) -> Result<Pixels, String> {
        if let (Some(start), Some(stop)) = (self.start_pos, self.cursor_pos) {
            let end = if self.snap {
                let x0 = start.point.x;
                let y0 = start.point.y;
                let x1 = stop.point.x;
                let y1 = stop.point.y;
                let dx = x1 - x0;
                let dy = y1 - y0;
                let d = f32::min(dx, dy);
                let mut end = start.clone();
                end.point.x = start.point.x + d;
                end.point.y = start.point.y + d;
                end
            } else {
                stop
            };

            let begin_pos = if self.symmetric {
                let x = start.point.x - (end.point.x-start.point.x);
                let y = start.point.y - (end.point.y-start.point.y);
                Some(pixel!{x, y, Color::red()})
            } else {
                self.start_pos
            };

            get_rect(begin_pos, Some(end), self.filled)
        } else {
            Err("start or end is none".to_owned())
        }
    }

    fn finalize_rect(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        if let Ok(mut pixs) = self.get_rect() {
            xpr.history.enter()?;
            pixs.set_color(&xpr.color());
            xpr.current_layer_mut().unwrap().content.extend(&pixs);
        }
        Ok(())
    }

    fn draw_rect(&self, xpr: &mut Xprite) -> Result<(), String> {
        if let Ok(mut pixs) = self.get_rect() {
            pixs.set_color(&xpr.color());
            xpr.add_pixels(&pixs);
        }
        Ok(())
    }

}

impl Tool for Rect {

    fn tool_type(&self) -> ToolType {
        ToolType::Rect
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Vec2D) -> Result<(), String> {
        // set current cursor_pos
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel {point, color});
        self.draw(xpr)?;
        Ok(())
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Vec2D) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel {point, color});
        self.finalize_rect(xpr)?;
        self.is_mouse_down = None;
        self.start_pos = None;
        self.draw(xpr)?;
        Ok(())
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, p: Vec2D, button: InputItem) -> Result<(), String> {
        if InputItem::Left != button { return Ok(()); }
        self.is_mouse_down = Some(button);
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.start_pos = Some(Pixel{point, color});
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        xpr.new_frame();
        self.draw_rect(xpr).unwrap();
        self.set_cursor(xpr);
        Ok(())
    }

    fn set(&mut self, xpr: &mut Xprite, option: &str, value: &str) -> Result<(), String> {
        match option {
            "ctrl" => {
                match value {
                    "true" => { self.symmetric = true }
                    "false" => { self.symmetric = false }
                    _ => error!("unimpl for ctrl: {}", value)
                }
                self.draw(xpr)?;
            }
            "shift" => {
                match value {
                    "true" => { self.snap = true; }
                    "false" => { self.snap = false; }
                    _ => error!("unimpl for ctrl: {}", value)
                }
                self.draw(xpr)?;
            }
            "alt" => {
                info!("alt pressed (unimplemented)");
            }
            _ => (),
        }
        Ok(())
    }


}
