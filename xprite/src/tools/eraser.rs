use crate::prelude::*;
use crate::algorithms::line::continuous_line;

#[derive(Debug)]
pub struct Eraser {
    is_mouse_down: Option<InputItem>,
    current_polyline: Polyline,
    cursor: Option<Pixels>,
    cursor_pos: Option<Vec2f>,
    pub brush: Brush,
    update_buffer: Option<Pixels>,
    draw_buffer: Pixels,
    last_mouse_down_or_up: Option<Vec2f>,
    shift: bool,
}

impl Default for Eraser {
    fn default() -> Self {
        Self::new()
    }
}

impl Eraser {
    pub fn new() -> Self {
        let is_mouse_down = None;
        let cursor = None;
        let cursor_pos = None;
        let brush = Brush::circle(1, Color::orange());
        let current_polyline = Polyline::new();

        Self {
            shift: false,
            is_mouse_down,
            current_polyline,
            cursor,
            last_mouse_down_or_up: None,
            cursor_pos,
            brush,
            update_buffer: None,
            draw_buffer: Pixels::new(),
        }
    }

    fn erase_stroke(&self, xpr: &Xprite) -> Result<Pixels, String> {
        let mut line_pixs = self.current_polyline.to_pixel_coords(xpr)?.connect_with_line(xpr.color())?;
        line_pixs.push(Pixel{ point: self.cursor_pos.unwrap(), color: Color::void()});
        let brushstroke = self.brush.follow_stroke(&line_pixs).unwrap();
        Ok(brushstroke)
    }

    fn finalize(&mut self, xpr: &Xprite) -> Result<(), String> {
        let stroke = self.erase_stroke(xpr)?;
        self.update_buffer = Some(stroke);
        Ok(())
    }

    fn draw_line(&self, color: Color) -> Option<Pixels> {
        let buf = continuous_line(self.last_mouse_down_or_up?, self.cursor_pos?, color);
        let buf = self.brush.follow_stroke(&buf)?;
        Some(buf)
    }

    fn finalize_continuous_line(&mut self, xpr: &Xprite, start: Option<Vec2f>, stop: Option<Vec2f>) -> Result<(), String> {
        if let (Some(start), Some(stop)) = (start, stop) {
            let buf = continuous_line(start, stop, xpr.color());
            let buf = self.brush.follow_stroke(&buf).unwrap();
            self.update_buffer = Some(buf);
        }
        Ok(())
    }

}

impl Tool for Eraser {

    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        self.cursor = self.brush.to_canvas_pixels(point, xpr.color());
        self.cursor_pos = Some(point);

        if self.shift {
            if let Some(pixs) = self.draw_line(xpr.color()) {
                self.draw_buffer = pixs;
                return Ok(());
            }
        }

        // if mouse is done
        if self.is_mouse_down.is_none() || self.cursor.is_none() {
            return Ok(());
        }

        self.current_polyline.push(p);

        let stroke = self.erase_stroke(xpr)?;
        self.draw_buffer.extend(&stroke);

        // let pixels = self.brush.to_canvas_pixels(p, xpr.color());
        // if let Some(pixels) = pixels {
        //     self.buffer.extend(&pixels);
        // }
        Ok(())
    }

    fn mouse_down(&mut self, xpr: &Xprite, p: Vec2f, button: InputItem) -> Result<(), String> {
        self.is_mouse_down = Some(button);
        self.current_polyline.push(p);

        let pixels = self.brush.to_canvas_pixels(xpr.canvas.shrink_size(p), xpr.color());
        if let Some(pixels) = pixels {
            if button == InputItem::Left {
                self.draw_buffer.extend(&pixels);
            } else {
                // xpr.remove_pixels(&pixels);
            }
        }
        Ok(())
    }

    fn mouse_up(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        if self.is_mouse_down.is_none() {
            return Ok(());
        }

        let prev = self.last_mouse_down_or_up;
        let point = xpr.canvas.shrink_size(p);
        self.last_mouse_down_or_up = Some(point);

        let button = self.is_mouse_down.unwrap();
        if button == InputItem::Right {
            return Ok(());
        }
        if self.shift {
            self.finalize_continuous_line(xpr, prev, self.cursor_pos)?;
        } else {
            self.finalize(xpr)?;
        }
        self.current_polyline.clear();
        self.draw_buffer.clear();
        self.is_mouse_down = None;
        Ok(())
    }

    fn update(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        if let Some(pixs) = &self.update_buffer {
            xpr.history.enter()?;
            let reflected = xpr.toolbox.symmetry.clone().borrow().process(&pixs);
            let l = xpr.current_layer().unwrap();
            let layer_mut = &mut l.borrow_mut();
            layer_mut.content.sub_mut(&reflected);
            layer_mut.content.sub_mut(&pixs);
            layer_mut.visible = true;
            self.update_buffer = None;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        xpr.new_frame();
        if let Some(cursor) = &self.cursor {
            xpr.set_cursor(cursor);
        }

        let l = xpr.current_layer().unwrap();
        let mut layer = l.borrow_mut();
        if !self.draw_buffer.is_empty() {
            // set current layer to invisible
            layer.visible = false;
            // let content = layer.content.clone(); // HACK: doesn't borrowck
            xpr.add_pixels(&layer.content);
            xpr.remove_pixels(&self.draw_buffer);
            let reflected = xpr.toolbox.symmetry.clone().borrow().process(&self.draw_buffer);
            xpr.remove_pixels(&reflected);
            Ok(true)
        } else {
            layer.visible = true;
            Ok(false)
        }
    }

    fn set(&mut self, xpr: &Xprite, option: &str, value: &str) -> Result<(), String> {
        match option {
            "mode" => {}
            "brush" => {
                let brush = value.parse()?;
                self.brush = brush;
            }
            "LShift" | "RShift" => match value {
                "true" => {
                    self.shift = true;
                    if let Some(pixs) = self.draw_line(xpr.color()) {
                        self.draw_buffer = pixs;
                    }
                }
                "false" => {
                    self.shift = false;
                    self.draw_buffer.clear();
                }
                _ => error!("malformed value: {}", value),
            },
            _ => (),
        }
        Ok(())
    }
}
