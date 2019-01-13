use crate::prelude::*;

#[derive(Debug)]
pub struct Vector {
    is_mouse_down: Option<InputItem>,
    cursor_pos: Option<Pixel>,
    brush: Brush,
    pub tolerence: f32,
    pixs_buf: Pixels,
    current_polyline: Option<Polyline>,
    pub draw_bezier: bool,
    pub sort: bool,
}

impl Default for Vector {
    fn default() -> Self {
        Self::new()
    }
}

impl Vector {
    pub fn new() -> Self {
        let is_mouse_down = None;
        let cursor_pos = None;
        let brush = Brush::pixel();
        let current_polyline = Some(Polyline::new());
        let pixs_buf = Pixels::new();

        Self {
            is_mouse_down,
            current_polyline,
            cursor_pos,
            brush,
            pixs_buf,
            tolerence: 1.,
            draw_bezier: true,
            sort: true,
        }
    }
}

impl Tool for Vector {

    fn tool_type(&self) -> ToolType {
        ToolType::Vector
    }

    fn cursor(&self) -> Option<Pixels> {
        let p = self.cursor_pos?;
        Some(pixels!(p))
    }

    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2D) -> Result<(), String> {
        // update cursor pos
        let pixels = self.brush.to_canvas_pixels(xpr.canvas.shrink_size(p), xpr.color());
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel{point, color});

        if self.is_mouse_down.is_none() || pixels.is_none() {
            return Ok(())
        }

        // the rest handles when left button is pressed
        let p = xpr.canvas.shrink_size_no_floor(p);
        self.current_polyline.as_mut().ok_or_else(||"cannot borrow as mut")?.push(p);

        // let button = self.is_mouse_down.clone().unwrap();
        // if button == InputItem::Left {
        //     let line_pixs = self.current_polyline.as_mut()?.connect_with_line(&xpr)?;
        //     let pixs = {
        //         let perfect = pixel_perfect(&line_pixs);
        //         Pixels::from_slice(&perfect)
        //     };
        //     self.pixs_buf.extend(&pixs);
        // } else if button == InputItem::Right {
        //     // xpr.remove_pixels(&pixels.unwrap());
        // }
        Ok(())
    }

    fn mouse_down(&mut self, xpr: &Xprite, p: Vec2D, button: InputItem) -> Result<(), String>{
        self.is_mouse_down = Some(button);

        let p = xpr.canvas.shrink_size_no_floor(p);
        self.current_polyline.as_mut().ok_or_else(||"cannot borrow as mut".to_owned())?.push(p);
        // self.pixs_buf.clear();
        // let pixels = self.to_canvas_pixels(xpr, xpr.canvas.shrink_size(p), xpr.color());
        // if let Some(pixels) = pixels {
        //     if button == InputItem::Left {
        //         self.pixs_buf.extend(&pixels);
        //     } else {
        //         // xpr.remove_pixels(&pixels);
        //     }
        // }
        Ok(())
    }

    fn mouse_up(&mut self, xpr: &Xprite, _p: Vec2D) -> Result<(), String> {
        if self.is_mouse_down.is_none() {return Ok(()); }
        let button = self.is_mouse_down.unwrap();
        if button == InputItem::Right { return Ok(()); }

        // xpr.history.enter()?;
        // // commit pixels
        // xpr.history.top()
        //     .selected_layer
        //     .borrow_mut()
        //     .content
        //     .extend(&self.pixs_buf);

        // xpr.history.top()
        //     .selected_layer
        //     .borrow_mut()
        //     .paths
        //     .push((simple.clone(), path));

        // self.current_polyline.clear();
        // self.pixs_buf.clear();

        self.is_mouse_down = None;
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        xpr.new_frame();
        self.set_cursor(xpr);
        self.pixs_buf.clear();
        if let Ok(simple) = self.current_polyline.as_ref()
            .ok_or_else(||"cannot borrow as mut".to_owned())?
            .reumann_witkam(self.tolerence) {

            let (path, pixs_buf) = {
                let path = simple.interp();
                let mut rasterized = path.rasterize(xpr, self.sort).unwrap();
                rasterized.set_color(&Color::orange());
                (path, rasterized)
            };

            self.pixs_buf.extend(&pixs_buf);
            if self.draw_bezier {
                xpr.bz_buf.extend(path.segments);
            }

            xpr.add_pixels(&self.pixs_buf);
        }

        Ok(())
    }

    fn set(&mut self, _xpr: &Xprite, option: &str, value: &str) -> Result<(), String> {
        match option {
            "tolerence" => {
                if let Ok(val) = value.parse() {
                    self.tolerence = val;
                } else {
                    error!("cannot parse val: {}", value);
                }
            }
            "brush" => {
                match value {
                    "cross" => self.brush = Brush::cross(),
                    "pixel" => self.brush = Brush::pixel(),
                    _ => error!("malformed value: {}", value),
                }
            }
            _ => (),
        }
        Ok(())
    }
}
