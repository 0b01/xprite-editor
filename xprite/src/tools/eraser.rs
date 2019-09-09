use crate::prelude::*;
use crate::algorithms::line::continuous_line;

use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
pub struct BufferedPolyline {
    polyline: Polyline,
    upto: usize,
    buffer: Pixels,
}

impl BufferedPolyline {
    pub fn new() -> Self {
        Self {
            polyline: Polyline::new(),
            upto: 0,
            buffer: Pixels::new(),
        }
    }

    pub fn push(&mut self, point: Vec2f) {
        self.polyline.push(point);
    }

    pub fn process(&mut self, brush: &Brush, color: Color) -> Result<&Pixels, String> {
        self.upto += self.polyline.pos.len();
        let line_pixs = self.polyline.connect_with_line(color)?;
        let brushstroke = brush.follow_stroke(&line_pixs).unwrap();
        self.buffer.extend(&brushstroke);
        Ok(&self.buffer)
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.upto = 0;
        self.polyline.clear();
    }
}

#[derive(Debug)]
pub struct Eraser {
    is_mouse_down: Option<InputItem>,
    buffered_polyline: Rc<RefCell<BufferedPolyline>>,
    cursor: Option<Pixels>,
    cursor_pos: Option<Vec2f>,
    pub brush: Brush,
    draw_buffer: Pixels,
    last_mouse_down_or_up: Option<Vec2f>,
    finalized: Option<FinalizeType>,
    shift: bool,
}

impl Default for Eraser {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
enum FinalizeType {
    Line,
    Polyline,
}

impl Eraser {
    pub fn new() -> Self {
        let is_mouse_down = None;
        let cursor = None;
        let cursor_pos = None;
        let brush = Brush::circle(1, Color::orange());
        let buffered_polyline = Rc::new(RefCell::new(BufferedPolyline::new()));

        Self {
            shift: false,
            is_mouse_down,
            buffered_polyline,
            cursor,
            last_mouse_down_or_up: None,
            cursor_pos,
            brush,
            finalized: None,
            draw_buffer: Pixels::new(),
        }
    }



    fn draw_line(&self, color: Color) -> Option<Pixels> {
        let buf = continuous_line(self.last_mouse_down_or_up?, self.cursor_pos?, color);
        let buf = self.brush.follow_stroke(&buf)?;
        Some(buf)
    }

    fn erase(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        let color = xpr.color();

        let mut add = |pixs: &Pixels| {
            xpr.history.enter().unwrap();
            let reflected = xpr.toolbox.symmetry.clone().borrow().process(pixs);
            let l = xpr.current_layer().unwrap();
            let layer_mut = &mut l.borrow_mut();
            layer_mut.content.sub_mut(&reflected);
            layer_mut.content.sub_mut(pixs);
            layer_mut.visible = true;
        };

        let ret = match self.finalized {
            Some(FinalizeType::Line) => {
                let start = self.last_mouse_down_or_up;
                let stop = self.cursor_pos;
                if let (Some(start), Some(stop)) = (start, stop) {
                    let buf = continuous_line(start, stop, color);
                    add(&self.brush.follow_stroke(&buf).unwrap());
                    Ok(true)
                } else {
                    Ok(false)
                }
            }
            Some(FinalizeType::Polyline) => {
                let b = Rc::clone(&self.buffered_polyline);
                let mut c = b.borrow_mut();
                let buf = c.process(&self.brush, color);
                if let Ok(buf) = buf { add(buf); Ok(true) } else { Ok(false) }
            }
            None => Ok(false),
        };
        self.finalized = None;
        ret
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

        self.buffered_polyline.borrow_mut().push(point);

        let b = Rc::clone(&self.buffered_polyline);
        let mut c = b.borrow_mut();
        let stroke = c.process(&self.brush, xpr.color())?;
        self.draw_buffer.extend(&stroke);

        // let pixels = self.brush.to_canvas_pixels(p, xpr.color());
        // if let Some(pixels) = pixels {
        //     self.buffer.extend(&pixels);
        // }
        Ok(())
    }

    fn mouse_down(&mut self, xpr: &Xprite, p: Vec2f, button: InputItem) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        self.is_mouse_down = Some(button);
        self.buffered_polyline.borrow_mut().push(point);

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

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Vec2f) -> Result<(), String> {
        if self.is_mouse_down.is_none() {
            return Ok(());
        }

        let point = xpr.canvas.shrink_size(p);
        self.buffered_polyline.borrow_mut().push(point);

        let button = self.is_mouse_down.unwrap();
        if button == InputItem::Right {
            return Ok(());
        }
        if self.shift {
            self.finalized = Some(FinalizeType::Line);
        } else {
            self.finalized = Some(FinalizeType::Polyline);
        }

        self.erase(xpr).unwrap();

        self.last_mouse_down_or_up = Some(point);

        self.buffered_polyline.borrow_mut().clear();
        self.draw_buffer.clear();
        self.is_mouse_down = None;
        Ok(())
    }

    fn update(&mut self, _xpr: &mut Xprite) -> Result<bool, String> {
        Ok(false)
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        xpr.new_frame();
        if let Some(cursor) = &self.cursor {
            xpr.set_cursor(cursor);
        }

        let l = xpr.current_layer().unwrap();
        let mut layer = l.borrow_mut();
        if !self.draw_buffer.is_empty() {
            let reflected = xpr.toolbox.symmetry.clone().borrow().process(&self.draw_buffer);
            // set current layer to invisible
            layer.visible = false;
            xpr.add_pixels(&layer.content);
            xpr.remove_pixels(&self.draw_buffer);
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
