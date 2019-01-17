use crate::prelude::*;

#[derive(Debug)]
pub struct Eraser {
    is_mouse_down: Option<InputItem>,
    current_polyline: Polyline,
    cursor: Option<Pixels>,
    cursor_pos: Option<Pixel>,
    brush: Brush,
    pub brush_type: BrushType,
    update_buffer: Option<Pixels>,
    draw_buffer: Pixels,
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
        let current_polyline = Polyline::new();

        Self {
            is_mouse_down,
            current_polyline,
            cursor,
            cursor_pos,
            brush,
            brush_type,
            update_buffer: None,
            draw_buffer: Pixels::new(),
        }
    }

    fn erase_stroke(&self, xpr: &Xprite) -> Result<Pixels, String> {
        let line_pixs = self.current_polyline.connect_with_line(&xpr)?;
        let brushstroke = self.brush.follow_stroke(&line_pixs).unwrap();
        Ok(brushstroke)
    }

    fn finalize(&mut self, xpr: &Xprite) -> Result<(), String> {
        let stroke = self.erase_stroke(xpr)?;
        self.update_buffer = Some(stroke);
        Ok(())
    }

}

impl Tool for Eraser {

    fn tool_type(&self) -> ToolType {
        ToolType::Eraser
    }

    fn cursor(&self) -> Option<Pixels> {
        let p = self.cursor_pos?;
        Some(pixels!(p))
    }

    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        let pixels = self.brush.to_canvas_pixels(xpr.canvas.shrink_size(p), xpr.color());
        self.cursor = pixels.clone();
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel{point, color});

        // if mouse is done
        if self.is_mouse_down.is_none() || pixels.is_none() {
            return Ok(())
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

    fn mouse_down(&mut self, xpr: &Xprite, p: Vec2f, button: InputItem) -> Result<(), String>{
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

    fn mouse_up(&mut self, xpr: &Xprite, _p: Vec2f) -> Result<(), String> {
        if self.is_mouse_down.is_none() {return Ok(()); }
        let button = self.is_mouse_down.unwrap();
        if button == InputItem::Right { return Ok(()); }
        self.finalize(xpr)?;
        self.current_polyline.clear();
        self.draw_buffer.clear();
        self.is_mouse_down = None;
        Ok(())
    }


    fn update(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        if let Some(pixs) = &self.update_buffer {
            xpr.history.enter()?;
            {
                let layer = &mut xpr.current_layer_mut().unwrap();
                layer.content.sub_(&pixs);
                layer.visible = true;
            }
        }
        self.update_buffer = None;
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        xpr.new_frame();
        self.set_cursor(xpr);

        let layer = xpr.current_layer_mut().unwrap();
        if !self.draw_buffer.is_empty() {
            layer.visible = false;
            // set current layer to invisible
            let content = layer.content.clone(); // HACK: doesn't borrowck
            xpr.add_pixels(&content);
            xpr.remove_pixels(&self.draw_buffer);
        } else {
            layer.visible = true;
        }
        Ok(())
    }

    fn set(&mut self, _xpr: &Xprite, option: &str, value: &str) -> Result<(), String> {
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
