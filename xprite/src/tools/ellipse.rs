use crate::algorithms::ellipse::*;
use crate::tools::*;

#[derive(Clone, Default, Debug)]
pub struct EllipseInfo {
    pub radius_major: f64,
    pub radius_minor: f64,
    pub aspect_ratio: f64,
    pub angle: f64,
    pub top_left: Vec2f,
    pub bottom_right: Vec2f,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Default, Debug)]
pub struct Ellipse {
    is_mouse_down: Option<InputItem>,
    cursor_pos: Option<Vec2f>,
    start_pos: Option<Vec2f>,
    snap: bool,
    symmetric: bool,
    pub filled: bool,
    buffer: Option<Pixels>,
}

impl Ellipse {
    pub fn new() -> Self {
        Ellipse {
            is_mouse_down: None,
            cursor_pos: None,
            start_pos: None,
            snap: false,
            symmetric: false,
            filled: false,
            buffer: None,
        }
    }

    pub fn get_info(&self) -> Option<EllipseInfo> {
        let start = self.start_pos?;
        let stop = self.cursor_pos?;

        let p1 = self.process_snap(start, stop);
        let p0 = self.process_symmetry(start, p1);

        let bb = Rect(p0, p1);
        let height = bb.h();
        let width = bb.w();
        let angle = (height / width).atan();
        let radius_major = f64::max(width, height) / 2.;
        let radius_minor = f64::min(width, height) / 2.;
        let aspect_ratio = width / height;

        Some(EllipseInfo {
            radius_major,
            radius_minor,
            aspect_ratio,
            angle,
            top_left: p0,
            bottom_right: p1,
            width,
            height,
        })
    }

    fn get_ellipse(&self, color: Color) -> Result<Pixels, String> {
        if let (Some(start), Some(stop)) = (self.start_pos, self.cursor_pos) {
            let end = self.process_snap(start, stop);
            let begin_pos = self.process_symmetry(start, end);
            get_ellipse(Some(begin_pos), Some(end), self.filled, color)
        } else {
            Err("start or end is none".to_owned())
        }
    }

    fn process_snap(&self, start: Vec2f, stop: Vec2f) -> Vec2f {
        if self.snap {
            let x0 = start.x;
            let y0 = start.y;
            let x1 = stop.x;
            let y1 = stop.y;
            let dx = x1 - x0;
            let dy = y1 - y0;
            let d = f64::min(dx, dy);
            let mut end = start;
            end.x = start.x + d;
            end.y = start.y + d;
            end
        } else {
            stop
        }
    }

    fn process_symmetry(&self, start: Vec2f, end: Vec2f) -> Vec2f {
        if self.symmetric {
            let x = start.x - (end.x - start.x);
            let y = start.y - (end.y - start.y);
            vec2f_xy! {x, y}
        } else {
            self.start_pos.unwrap()
        }
    }

    fn finalize_ellipse(&mut self, xpr: &Xprite) -> Result<bool, String> {
        if let Ok(pixs) = self.get_ellipse(xpr.color()) {
            if pixs.is_empty() {
                return Ok(false);
            }
            self.buffer = Some(pixs);
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl Tool for Ellipse {
    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        // set current cursor_pos
        let point = xpr.canvas.shrink_size(p);
        self.cursor_pos = Some(point);
        Ok(())
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Vec2f) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        self.cursor_pos = Some(point);
        self.finalize_ellipse(xpr)?;
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
        self.start_pos = Some(point);
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
            xpr.set_cursor(&pixels!(pixel!(p, xpr.color())));
        }
        if let Ok(pixs) = self.get_ellipse(xpr.color()) {
            xpr.add_pixels(&pixs);
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn set(&mut self, _xpr: &Xprite, option: &str, value: &str) -> Result<(), String> {
        match option {
            "LControl" | "RControl" => match value {
                "true" => self.symmetric = true,
                "false" => self.symmetric = false,
                _ => error!("unimpl for ctrl: {}", value),
            },
            "LShift" | "RShift" => match value {
                "true" => {
                    self.snap = true;
                }
                "false" => {
                    self.snap = false;
                }
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

#[cfg(test)]
mod tests {
    #[test]
    fn test_ellipse_info() {
        use super::*;
        let mut ell = Ellipse::new();
        ell.start_pos = Some(vec2f!(0, 0));
        ell.cursor_pos = Some(vec2f!(10, 10));

        dbg!(ell.get_info());
    }
}
