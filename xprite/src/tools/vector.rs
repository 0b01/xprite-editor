//! This module contains the implementation for the vector tool
use crate::prelude::*;
use std::str::FromStr;

#[derive(Debug, Copy, Clone)]
enum AnchorType {
    From,
    To,
    Ctrl1,
    Ctrl2,
}

#[derive(Eq, PartialEq, Clone, Copy, Debug)]
pub enum VectorMode {
    Continuous,
    Curvature,
}

impl VectorMode {
    pub fn as_str(&self) -> &str {
        match self {
            VectorMode::Continuous => "Continuous",
            VectorMode::Curvature => "Curvature",
        }
    }

    pub const VARIANTS: [VectorMode; 2] = [VectorMode::Continuous, VectorMode::Curvature];
}

impl FromStr for VectorMode {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, ()> {
        match string {
            "Continuous" => Ok(VectorMode::Continuous),
            "Curvature" => Ok(VectorMode::Curvature),
            _ => Err(()),
        }
    }
}

impl Default for VectorMode {
    fn default() -> Self {
        VectorMode::Continuous
    }
}

#[derive(Debug, Default)]
pub struct Vector {
    is_mouse_down: Option<InputItem>,
    update_buffer: Option<Pixels>,

    dragging_anchor: Option<(usize, AnchorType)>,

    cursor_pos: Option<Vec2f>,

    start_pos: Option<Vec2f>,
    end_pos: Option<Vec2f>,
    ctrl1_pos: Option<Vec2f>,
    ctrl2_pos: Option<Vec2f>,
    curves: Vec<CubicBezierSegment>,

    current_polyline: Polyline,
    recording: bool,

    pub brush: Brush,
    pub brush_type: BrushType,
    /// edit mode
    pub mode: VectorMode,
    /// polyline simplification tolerance threshold
    pub tolerence: f32,
    /// whether to draw bezier curve
    pub draw_bezier: bool,
    /// sort by segment
    pub mono_sort: bool,
}

impl Vector {
    pub fn new() -> Self {
        let is_mouse_down = None;
        let current_polyline = Polyline::new();
        Self {
            is_mouse_down,
            current_polyline,
            tolerence: 15.,
            draw_bezier: true,
            mono_sort: true,
            brush: Brush::circle(1, Color::orange()),
            recording: true,
            ..Default::default()
        }
    }

    fn draw_continuous(&self, color: Color) -> Result<(Path, Pixels), String> {
        let simplified = self.current_polyline.reumann_witkam(self.tolerence as f64);
        if let Ok(simplified) = simplified {
            let path = simplified.interp();
            let buf = path.rasterize(self.mono_sort, color).unwrap();
            Ok((path, buf))
        } else {
            Ok((Path::default(), Pixels::new()))
        }
    }

    fn get_draw_curvature(&self) -> Option<CubicBezierSegment> {
        match (self.start_pos, self.end_pos, self.ctrl1_pos, self.ctrl2_pos) {
            (None, ..) => None,
            (Some(from), None, ..) => Some({
                let to = self.cursor_pos?;
                CubicBezierSegment {
                    from,
                    to,
                    ctrl1: from,
                    ctrl2: to,
                }
            }),
            (Some(from), Some(to), None, ..) => Some({
                let ctrl1 = self.cursor_pos?;
                CubicBezierSegment { from, to, ctrl1, ctrl2: to }
            }),
            (Some(from), Some(to), Some(ctrl1), None) => Some({
                let ctrl2 = self.cursor_pos?;
                CubicBezierSegment { from, to, ctrl1, ctrl2 }
            }),
            (Some(_), Some(_), Some(_), Some(_)) => self.finalize_curvature(),
        }
    }

    fn finalize_curvature(&self) -> Option<CubicBezierSegment> {
        let from = self.start_pos?;
        let to = self.end_pos?;
        let ctrl1 = self.ctrl1_pos.unwrap_or(from);
        let ctrl2 = self.ctrl2_pos.unwrap_or(to);
        Some(CubicBezierSegment { from, to, ctrl1, ctrl2 })
    }

    fn reset_curvature(&mut self) -> Result<(), String> {
        self.start_pos = None;
        self.end_pos = None;
        self.ctrl1_pos = None;
        self.ctrl2_pos = None;

        Ok(())
    }

    pub fn add_to_hist(&mut self, xpr: &Xprite) -> Result<(), String> {
        self.update_buffer = Some({
            let mut ret = Pixels::new();
            for curve in &self.curves {
                if let Some(ras) = curve.rasterize(self.mono_sort, xpr.color()) {
                    let pixs = self.brush.follow_stroke(&ras).unwrap();
                    ret.extend(&pixs);
                }
            }
            ret
        });
        self.curves.clear();
        self.reset_curvature()?;
        Ok(())
    }

    fn get_anchor(&self, xpr: &Xprite, p: Vec2f) -> Option<(usize, AnchorType)> {
        for (i, curve) in self.curves.iter().enumerate() {
            let &CubicBezierSegment { from, to, ctrl1, ctrl2 } = curve;
            if xpr.canvas.within_circle(from, p) {
                return Some((i, AnchorType::From));
            } else if xpr.canvas.within_circle(ctrl1, p) {
                return Some((i, AnchorType::Ctrl1));
            } else if xpr.canvas.within_circle(ctrl2, p) {
                return Some((i, AnchorType::Ctrl2));
            } else if xpr.canvas.within_circle(to, p) {
                return Some((i, AnchorType::To));
            }
        }
        None
    }

    fn dragging(&mut self) -> Option<()> {
        let (idx, ty) = self.dragging_anchor?;
        let pos = self.cursor_pos?;
        let curve = &mut self.curves[idx];
        match ty {
            AnchorType::From => {
                let orig = &curve.from;
                curve.ctrl1 -= *orig - pos;
                curve.from = pos;
                Some(())
            }
            AnchorType::To => {
                let orig = &mut curve.to;
                curve.ctrl2 -= *orig - pos;
                (*orig) = pos;
                Some(())
            }
            AnchorType::Ctrl1 => {
                curve.ctrl1 = pos;
                Some(())
            }
            AnchorType::Ctrl2 => {
                curve.ctrl2 = pos;
                Some(())
            }
        }
    }
}

impl Tool for Vector {
    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        // update cursor pos
        let pixels = self.brush.to_canvas_pixels(xpr.canvas.shrink_size(p), xpr.color());
        let point = xpr.canvas.shrink_size(p);
        self.cursor_pos = Some(point);

        if self.dragging().is_some() {
            return Ok(());
        }

        if self.is_mouse_down.is_none() || pixels.is_none() {
            return Ok(());
        }

        // the rest handles when left button is pressed
        let p = xpr.canvas.shrink_size_no_floor(p);
        match self.mode {
            VectorMode::Continuous => {
                if self.recording {
                    self.current_polyline.push(p);
                }
            }
            VectorMode::Curvature => {
                // noop
            }
        };

        Ok(())
    }

    fn mouse_down(&mut self, xpr: &Xprite, point: Vec2f, button: InputItem) -> Result<(), String> {
        self.is_mouse_down = Some(button);
        let p = xpr.canvas.shrink_size_no_floor(point);
        self.cursor_pos = Some(p);

        self.dragging_anchor = self.get_anchor(xpr, point);

        if button == InputItem::Right {
            self.reset_curvature()?;
            self.recording = false;
            return Ok(());
        }

        if self.dragging_anchor.is_some() {
            return Ok(());
        }

        match self.mode {
            VectorMode::Continuous => {
                self.recording = true;
                self.current_polyline.push(p);
            }
            VectorMode::Curvature => {
                if self.start_pos.is_none() {
                    self.start_pos = Some(p);
                } else if self.end_pos.is_none() {
                    self.end_pos = Some(p);
                } else if self.ctrl1_pos.is_none() {
                    self.ctrl1_pos = Some(p);
                } else if self.ctrl2_pos.is_none() {
                    self.ctrl2_pos = Some(p);

                    let curve = self.finalize_curvature();
                    self.curves.push(curve.unwrap());
                    let tmp = self.end_pos;
                    self.reset_curvature()?;
                    self.start_pos = tmp;
                } else {
                    error!("This should never happen");
                }
            }
        };
        Ok(())
    }

    fn mouse_up(&mut self, _xpr: &Xprite, _p: Vec2f) -> Result<(), String> {
        if self.is_mouse_down.is_none() {
            return Ok(());
        }
        self.recording = false;

        let simple = self.current_polyline.reumann_witkam(self.tolerence as f64);
        if let Ok(simple) = simple {
            let path = simple.interp();
            self.curves.extend(path.segments);
        }
        self.current_polyline.clear();

        self.dragging_anchor = None;
        let button = self.is_mouse_down.unwrap();
        if button == InputItem::Right {
            return Ok(());
        }

        self.is_mouse_down = None;
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<bool, String> {
        xpr.new_frame();

        if let Some(p) = self.cursor_pos {
            xpr.set_cursor(&pixels!(Pixel { point: p, color: Color::red() }));
        }

        let mut ret = Pixels::new();
        match self.mode {
            VectorMode::Continuous => {
                if let Ok((path, buf)) = self.draw_continuous(xpr.color()) {
                    if self.draw_bezier {
                        xpr.bz_buf.extend(path.segments);
                    }
                    ret.extend(&buf);
                }
            }
            VectorMode::Curvature => {
                if let Some(c) = self.get_draw_curvature() {
                    if self.draw_bezier {
                        xpr.bz_buf.push(c.clone());
                    }
                    if let Some(ras) = c.rasterize(self.mono_sort, xpr.color()) {
                        ret.extend(&ras);
                    }
                }
            }
        };

        // rasterize curves buffer
        for curve in &self.curves {
            if self.draw_bezier {
                xpr.bz_buf.push(curve.clone());
            }
            if let Some(ras) = curve.rasterize(self.mono_sort, xpr.color()) {
                ret.extend(&ras);
            }
        }

        let pixs = self.brush.follow_stroke(&ret).unwrap();
        xpr.add_pixels(&pixs);

        Ok(true)
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

    fn set(&mut self, xpr: &Xprite, option: &str, value: &str) -> Result<(), String> {
        match option {
            "tolerence" => {
                if let Ok(val) = value.parse() {
                    self.tolerence = val;
                } else {
                    error!("cannot parse val: {}", value);
                }
            }
            "mode" => {
                use self::VectorMode::*;
                match VectorMode::from_str(value) {
                    Ok(Continuous) => self.mode = Continuous,
                    Ok(Curvature) => self.mode = Curvature,
                    _ => (),
                };
            }
            "brush" => {
                self.brush = value.parse()?;
            }
            "Return" | "Enter" => {
                self.add_to_hist(xpr)?;
            }
            _ => (),
            // i => info!("{}", i),
        }
        Ok(())
    }
}
