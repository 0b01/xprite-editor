use crate::prelude::*;
use crate::algorithms::sorter::sort_path;
use crate::algorithms::pixel_perfect::pixel_perfect;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum PencilMode {
    /// just run pixel perfect - nothing else
    PixelPerfect,
    /// convert to vector and sort everything by slope
    SimplifyAndSortWhole,
    /// convert to vector and sort each segment
    SimplifyAndSortByParts,
    /// sort each monotonic segment
    SortedMonotonic,
}

pub struct Pencil {
    is_mouse_down: Option<InputItem>,
    current_polyline: Polyline,
    cursor: Option<Pixels>,
    cursor_pos: Option<Pixel>,
    brush: Brush,
    tolerence: f32,
    mode: PencilMode,
}
impl Pencil {
    pub fn new() -> Self {
        let is_mouse_down = None;
        let cursor = None;
        let cursor_pos = None;
        let brush = Brush::pixel();
        let current_polyline = Polyline::new();

        Self {
            is_mouse_down,
            current_polyline,
            cursor,
            cursor_pos,
            brush,
            tolerence: 2.,
            mode: PencilMode::PixelPerfect,
        }
    }

    pub fn draw_polyline(&mut self, xpr: &mut Xprite, polyline: &Polyline, sort_parts: bool, sort_whole: bool) {

        let path = polyline.interp();
        let mut rasterized = path.rasterize(xpr, sort_parts, sort_whole).unwrap();
        let pixels = rasterized.with_color(&Color::grey());
        xpr.add_pixels(&pixels);

        // // plot anchors
        // for &p in polyline.pos.iter() {
        //     let Point2D{x, y} = xpr.canvas.shrink_size(p.as_i32());
        //     let color = ColorOption::Set(Color::blue());
        //     xpr.draw_pixel(x, y, color);
        // }

        // // plot control points
        // for seg in &path.segments {
        //     let CubicBezierSegment { ctrl1, ctrl2, .. } = seg;
        //     for point in vec![ctrl1, ctrl2] {
        //         let Point2D{x, y} = xpr.canvas.shrink_size(point.as_i32());
        //         xpr.draw_pixel(x, y, ColorOption::Set(Color::red()));
        //     }
        // }

    }

    fn draw_cursor(&self, xpr: &mut Xprite) -> Option<()> {
        if self.cursor.is_none() { return None; }
        let cursor = self.cursor.clone().unwrap();
        xpr.add_pixels(&cursor);
        Some(())
    }

}

impl Tool for Pencil {

    fn get_name(&self) -> &'static str {
        "pencil"
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Point2D<f32>) -> Option<()> {
        let pixels = xpr.canvas.to_pixels(p, &self.brush, xpr.color());
        self.cursor = pixels.clone();
        let point = xpr.canvas.shrink_size(&p);
        let color = ColorOption::Set(xpr.color());
        self.cursor_pos = Some(Pixel{point, color});

        // if mouse is done
        if self.is_mouse_down.is_none() || pixels.is_none() {
            return self.draw(xpr);
        }

        self.current_polyline.push(p);

        let button = self.is_mouse_down.clone().unwrap();
        if button == InputItem::Left {
            xpr.history.undo();
            xpr.history.enter();
            let line_pixs = self.current_polyline.connect_with_line(&xpr)?;
            let perfect = pixel_perfect(&line_pixs);
            xpr.add_pixels(&Pixels::from_slice(&perfect));
        } else if button == InputItem::Right {
            // xpr.remove_pixels(&pixels.unwrap());
        }
        self.draw(xpr)
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, p: Point2D<f32>, button: InputItem) -> Option<()>{
        self.is_mouse_down = Some(button);
        xpr.history.enter();

        self.current_polyline.push(p);

        let pixels = xpr.canvas.to_pixels(p, &self.brush, xpr.color());
        if let Some(pixels) = pixels {
            if button == InputItem::Left {
                xpr.add_pixels(&pixels);
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

        xpr.history.undo();
        xpr.history.enter();
        use self::PencilMode::*;
        match self.mode {
            SimplifyAndSortByParts => {
                // simply curve then rasterize
                let simple = self.current_polyline.reumann_witkam(self.tolerence)?;
                self.draw_polyline(xpr, &simple, true, false);
            }
            SimplifyAndSortWhole => {
                // simply curve then rasterize
                let simple = self.current_polyline.reumann_witkam(self.tolerence)?;
                self.draw_polyline(xpr, &simple, false, true);
            }
            PixelPerfect => {
                let points = self.current_polyline.connect_with_line(xpr)?;
                let perfect = &pixel_perfect(&points);
                xpr.add_pixels(&Pixels::from_slice(&perfect).with_color(&Color::grey()));
            }
            SortedMonotonic => {
                let points = self.current_polyline.connect_with_line(xpr)?;
                let mut perfect = pixel_perfect(&points);
                let sorted = sort_path(&mut perfect)?;
                xpr.add_pixels(&Pixels::from_slice(&sorted).with_color(&Color::grey()));
            }
        }

        self.current_polyline.clear();
        self.is_mouse_down = None;

        self.draw(xpr);
        Some(())
    }

    fn draw(&self, xpr: &mut Xprite) -> Option<()> {
        // xpr.canvas.clear_all();
        self.draw_cursor(xpr)
    }

    fn set(&mut self, _xpr: &mut Xprite, option: &str, value: &str) -> Option<()> {
        match option {
            "mode" => {
                use self::PencilMode::*;
                match value {
                    "monotonic" => self.mode = SortedMonotonic,
                    "pp"        => self.mode = PixelPerfect,
                    "whole"     => self.mode = SimplifyAndSortWhole,
                    "parts"     => self.mode = SimplifyAndSortByParts,
                    _ => panic!("malformed value: {}", value),
                };
            }
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
