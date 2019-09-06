use crate::algorithms::line::*;
use crate::tools::*;

#[derive(Clone, Default, Debug)]
pub struct Line {
    is_mouse_down: Option<InputItem>,
    cursor_pos: Option<Pixel>,
    start_pos: Option<Pixel>,
    snap: bool,
    is_snap_45: bool,
    buffer: Option<Pixels>,
}

impl Line {
    pub fn new() -> Self {
        Line {
            is_mouse_down: None,
            cursor_pos: None,
            start_pos: None,
            snap: false,
            is_snap_45: false,
            buffer: None,
        }
    }

    fn get_line(&self, xpr: &Xprite) -> Option<Pixels> {
        let start = self.start_pos?;
        let stop = self.cursor_pos?;
        if self.snap {
            Some(snapped_line(self.is_snap_45, &start, &stop))
        } else {
            Some(continuous_line(start.point, stop.point, xpr.color()))
        }
    }

    fn finalize_line(&mut self, xpr: &Xprite) -> Result<(), String> {
        if let Some(mut pixs) = self.get_line(xpr) {
            pixs.set_color(xpr.color());
            self.buffer = Some(pixs);
        }
        Ok(())
    }
}

impl Tool for Line {
    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        // set current cursor_pos
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel { point, color });
        Ok(())
    }

    fn mouse_up(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel { point, color });
        self.finalize_line(xpr)?;
        self.is_mouse_down = None;
        self.start_pos = None;
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

    fn update(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        if let Some(pixs) = &self.buffer {
            xpr.finalize_pixels(&pixs)?;
            self.buffer = None;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        xpr.new_frame();

        if let Some(p) = self.cursor_pos {
            xpr.set_cursor(&pixels!(p));
        }

        if let Some(mut pixs) = self.get_line(xpr) {
            pixs.set_color(xpr.color());
            xpr.add_pixels(&pixs);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn set(&mut self, _xpr: &Xprite, option: &str, value: &str) -> Result<(), String> {
        match option {
            "LControl" | "RControl" => match value {
                "true" => {
                    self.snap = true;
                    self.is_snap_45 = true
                }
                "false" => self.snap = false,
                _ => error!("unimpl for ctrl: {}", value),
            },
            "LShift" | "RShift" => match value {
                "true" => {
                    self.snap = true;
                    self.is_snap_45 = false
                }
                "false" => self.snap = false,
                _ => error!("unimpl for ctrl: {}", value),
            },
            "alt" => {
                info!("alt pressed (unimplemented)");
            }
            _ => info!("unimplemented option: {}", option),
        }
        Ok(())
    }
}
