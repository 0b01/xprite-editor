use crate::tools::*;
use crate::algorithms::rect::*;

#[derive(Clone, Default)]
pub struct Rect {
    is_mouse_down: Option<InputItem>,
    cursor_pos: Option<Pixel>,
    start_pos: Option<Pixel>,
    snap: bool,
    is_snap_45: bool,
    pub filled: bool,
}

impl Rect {
    pub fn new() -> Self {
        Rect {
            is_mouse_down: None,
            cursor_pos: None,
            start_pos: None,
            snap: false,
            is_snap_45: false,
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

    fn finalize_rect(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        let mut pixs = get_rect(self.start_pos, self.cursor_pos, self.filled)?;
        xpr.history.enter()?;
        pixs.set_color(&xpr.color());
        xpr.history.top().selected_layer.borrow_mut().content.extend(&pixs);
        Ok(())
    }

    fn draw_rect(&self, xpr: &mut Xprite) -> Result<(), String> {
        let mut pixs = get_rect(self.start_pos, self.cursor_pos, self.filled)?;
        pixs.set_color(&xpr.color());
        xpr.add_pixels(&pixs);
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
        self.draw(xpr);
        Ok(())
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Vec2D) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel {point, color});
        self.finalize_rect(xpr)?;
        self.is_mouse_down = None;
        self.start_pos = None;
        self.draw(xpr);
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
        self.draw_rect(xpr);
        self.set_cursor(xpr);
        Ok(())
    }

    fn set(&mut self, xpr: &mut Xprite, option: &str, value: &str) -> Result<(), String> {
        match option {
            "ctrl" => {
                match value {
                    "true" => { self.snap = true; self.is_snap_45 = true }
                    "false" => { self.snap = false }
                    _ => error!("unimpl for ctrl: {}", value)
                }
                self.draw(xpr);
            }
            "shift" => {
                match value {
                    "true" => { self.snap = true; self.is_snap_45 = false }
                    "false" => { self.snap = false }
                    _ => error!("unimpl for ctrl: {}", value)
                }
                self.draw(xpr);
            }
            "alt" => {
                info!("alt pressed (unimplemented)");
            }
            _ => info!("unimplemented option: {}", option)
        }
        Ok(())
    }


}
