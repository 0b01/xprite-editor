use crate::prelude::*;

#[derive(Debug)]
pub struct Eraser {
    is_mouse_down: Option<InputItem>,
    current_polyline: Polyline,
    cursor: Option<Pixels>,
    cursor_pos: Option<Pixel>,
    brush: Brush,
    pub brush_type: BrushType,
    buffer: Pixels,
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
        self.cursor.as_ref().map(|cursor| {
            xpr.set_cursor(cursor);
        })
    }
}

impl Tool for Eraser {

    fn tool_type(&self) -> ToolType {
        ToolType::Eraser
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Vec2D) -> Result<(), String> {
        let pixels = self.brush.to_canvas_pixels(xpr.canvas.shrink_size(p), xpr.color());
        self.cursor = pixels.clone();
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel{point, color});

        // if mouse is done
        if self.is_mouse_down.is_none() || pixels.is_none() {
            return self.draw(xpr)
        }

        self.current_polyline.push(p);
        let line_pixs = self.current_polyline.connect_with_line(&xpr)?;
        let brushstroke = self.brush.follow_stroke(&line_pixs).unwrap();
        self.buffer.extend(&brushstroke);

        // let pixels = self.brush.to_canvas_pixels(p, xpr.color());
        // if let Some(pixels) = pixels {
        //     self.buffer.extend(&pixels);
        // }

        self.draw(xpr)
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, p: Vec2D, button: InputItem) -> Result<(), String>{
        self.is_mouse_down = Some(button);
        self.current_polyline.push(p);

        self.buffer.clear();
        let pixels = self.brush.to_canvas_pixels(xpr.canvas.shrink_size(p), xpr.color());
        if let Some(pixels) = pixels {
            if button == InputItem::Left {
                self.buffer.extend(&pixels);
            } else {
                // xpr.remove_pixels(&pixels);
            }
        }
        self.draw(xpr)
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, _p: Vec2D) -> Result<(), String> {
        if self.is_mouse_down.is_none() {return Ok(()); }
        let button = self.is_mouse_down.unwrap();
        if button == InputItem::Right { return Ok(()); }

        xpr.history.enter()?;
        {
            let layer = &mut xpr.current_layer_mut().unwrap();
            layer.content.sub(&self.buffer);
            layer.visible = true;
        }

        self.current_polyline.clear();
        self.buffer.clear();
        self.is_mouse_down = None;

        self.draw(xpr)?;
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        xpr.new_frame();
        self.set_cursor(xpr);

        let layer = xpr.current_layer_mut().unwrap();
        if !self.buffer.0.is_empty() {
            layer.visible = false;
            // set current layer to invisible
            let content = layer.content.clone(); // HACK: doesn't borrowck
            xpr.add_pixels(&content);
            xpr.remove_pixels(&self.buffer);
        } else {
            layer.visible = true;
        }
        Ok(())
    }

    fn set(&mut self, _xpr: &mut Xprite, option: &str, value: &str) -> Result<(), String> {
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
        Ok(())
    }
}
