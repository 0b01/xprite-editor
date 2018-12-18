use crate::tools::*;
use crate::algorithms::rect::*;

#[derive(Clone, Default)]
pub struct Texture {
    is_mouse_down: Option<InputItem>,
    cursor_pos: Option<Pixel>,
    start_pos: Option<Pixel>,
    snap: bool,
    is_snap_45: bool,
    pub filled: bool,
}

impl Texture {
    pub fn new() -> Self {
        Texture {
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

    fn finalize_line(&mut self, xpr: &mut Xprite) -> Option<()> {
        if let Some(mut pixs) = get_rect(self.start_pos, self.cursor_pos, self.filled) {
            xpr.history.enter()?;
            pixs.set_color(&xpr.color());
            // xpr.history.top().selected_layer.borrow_mut().content.extend(&pixs);
        }
        Some(())
    }

    fn draw_line(&self, xpr: &mut Xprite) -> Option<()> {
        if let Some(mut pixs) = get_rect(self.start_pos, self.cursor_pos, self.filled) {
            pixs.set_color(&xpr.color());
            // xpr.add_pixels(&pixs)
        }
        Some(())
    }

}

impl Tool for Texture {

    fn tool_type(&self) -> ToolType {
        ToolType::Texture
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Vec2D) -> Option<()> {
        // set current cursor_pos
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel {point, color});
        self.draw(xpr);
        Some(())
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Vec2D) -> Option<()> {
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel {point, color});
        self.finalize_line(xpr)?;
        self.is_mouse_down = None;
        self.start_pos = None;
        self.draw(xpr);
        Some(())
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, p: Vec2D, button: InputItem) -> Option<()> {
        if InputItem::Left != button { return Some(()); }
        self.is_mouse_down = Some(button);
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.start_pos = Some(Pixel{point, color});
        Some(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Option<()> {
        xpr.new_frame();
        self.draw_line(xpr);
        self.set_cursor(xpr);
        Some(())
    }

    fn set(&mut self, xpr: &mut Xprite, option: &str, value: &str) -> Option<()> {
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
        Some(())
    }


}
