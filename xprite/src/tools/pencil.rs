use crate::prelude::*;
use crate::algorithms::sorter::sort_path;
use crate::algorithms::pixel_perfect::pixel_perfect;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum PencilMode {
    /// raw - no processing
    Raw,
    /// pixel perfect - nothing else
    PixelPerfect,
    /// sort each monotonic segment
    SortedMonotonic,
}

impl PencilMode {
    pub fn as_str(&self) -> &str {
        match self {
            PencilMode::Raw => "Raw",
            PencilMode::PixelPerfect => "Pixel Perfect",
            PencilMode::SortedMonotonic => "Sorted Monotonic",
        }
    }
    pub fn from_str(string: &str) -> Self {
        match string {
            "Raw" => PencilMode::Raw,
            "Pixel Perfect" => PencilMode::PixelPerfect,
            "Sorted Monotonic" => PencilMode::SortedMonotonic,
            _ => unimplemented!(),
        }
    }

    pub const VARIANTS: [PencilMode; 3] = [
        PencilMode::Raw,
        PencilMode::PixelPerfect,
        PencilMode::SortedMonotonic,
    ];
}

pub struct Pencil {
    is_mouse_down: Option<InputItem>,
    current_polyline: Polyline,
    cursor: Option<Pixels>,
    cursor_pos: Option<Pixel>,
    brush: Brush,
    pub mode: PencilMode,
    pub brush_type: BrushType,
    buffer: Pixels,
}

impl Pencil {
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
            mode: PencilMode::PixelPerfect,
            buffer,
        }
    }

    pub fn draw_polyline(&mut self, xpr: &mut Xprite, polyline: &Polyline) -> Pixels {
        let path = polyline.interp();
        let mut rasterized = path.rasterize(xpr).unwrap();
        rasterized.set_color(&xpr.color());
        // self.buffer.extend(&pixels);
        rasterized
    }

    fn set_cursor(&self, xpr: &mut Xprite) -> Option<()> {
        if self.cursor.is_none() { return None; }
        let cursor = self.cursor.clone().unwrap();
        xpr.set_cursor(&cursor);
        Some(())
    }

    /// convert brush shape to actual pixel on canvas
    pub fn brush2pixs(&self, xpr: &Xprite, cursor: Point2D, color: Color) -> Option<Pixels> {
        let Point2D {x, y} = xpr.canvas.shrink_size(&cursor);

        let (brush_w, brush_h) = self.brush.size;

        if (x + brush_w) >= xpr.canvas.art_w || (y + brush_h) >= xpr.canvas.art_h {
            None
        } else {
            let (offset_x, offset_y) = self.brush.offset;
            let ret: Vec<Pixel> = self.brush.shape.iter().map(
                |Pixel {point,..}| Pixel {
                    point: Point2D::new(point.x+x + offset_x, point.y+y + offset_y),
                    color: color,
                }
            ).collect();
            Some(Pixels::from_slice(&ret))
        }
    }

}

impl Tool for Pencil {

    fn tool_type(&self) -> ToolType {
        ToolType::Pencil
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Point2D) -> Option<()> {
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

        let button = self.is_mouse_down.clone().unwrap();
        if button == InputItem::Left {
                self.buffer.clear();
                let line_pixs = self.current_polyline.connect_with_line(&xpr)?;
                let mut pixs = if self.mode != PencilMode::Raw {
                    let perfect = pixel_perfect(&line_pixs);
                    Pixels::from_slice(&perfect)
                } else {
                    Pixels::from_slice(&line_pixs)
                };
                pixs.with_color(&xpr.color());
                self.buffer.extend(&pixs);
        } else if button == InputItem::Right {
            // xpr.remove_pixels(&pixels.unwrap());
        }
        self.draw(xpr)
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, p: Point2D, button: InputItem) -> Option<()>{
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

    fn mouse_up(&mut self, xpr: &mut Xprite, _p: Point2D) -> Option<()> {
        if self.is_mouse_down.is_none() {return Some(()); }
        let button = self.is_mouse_down.clone().unwrap();
        if button == InputItem::Right { return Some(()); }

        use self::PencilMode::*;
        match self.mode {
            Raw => {
                // no processing
            }
            PixelPerfect => {
                // if there is only one pixel in the buffer
                if self.buffer.0.len() == 1 {
                    // noop
                } else {
                    self.buffer.clear();
                    let points = self.current_polyline.connect_with_line(xpr)?;
                    let perfect = &pixel_perfect(&points);
                    let mut pixs = Pixels::from_slice(&perfect);
                    pixs.set_color(&xpr.color());
                    self.buffer.extend(&pixs);
                }
            }
            SortedMonotonic => {
                self.buffer.clear();
                let points = self.current_polyline.connect_with_line(xpr)?;
                let mut perfect = pixel_perfect(&points);
                let sorted = sort_path(&mut perfect)?;
                let mut pixs = Pixels::from_slice(&sorted);
                pixs.set_color(&xpr.color());
                self.buffer.extend(&pixs);
            }
        }

        xpr.history.enter()?;
        xpr.history.top()
            .selected_layer
            .borrow_mut()
            .content
            .extend(&self.buffer);

        self.current_polyline.clear();
        self.buffer.clear();
        self.is_mouse_down = None;

        self.draw(xpr);
        Some(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Option<()> {
        xpr.new_frame();
        self.set_cursor(xpr);
        xpr.add_pixels(&self.buffer);

        Some(())
    }

    fn set(&mut self, _xpr: &mut Xprite, option: &str, value: &str) -> Option<()> {
        match option {
            "mode" => {
                use self::PencilMode::*;
                match PencilMode::from_str(value) {
                    Raw             => self.mode = Raw,
                    SortedMonotonic => self.mode = SortedMonotonic,
                    PixelPerfect    => self.mode = PixelPerfect,
                };
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
