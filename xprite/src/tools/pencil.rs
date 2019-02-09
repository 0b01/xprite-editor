use crate::algorithms::line::continuous_line;
use crate::prelude::*;

use std::str::FromStr;

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum PencilMode {
    /// raw - noop
    Raw,
    /// pixel perfect - nothing else
    PixelPerfect,
    /// pixel perfect - nothing else
    PixelAntiPerfect,
    /// sort each monotonic segment
    SortedMonotonic,
}

impl PencilMode {
    pub fn as_str(&self) -> &str {
        match self {
            PencilMode::Raw => "Raw",
            PencilMode::PixelPerfect => "Pixel Perfect",
            PencilMode::PixelAntiPerfect => "Pixel Anti-Perfect",
            PencilMode::SortedMonotonic => "Sorted Monotonic",
        }
    }

    pub const VARIANTS: [PencilMode; 4] = [
        PencilMode::Raw,
        PencilMode::PixelPerfect,
        PencilMode::PixelAntiPerfect,
        PencilMode::SortedMonotonic,
    ];
}

impl FromStr for PencilMode {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, ()> {
        match string {
            "Raw" => Ok(PencilMode::Raw),
            "Pixel Anti-Perfect" => Ok(PencilMode::PixelAntiPerfect),
            "Pixel Perfect" => Ok(PencilMode::PixelPerfect),
            "Sorted Monotonic" => Ok(PencilMode::SortedMonotonic),
            _ => panic!("impossible"),
        }
    }
}

#[derive(Debug)]
pub struct Pencil {
    is_mouse_down: Option<InputItem>,
    current_polyline: Polyline,
    cursor: Option<Pixels>,
    cursor_pos: Option<Vec2f>,
    pub mode: PencilMode,

    last_mouse_down_or_up: Option<Vec2f>,
    shift: bool,

    brush: Brush,
    pub brush_type: BrushType,

    moved: bool,
    draw_buffer: Pixels,
    update_buffer: Option<Pixels>,
    redraw: bool,
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
        let brush_type = BrushType::Pixel;
        let brush = Brush::pixel();
        let current_polyline = Polyline::new();
        let redraw = true;

        Self {
            is_mouse_down,
            cursor_pos: None,
            last_mouse_down_or_up: None,
            shift: false,
            current_polyline,
            cursor,
            brush,
            brush_type,
            mode: PencilMode::PixelPerfect,
            moved: false,
            draw_buffer: Pixels::new(),
            update_buffer: None,
            redraw,
        }
    }

    pub fn draw_stroke(&self, xpr: &Xprite) -> Result<Pixels, String> {
        let mut line_pixs = self
            .current_polyline
            .to_pixel_coords(xpr)?
            .connect_with_line()?;
        let pixs = if self.mode == PencilMode::Raw {
            line_pixs
        } else {
            line_pixs.pixel_perfect();
            line_pixs
        };
        let mut pixs = self.brush.follow_stroke(&pixs).unwrap();
        pixs.set_color(xpr.color());
        Ok(pixs)
    }

    fn finalize_continuous_line(
        &mut self,
        xpr: &Xprite,
        start: Option<Vec2f>,
        stop: Option<Vec2f>,
    ) -> Result<(), String> {
        if let (Some(start), Some(stop)) = (start, stop) {
            let buf = continuous_line(start, stop);
            let mut buf = self.brush.follow_stroke(&buf).unwrap();
            buf.set_color(xpr.color());
            self.update_buffer = Some(buf);
        }
        Ok(())
    }

    fn finalize(&mut self, xpr: &Xprite) -> Result<(), String> {
        use self::PencilMode::*;
        let mut buf = match self.mode {
            Raw => self.draw_stroke(xpr)?,
            PixelPerfect | PixelAntiPerfect => {
                // if mousedown w/o move
                if !self.moved {
                    self.cursor.clone().unwrap()
                } else {
                    let mut points = self
                        .current_polyline
                        .to_pixel_coords(xpr)?
                        .connect_with_line()?;
                        if self.mode == PencilMode::PixelPerfect
                            {points.pixel_perfect();}
                        else
                            {points.pixel_antiperfect();}

                    let path = self.brush.follow_stroke(&points).unwrap();
                    path
                }
            }
            SortedMonotonic => {
                let mut points = self
                    .current_polyline
                    .to_pixel_coords(xpr)?
                    .connect_with_line()?;
                points.pixel_perfect();
                if points.len() > 1 { points.monotonic_sort(); }
                let path = self.brush.follow_stroke(&points).unwrap();
                path
            }
        };

        buf.set_color(xpr.color());
        self.update_buffer = Some(buf);
        Ok(())
    }

    fn draw_line(&self) -> Option<Pixels> {
        let buf = continuous_line(self.last_mouse_down_or_up?, self.cursor_pos?);
        let buf = self.brush.follow_stroke(&buf)?;
        Some(buf)
    }
}

impl Tool for Pencil {
    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        let pixels = self.brush.to_canvas_pixels(point, xpr.color());
        self.cursor = pixels.clone();
        self.cursor_pos = Some(point);

        if self.shift {
            if let Some(pixs) = self.draw_line() {
                self.draw_buffer = pixs;
                self.redraw = true;
                return Ok(());
            }
        }

        // if mouse is down
        if self.is_mouse_down.is_none() || pixels.is_none() {
            return Ok(());
        }
        self.moved = true;
        self.current_polyline.push(p);

        let stroke = self.draw_stroke(xpr)?;
        self.draw_buffer = stroke;
        self.redraw = true;

        Ok(())
    }

    fn mouse_down(&mut self, xpr: &Xprite, p: Vec2f, button: InputItem) -> Result<(), String> {
        self.is_mouse_down = Some(button);

        self.current_polyline.push(p);
        let pixels = self
            .brush
            .to_canvas_pixels(xpr.canvas.shrink_size(p), xpr.color());
        // TODO:
        if let Some(pixels) = pixels {
            if button == InputItem::Left {
                self.draw_buffer.extend(&pixels);
                self.redraw = true
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
        self.is_mouse_down = None;
        self.draw_buffer.clear();
        self.redraw = false;
        self.moved = false;

        Ok(())
    }

    fn cursor(&self) -> Option<Pixels> {
        self.cursor.clone()
    }

    fn update(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        if let Some(pixs) = &self.update_buffer {
            xpr.history.enter()?;
            xpr.current_layer_mut()
                .ok_or_else(|| "Layer doesn't exist.".to_owned())?
                .content
                .extend(pixs);
            self.update_buffer = None;
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
        if self.redraw {
            xpr.add_pixels(&self.draw_buffer.with_color(xpr.color()));
            self.redraw = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn set(&mut self, _xpr: &Xprite, option: &str, value: &str) -> Result<(), String> {
        match option {
            "mode" => {
                match PencilMode::from_str(value) {
                    Ok(mode) => self.mode = mode,
                    _ => (),
                };
            }
            "brush" => match value {
                "+" => {
                    self.brush = Brush::cross();
                    self.brush_type = BrushType::Cross;
                }
                "." => {
                    self.brush = Brush::pixel();
                    self.brush_type = BrushType::Pixel;
                }
                _ => {
                    if value.starts_with("o") {
                        let size = value[1..].parse::<i32>().unwrap();
                        self.brush = Brush::circle(size);
                        self.brush_type = BrushType::Circle;
                    } else if value.starts_with("s") {
                        let size = value[1..].parse::<i32>().unwrap();
                        self.brush = Brush::square(size);
                        self.brush_type = BrushType::Square;
                    } else if value.starts_with("/") {
                        let params: Vec<f64> = value[1..].split(",").map(|i|i.parse().unwrap()).collect();
                        self.brush = Brush::line(params[0] as i32, params[1]);
                        self.brush_type = BrushType::Line;
                    }
                }
            },
            "shift" => match value {
                "true" => {
                    self.shift = true;
                    if let Some(pixs) = self.draw_line() {
                        self.draw_buffer = pixs;
                    }
                    self.redraw = true;
                }
                "false" => {
                    self.shift = false;
                    self.draw_buffer.clear();
                    self.redraw = true;
                }
                _ => error!("malformed value: {}", value),
            },
            _ => (),
        }
        Ok(())
    }
}
