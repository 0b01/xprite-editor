use crate::prelude::*;
use crate::algorithms::sorter::sort_path;
use crate::algorithms::pixel_perfect::pixel_perfect;
use std::rc::Rc;

pub struct Eraser {
    is_mouse_down: Option<InputItem>,
    current_polyline: Polyline,
    cursor: Option<Pixels>,
    cursor_pos: Option<Pixel>,
    brush: Brush,
    pub brush_type: BrushType,
    buffer: Pixels,
}

impl Eraser {
    pub fn new() -> Self {
        let is_mouse_down = None;
        let cursor = None;
        let cursor_pos = None;
        let brush_type = BrushType::Pixel;
        let brush = Brush::pixel();
        let buffer = Pixels::new();
        let current_polyline = Polyline::new();

        Self {
            is_mouse_down,
            current_polyline,
            cursor,
            cursor_pos,
            brush,
            brush_type,
            buffer,
        }
    }

    fn set_cursor(&self, xpr: &mut Xprite) -> Option<()> {
        if self.cursor.is_none() { return None; }
        let cursor = self.cursor.clone().unwrap();
        xpr.set_cursor(&cursor);
        Some(())
    }

    /// convert brush shape to actual pixel on canvas
    pub fn brush2pixs(&self, xpr: &Xprite, cursor: Vec2D, color: Color) -> Option<Pixels> {
        let Vec2D {x, y} = xpr.canvas.shrink_size(&cursor);

        let (brush_w, brush_h) = self.brush.size;

        if (x + brush_w) >= xpr.canvas.art_w || (y + brush_h) >= xpr.canvas.art_h {
            None
        } else {
            let (offset_x, offset_y) = self.brush.offset;
            let ret: Vec<Pixel> = self.brush.shape.iter().map(
                |Pixel {point,..}| Pixel {
                    point: Vec2D::new(point.x+x + offset_x, point.y+y + offset_y),
                    color: color,
                }
            ).collect();
            Some(Pixels::from_slice(&ret))
        }
    }

}

impl Tool for Eraser {

    fn tool_type(&self) -> ToolType {
        ToolType::Eraser
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Vec2D) -> Option<()> {
        let pixels = self.brush2pixs(xpr, p, xpr.color());
        self.cursor = pixels.clone();
        let point = xpr.canvas.shrink_size(&p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel{point, color});

        // if mouse is done
        if self.is_mouse_down.is_none() || pixels.is_none() {
            return self.draw(xpr);
        }

        self.current_polyline.push(p);
        let line_pixs = self.current_polyline.connect_with_line(&xpr)?;
        self.buffer.extend(&Pixels::from_slice(&line_pixs));
        let pixels = self.brush2pixs(xpr, p, xpr.color());
        if let Some(pixels) = pixels {
            self.buffer.extend(&pixels);
        }

        self.draw(xpr)
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, p: Vec2D, button: InputItem) -> Option<()>{
        self.is_mouse_down = Some(button);
        self.current_polyline.push(p);

        self.buffer.clear();
        let pixels = self.brush2pixs(xpr, p, xpr.color());
        if let Some(pixels) = pixels {
            if button == InputItem::Left {
                self.buffer.extend(&pixels);
            } else {
                // xpr.remove_pixels(&pixels);
            }
        }
        self.draw(xpr)
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, _p: Vec2D) -> Option<()> {
        if self.is_mouse_down.is_none() {return Some(()); }
        let button = self.is_mouse_down.clone().unwrap();
        if button == InputItem::Right { return Some(()); }

        xpr.history.enter()?;
        {
            let layer = &mut xpr.history.top_mut()
                .selected_layer
                .borrow_mut();
            layer.content.sub(&self.buffer);
        }

        self.buffer.clear();
        self.is_mouse_down = None;

        self.draw(xpr);
        Some(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Option<()> {
        xpr.new_frame();
        self.set_cursor(xpr);
        self.buffer.set_color(&xpr.color());

        // set current layer to invisible
        let layer = Rc::clone(&xpr.history.top_mut().selected_layer);
        layer.borrow_mut().visible = false;

        xpr.add_pixels(&layer.borrow_mut().content);
        xpr.remove_pixels(&self.buffer);

        Some(())
    }

    fn set(&mut self, _xpr: &mut Xprite, option: &str, value: &str) -> Option<()> {
        match option {
            "mode" => {
            }
            "brush" => {
                match value {
                    "+" => self.brush = Brush::cross(),
                    "." => self.brush = Brush::pixel(),
                    _ => error!("malformed value: {}", value),
                }
            }
            _ => (),
        }
        Some(())
    }
}
