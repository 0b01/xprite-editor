use crate::tools::*;
use crate::algorithms::line::*;

#[derive(Clone)]
pub struct Rect {
    is_mouse_down: Option<InputItem>,
    cursor_pos: Option<Pixel>,
    start_pos: Option<Pixel>,
    snap: bool,
    is_snap_45: bool,
}

impl Rect {
    pub fn new() -> Self {
        Rect {
            is_mouse_down: None,
            cursor_pos: None,
            start_pos: None,
            snap: false,
            is_snap_45: false,
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

    fn get_line(&self) -> Option<Vec<Pixel>> {
        let start = self.start_pos?;
        let stop = self.cursor_pos?;
        if self.snap {
            Some(snapped_line(self.is_snap_45, &start, &stop))
        } else {
            Some(bresenham(&start.point.into(), &stop.point.into()))
        }
    }

    fn finalize_line(&mut self, xpr: &mut Xprite) -> Option<()> {
        if let Some(pixs) = self.get_line() {
            xpr.history.enter()?;
            xpr.history.top().selected_layer.borrow_mut().content.extend(&Pixels::from_slice(&pixs));
        }
        Some(())
    }

    fn draw_line(&self, xpr: &mut Xprite) -> Option<()> {
        if let Some(pixs) = self.get_line() {
            xpr.add_stroke(&pixs)
        }
        Some(())
    }

}

impl Tool for Rect {

    fn tool_type(&self) -> ToolType {
        ToolType::Rect
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Vec2D) -> Option<()> {
        // set current cursor_pos
        let point = xpr.canvas.shrink_size(&p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel {point, color});
        self.draw(xpr);
        Some(())
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Vec2D) -> Option<()> {
        let point = xpr.canvas.shrink_size(&p);
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
        let point = xpr.canvas.shrink_size(&p);
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