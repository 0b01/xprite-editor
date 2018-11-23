use crate::prelude::*;
use crate::algorithms::sorter::sort_path;
use crate::algorithms::pixel_perfect::pixel_perfect;

type Circle = Pixels;

pub struct Vector {
    is_mouse_down: Option<InputItem>,
    cursor_pos: Option<Pixel>,
    brush: Brush,
    tolerence: f32,
    buffer: Pixels,
    current_polyline: Polyline,
}

impl Vector {
    pub fn new() -> Self {
        let is_mouse_down = None;
        let cursor_pos = None;
        let brush = Brush::pixel();
        let current_polyline = Polyline::new();
        let buffer = Pixels::new();

        Self {
            is_mouse_down,
            current_polyline,
            cursor_pos,
            brush,
            buffer,
            tolerence: 2.,
        }
    }

    pub fn draw_polyline(&mut self, xpr: &mut Xprite, polyline: &Polyline) -> (Pixels, Circle) {
        let mut circ_buf = Pixels::new();
        let path = polyline.interp();
        let mut rasterized = path.rasterize(xpr).unwrap();
        rasterized.set_color(&Color::orange());
        // self.buffer.extend(&pixels);

        // plot anchors
        for &p in polyline.pos.iter() {
            let Point2D{x, y} = xpr.canvas.shrink_size_no_floor(&p);
            circ_buf.push(pixel!(x, y, Color::blue()));
        }

        // plot control points
        for seg in &path.segments {
            let CubicBezierSegment { ctrl1, ctrl2, .. } = seg;
            for p in vec![ctrl1, ctrl2] {
                let Point2D{x, y} = xpr.canvas.shrink_size_no_floor(p);
                circ_buf.push(pixel!(x, y, Color::red()));
            }
        }

        (rasterized, circ_buf)
    }

    /// convert brush shape to actual pixel on canvas
    pub fn brush2pixs(&self, xpr: &Xprite, cursor: Point2D<f32>, color: Color) -> Option<Pixels> {
        let Point2D {x, y} = xpr.canvas.shrink_size(&cursor);

        let (brush_w, brush_h) = self.brush.size;

        if (x + brush_w) >= xpr.canvas.art_w || (y + brush_h) >= xpr.canvas.art_h {
            None
        } else {
            let (offset_x, offset_y) = self.brush.offset;
            let ret: Vec<Pixel> = self.brush.shape.iter().map(
                |Pixel {point,..}| Pixel {
                    point: Point2D::new(point.x+x + offset_x, point.y+y + offset_y),
                    color: ColorOption::Set(color),
                }
            ).collect();
            Some(Pixels::from_slice(&ret))
        }
    }

}

impl Tool for Vector {

    fn tool_type(&self) -> ToolType {
        ToolType::Vector
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Point2D<f32>) -> Option<()> {
        // update cursor pos
        let pixels = self.brush2pixs(xpr, p, xpr.color());
        let point = xpr.canvas.shrink_size(&p);
        let color = ColorOption::Set(xpr.color());
        self.cursor_pos = Some(Pixel{point, color});

        if self.is_mouse_down.is_none() || pixels.is_none() {
            return self.draw(xpr);
        }

        // the rest handles when left button is pressed
        self.current_polyline.push(p);

        let button = self.is_mouse_down.clone().unwrap();
        if button == InputItem::Left {
            let line_pixs = self.current_polyline.connect_with_line(&xpr)?;
            let pixs = {
                let perfect = pixel_perfect(&line_pixs);
                Pixels::from_slice(&perfect)
            };
            self.buffer.extend(&pixs);
        } else if button == InputItem::Right {
            // xpr.remove_pixels(&pixels.unwrap());
        }
        self.draw(xpr)
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, p: Point2D<f32>, button: InputItem) -> Option<()>{
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

    fn mouse_up(&mut self, xpr: &mut Xprite, _p: Point2D<f32>) -> Option<()> {
        if self.is_mouse_down.is_none() {return Some(()); }
        let button = self.is_mouse_down.clone().unwrap();
        if button == InputItem::Right { return Some(()); }

        self.buffer.clear();
        let simple = self.current_polyline.reumann_witkam(self.tolerence)?;
        let (pixs, circ_buf) = self.draw_polyline(xpr, &simple);
        self.buffer.extend(&pixs);

        xpr.history.enter()?;
        xpr.history.top()
            .selected_layer
            .borrow_mut()
            .content
            .extend(&self.buffer);

        xpr.draw_circ_buf.extend(&circ_buf);

        self.current_polyline.clear();
        self.buffer.clear();
        self.is_mouse_down = None;

        self.draw(xpr);
        Some(())
    }

    fn draw(&self, xpr: &mut Xprite) -> Option<()> {
        xpr.new_frame();
        xpr.add_pixels(&self.buffer);

        Some(())
    }

    fn set(&mut self, _xpr: &mut Xprite, option: &str, value: &str) -> Option<()> {
        match option {
            "tolerence" => {
                if let Ok(val) = value.parse() {
                    self.tolerence = val;
                } else {
                    panic!("cannot parse val: {}", value);
                }
            }
            "brush" => {
                match value {
                    "cross" => self.brush = Brush::cross(),
                    "pixel" => self.brush = Brush::pixel(),
                    _ => panic!("malformed value: {}", value),
                }
            }
            _ => (),
        }
        Some(())
    }
}
