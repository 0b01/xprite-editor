use crate::algorithms::rect::*;
use crate::tools::*;

#[derive(Clone, Default, Debug)]
pub struct Rect {
    is_mouse_down: Option<InputItem>,
    cursor_pos: Option<Pixel>,
    start_pos: Option<Pixel>,
    snap: bool,
    symmetric: bool,
    pub filled: bool,
    buffer: Option<Pixels>,
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
            buffer: None,
        }
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
                let d = f64::min(dx, dy);
                let mut end = start;
                end.point.x = start.point.x + d;
                end.point.y = start.point.y + d;
                end
            } else {
                stop
            };

            let begin_pos = if self.symmetric {
                let x = start.point.x - (end.point.x - start.point.x);
                let y = start.point.y - (end.point.y - start.point.y);
                Some(pixel_xy! {x, y, Color::red()})
            } else {
                self.start_pos
            };

            get_rect(begin_pos, Some(end), self.filled)
        } else {
            Err("start or end is none".to_owned())
        }
    }

    fn finalize_rect(&mut self, xpr: &Xprite) -> Result<(), String> {
        if let Ok(mut pixs) = self.get_rect() {
            pixs.set_color(xpr.color());
            self.buffer = Some(pixs);
        }
        Ok(())
    }
}

impl Tool for Rect {
    fn cursor(&self) -> Option<Pixels> {
        let p = self.cursor_pos?;
        Some(pixels!(p))
    }

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
        self.finalize_rect(xpr)?;
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
            xpr.history.enter()?;
            xpr.current_layer_mut().unwrap().content.extend(&pixs);
            self.buffer = None;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        xpr.new_frame();
        if let Some(cursor) = self.cursor() {
            xpr.set_cursor(&cursor);
        }
        if let Ok(mut pixs) = self.get_rect() {
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
                "true" => self.symmetric = true,
                "false" => self.symmetric = false,
                _ => error!("unimpl for ctrl: {}", value),
            },
            "shift" => match value {
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
