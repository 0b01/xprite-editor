use crate::prelude::*;
use std::str::FromStr;

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

    pub const VARIANTS: [VectorMode; 2] = [
        VectorMode::Continuous,
        VectorMode::Curvature,
    ];
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
    cursor_pos: Option<Pixel>,
    brush: Brush,
    current_polyline: Option<Polyline>,
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
        let cursor_pos = None;
        let brush = Brush::pixel();
        let current_polyline = Some(Polyline::new());

        Self {
            is_mouse_down,
            current_polyline,
            cursor_pos,
            brush,
            tolerence: 1.,
            draw_bezier: true,
            mono_sort: true,
            ..Default::default()
        }
    }

    fn draw_continuous(&self) -> Result<(Path, Pixels), String> {
        let simple = self.current_polyline
            .as_ref().ok_or_else(||"cannot borrow as mut".to_owned())?.reumann_witkam(self.tolerence)?;
        let path = simple.interp();
        let mut buf = path.rasterize(self.mono_sort).unwrap();
        buf.set_color(&Color::orange());

        Ok((path, buf))
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

    fn mouse_move(&mut self, xpr: &Xprite, p: Vec2f) -> Result<(), String> {
        // update cursor pos
        let pixels = self
            .brush
            .to_canvas_pixels(xpr.canvas.shrink_size(p), xpr.color());
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel { point, color });

        if self.is_mouse_down.is_none() || pixels.is_none() {
            return Ok(());
        }

        // the rest handles when left button is pressed
        let p = xpr.canvas.shrink_size_no_floor(p);
        self.current_polyline
            .as_mut()
            .ok_or_else(|| "cannot borrow as mut")?
            .push(p);

        Ok(())
    }

    fn mouse_down(&mut self, xpr: &Xprite, p: Vec2f, button: InputItem) -> Result<(), String> {
        self.is_mouse_down = Some(button);

        let p = xpr.canvas.shrink_size_no_floor(p);
        self.current_polyline
            .as_mut()
            .ok_or_else(|| "cannot borrow as mut".to_owned())?
            .push(p);
        Ok(())
    }

    fn mouse_up(&mut self, _xpr: &Xprite, _p: Vec2f) -> Result<(), String> {
        if self.is_mouse_down.is_none() {
            return Ok(());
        }
        let button = self.is_mouse_down.unwrap();
        if button == InputItem::Right {
            return Ok(());
        }

        self.is_mouse_down = None;
        Ok(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Result<(), String> {
        xpr.new_frame();
        self.set_cursor(xpr);

        let (path, buf) = self.draw_continuous()?;

        if self.draw_bezier {
            xpr.bz_buf.extend(path.segments);
        }
        xpr.add_pixels(&buf);

        Ok(())
    }

    fn update(&mut self, xpr: &mut Xprite) -> Result<(), String> {
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
            "mode" => {
                use self::VectorMode::*;
                match VectorMode::from_str(value) {
                    Ok(Continuous) => self.mode = Continuous,
                    Ok(Curvature) => self.mode = Curvature,
                    _ => (),
                };
            }
            // "brush" => match value {
            //     "cross" => self.brush = Brush::cross(),
            //     "pixel" => self.brush = Brush::pixel(),
            //     _ => error!("malformed value: {}", value),
            // },
            _ => (),
        }
        Ok(())
    }
}
