//! Pencil module
//!
//! The pencil is a pipelined tool:
//!
//! 1. Raw input
//! 2. Pixel perfect
//! 3. Pixel AntiPerfect
//! 4. Sorted Monotonic
//! 5. Selective Anti-Alias
//!
//! Internally, it is implemented this way:
//!
//! 1. Raw input
//! 2. pixel_perfect? or pixel_antiperfect?
//! 3. sorted_monotonic?
//! 4. anti_alias?
//! 5. follow_stroke

use crate::algorithms::line::continuous_line;
use crate::prelude::*;

#[derive(PartialEq, Clone, Debug)]
pub struct PencilProcessor {
    /// pixel perfect - nothing else
    ///
    /// None -> None
    /// true -> pixel perfect
    /// false -> pixel anti-perfect
    pub run_pixel_perfect: Option<bool>,
    /// sort each monotonic segment
    pub sorted_monotonic: bool,
    /// Anti-aliasing with background color for each segment
    pub selective_anti_aliasing: bool,
    /// aa color
    pub aa_alt_color: Option<Color>,
    /// aa threshold
    pub aa_threshold: f32,
    /// aa min_segment_length
    pub min_segment_length: i32,
    pub polyline: Polyline,
}

impl PencilProcessor {
    pub fn new() -> Self {
        Self {
            run_pixel_perfect: Some(true),
            sorted_monotonic: false,
            selective_anti_aliasing: false,
            aa_alt_color: None,
            aa_threshold: 0.5,
            min_segment_length: 2,
            polyline: Polyline::new(),
        }
    }

    pub fn clear(&mut self) {
        self.polyline.clear();
    }

    pub fn finalize(&self, brush: &Brush, color: Color) -> Result<Pixels, String> {
        let mut points = self.polyline.connect_with_line(color)?;
        // TODO: check self.moved
        match self.run_pixel_perfect {
            None => (),
            Some(true) => {
                points.pixel_perfect();
            }
            Some(false) => {
                points.pixel_antiperfect();
            }
        };
        if self.sorted_monotonic {
            assert_eq!(self.run_pixel_perfect, Some(true));
            if points.len() > 1 {
                // TODO: move len check inside function
                points.monotonic_sort();
            }
        }
        if self.selective_anti_aliasing {
            assert_eq!(self.run_pixel_perfect, Some(true));
            points.selective_antialias(
                self.aa_threshold as f64,
                self.aa_alt_color.unwrap_or(Color::Indexed(0)),
                self.min_segment_length as usize,
            );
        }
        let path = brush.follow_stroke(&points).unwrap();
        Ok(path)
    }

    pub fn draw(&self, brush: &Brush, color: Color) -> Result<Pixels, String> {
        let mut points = self.polyline.connect_with_line(color)?;
        match self.run_pixel_perfect {
            None => (),
            Some(true) => points.pixel_perfect(),
            Some(false) => points.pixel_antiperfect(),
        };
        let path = brush.follow_stroke(&points).unwrap();
        Ok(path)
    }

    pub fn push(&mut self, point: Vec2f) {
        self.polyline.push(point)
    }
}

#[derive(Debug)]
pub struct Pencil {
    is_mouse_down: Option<InputItem>,
    cursor: Option<Pixels>,
    cursor_pos: Option<Vec2f>,
    pub processor: PencilProcessor,
    last_mouse_down_or_up: Option<Vec2f>,
    shift: bool,
    pub brush: Brush,
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
        let brush = Brush::circle(1, Color::orange());
        let redraw = true;
        let processor = PencilProcessor::new();

        Self {
            is_mouse_down,
            cursor_pos: None,
            processor,
            last_mouse_down_or_up: None,
            shift: false,
            cursor,
            brush,
            moved: false,
            draw_buffer: Pixels::new(),
            update_buffer: None,
            redraw,
        }
    }

    fn finalize_continuous_line(&mut self, xpr: &Xprite, start: Option<Vec2f>, stop: Option<Vec2f>) -> Result<(), String> {
        if let (Some(start), Some(stop)) = (start, stop) {
            let buf = continuous_line(start, stop, xpr.color());
            let buf = self.brush.follow_stroke(&buf).unwrap();
            self.update_buffer = Some(buf);
        }
        Ok(())
    }

    fn finalize(&mut self, xpr: &Xprite) -> Result<(), String> {
        let buf = self.processor.finalize(&self.brush, xpr.color())?;
        self.update_buffer = Some(buf);
        Ok(())
    }

    fn draw_line(&self, color: Color) -> Option<Pixels> {
        let buf = continuous_line(self.last_mouse_down_or_up?, self.cursor_pos?, color);
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
            if let Some(pixs) = self.draw_line(xpr.color()) {
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
        self.processor.polyline.push(point);

        let stroke = self.processor.draw(&self.brush, xpr.color())?;
        self.draw_buffer = stroke;
        self.redraw = true;

        Ok(())
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Vec2f) -> Result<(), String> {
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

        self.processor.clear();
        self.is_mouse_down = None;
        self.draw_buffer.clear();
        self.redraw = false;
        self.moved = false;

        Ok(())
    }

    fn mouse_down(&mut self, xpr: &Xprite, p: Vec2f, button: InputItem) -> Result<(), String> {
        let point = xpr.canvas.shrink_size(p);
        self.is_mouse_down = Some(button);

        self.processor.polyline.push(point);
        let pixels = self.brush.to_canvas_pixels(xpr.canvas.shrink_size(p), xpr.color());
        // TODO:
        if let Some(pixels) = pixels {
            if button == InputItem::Left {
                self.draw_buffer.extend(&pixels);
                self.redraw = true;
            } else {
                // xpr.remove_pixels(&pixels);
            }
        }
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        xpr.new_frame();
        if let Some(cursor) = &self.cursor {
            xpr.set_cursor(cursor);
        }
        if self.redraw {
            xpr.add_pixels(&self.draw_buffer);
            self.redraw = false;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn update(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        if let Some(pixs) = &self.update_buffer {
            xpr.finalize_pixels(&pixs)?;
            self.update_buffer = None;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    // TODO: dedupe brush instantiation code(pencil, eraser)
    fn set(&mut self, xpr: &Xprite, option: &str, value: &str) -> Result<(), String> {
        match option {
            "brush" => {
                self.brush = value.parse()?;
            }
            "LShift" | "RShift" => match value {
                "true" => {
                    self.shift = true;
                    if let Some(pixs) = self.draw_line(xpr.color()) {
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
