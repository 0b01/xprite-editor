use crate::prelude::*;
use crate::algorithms::pixel_perfect::pixel_perfect;
use std::str::FromStr;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PencilMode {
    /// raw - noop
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

    pub const VARIANTS: [PencilMode; 3] = [
        PencilMode::Raw,
        PencilMode::PixelPerfect,
        PencilMode::SortedMonotonic,
    ];
}

impl FromStr for PencilMode {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, ()> {
        match string {
            "Raw" => Ok(PencilMode::Raw),
            "Pixel Perfect" => Ok(PencilMode::PixelPerfect),
            "Sorted Monotonic" => Ok(PencilMode::SortedMonotonic),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
pub struct Pencil {
    is_mouse_down: Option<InputItem>,
    current_polyline: Polyline,
    cursor: Option<Pixels>,
    cursor_pos: Option<Pixel>,
    brush: Brush,
    pub mode: PencilMode,
    pub brush_type: BrushType,
    moved: bool,
    draw_buffer: Pixels,
    update_buffer: Option<Pixels>,
}

impl Default for Pencil {
    fn default() -> Self {
        Self::new()
    }
}


impl Pencil {
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
            mode: PencilMode::PixelPerfect,
            moved: false,
            draw_buffer: Pixels::new(),
            update_buffer: None,
        }
    }

    pub fn draw_stroke(&self, xpr: &Xprite) -> Result<Pixels, String> {
        let mut line_pixs = self.current_polyline.connect_with_line(&xpr)?;
        let pixs = if self.mode == PencilMode::Raw {
            line_pixs
        } else {
            line_pixs.pixel_perfect();
            line_pixs
        };
        let mut pixs = self.brush.follow_stroke(&pixs).unwrap();
        pixs.set_color(&xpr.color());
        Ok(pixs)
    }

    pub fn finalize(&mut self, xpr: &Xprite) -> Result<(), String> {
        use self::PencilMode::*;
        let mut buf = match self.mode {
            Raw => {
                self.draw_stroke(xpr)?
            }
            PixelPerfect => {
                // if mousedown w/o move
                if !self.moved {
                    self.cursor.clone().unwrap()
                } else {
                    let mut points = self.current_polyline.connect_with_line(xpr)?;
                    points.pixel_perfect();
                    let path = self.brush.follow_stroke(&points).unwrap();
                    path
                }
            }
            SortedMonotonic => {
                let mut points = self.current_polyline.connect_with_line(xpr)?;
                points.pixel_perfect();
                points.monotonic_sort();
                let path = self.brush.follow_stroke(&points).unwrap();
                path
            }
        };

        buf.set_color(&xpr.color());
        self.update_buffer = Some(buf);
        Ok(())
    }

}

impl Tool for Pencil {

    fn tool_type(&self) -> ToolType {
        ToolType::Pencil
    }

    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2D) -> Result<(), String> {
        let pixels = self.brush.to_canvas_pixels(xpr.canvas.shrink_size(p), xpr.color());
        self.cursor = pixels.clone();
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel{point, color});

        // if mouse is done
        if self.is_mouse_down.is_none() || pixels.is_none() {
            return Ok(())
        }
        self.moved = true;
        self.current_polyline.push(p);

        let stroke = self.draw_stroke(xpr)?;
        self.draw_buffer.extend(&stroke);

        Ok(())
    }

    fn mouse_down(&mut self, xpr: &Xprite, p: Vec2D, button: InputItem) -> Result<(), String>{
        self.is_mouse_down = Some(button);

        self.current_polyline.push(p);
        let pixels = self.brush.to_canvas_pixels(xpr.canvas.shrink_size(p), xpr.color());
        // TODO:
        if let Some(pixels) = pixels {
            if button == InputItem::Left {
                self.draw_buffer.extend(&pixels);
            } else {
                // xpr.remove_pixels(&pixels);
            }
        }
        Ok(())
    }

    fn mouse_up(&mut self, xpr: &Xprite, _p: Vec2D) -> Result<(), String> {
        if self.is_mouse_down.is_none() {return Ok(()); }
        let button = self.is_mouse_down.unwrap();
        if button == InputItem::Right { return Ok(()); }

        self.finalize(xpr)?;

        self.current_polyline.clear();
        self.is_mouse_down = None;
        self.draw_buffer.clear();
        self.moved = false;

        Ok(())
    }

    fn cursor(&self) -> Option<Pixels> {
        self.cursor.clone()
    }

    fn update(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        if let Some(pixs) = &self.update_buffer {
            xpr.history.enter()?;
            xpr.current_layer_mut()
                .ok_or_else(||"Layer doesn't exist.".to_owned())?
                .content
                .extend(pixs);
        }
        self.update_buffer = None;
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        xpr.new_frame();
        self.set_cursor(xpr);
        xpr.add_pixels(&self.draw_buffer.with_color(&xpr.color()));
        Ok(())
    }

    fn set(&mut self, _xpr: &Xprite, option: &str, value: &str) -> Result<(), String> {
        match option {
            "mode" => {
                use self::PencilMode::*;
                match PencilMode::from_str(value) {
                    Ok(Raw)             => self.mode = Raw,
                    Ok(SortedMonotonic) => self.mode = SortedMonotonic,
                    Ok(PixelPerfect)    => self.mode = PixelPerfect,
                    _ => (),
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
        Ok(())
    }
}
