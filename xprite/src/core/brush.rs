use crate::algorithms::{ellipse, line, rect};
use crate::prelude::*;
use std::f64::consts::PI;
use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum BrushType {
    /// size
    Circle(u32),
    /// size
    Square(u32),
    /// size, angle
    Line(u32, u32),
}

impl BrushType {
    pub fn as_str(&self) -> &str {
        match self {
            BrushType::Circle(_) => "o",
            BrushType::Square(_) => "s",
            BrushType::Line(_, _) => "/",
        }
    }
    pub const VARIANTS: [BrushType; 3] = [BrushType::Circle(8), BrushType::Square(4), BrushType::Line(2, 0)];
}

impl Default for BrushType {
    fn default() -> Self {
        BrushType::Circle(1)
    }
}

impl FromStr for Brush {
    type Err = String;
    fn from_str(value: &str) -> Result<Brush, String> {
        if value.starts_with('o') {
            let size = value[1..].parse::<u32>().unwrap();
            Ok(Brush::circle(size, Color::orange())) // TODO: change color
        } else if value.starts_with('s') {
            let size = value[1..].parse::<u32>().unwrap();
            Ok(Brush::square(size))
        } else if value.starts_with('/') {
            let params: Vec<f64> = value[1..].split(',').map(|i| i.parse().unwrap()).collect();
            let color: Color = Color::orange(); //TODO: change color
            Ok(Brush::line(params[0] as u32, params[1] as u32, color))
        } else {
            Err("unimplemented brush shape".to_owned())
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Brush {
    pub shape: PixelOffsets,
    pub bb: (f64, f64),
    pub offset: (f64, f64),
    pub brush_type: BrushType,
}

impl Default for Brush {
    fn default() -> Self {
        Self::new()
    }
}

impl Brush {
    pub fn new() -> Self {
        Brush::circle(1, Color::orange())
    }

    pub fn pixel() -> Self {
        let mut shape = Pixels::new();
        shape.push(pixel!(0., 0., Color::red()));
        Self {
            shape,
            bb: (1., 1.),
            offset: (0., 0.),
            brush_type: BrushType::Circle(1),
        }
    }

    pub fn cross() -> Self {
        let mut shape = Pixels::new();
        shape.push(pixel!(0., 1., Color::red()));
        shape.push(pixel!(1., 0., Color::red()));
        shape.push(pixel!(1., 1., Color::red()));
        shape.push(pixel!(1., 2., Color::red()));
        shape.push(pixel!(2., 1., Color::red()));
        Self {
            shape,
            bb: (3., 3.),
            offset: (-1., -1.),
            brush_type: BrushType::Circle(3),
        }
    }

    pub fn square(size: u32) -> Self {
        let shape = rect::filled_rect(0, 0, size as i32, size as i32, Color::red()).unwrap();
        let off = f64::from(size / 2);
        Self {
            shape,
            bb: (f64::from(size), f64::from(size)),
            offset: (-off, -off),
            brush_type: BrushType::Square(size),
        }
    }

    pub fn circle(size: u32, color: Color) -> Self {
        if size == 1 {
            return Self::pixel();
        }
        if size == 3 {
            return Self::cross();
        }
        let shape = ellipse::algo_ellipsefill(0, 0, size as i32, size as i32 - 1, color);
        let off = (size / 2) as f64;
        Self {
            shape,
            bb: (size as f64, f64::from(size)),
            offset: (-off, -off),
            brush_type: BrushType::Circle(size),
        }
    }

    pub fn line(sz: u32, angle: u32, color: Color) -> Self {
        let size = f64::from(sz);

        let a = PI * f64::from(angle) / 180.;
        let r = size / 2.;
        let d = size;
        let x1 = (r + r * (a + PI).cos()) as i32;
        let y1 = (r - r * (a + PI).sin()) as i32;
        let x2 = (x1 as f64 + d * (a).cos()) as i32;
        let y2 = (y1 as f64 - d * (a).sin()) as i32;

        let p1 = vec2f!(y1, x1);
        let p2 = vec2f!(y2, x2);
        let shape = line::continuous_line(p1, p2, color);

        let bb = {
            let rect = shape.bounding_rect();
            let w = rect.w();
            let h = rect.h();
            (w, h)
        };
        let offset = {
            let off_x = bb.0 / 2.;
            let off_y = bb.1 / 2.;
            (-off_x.floor(), -off_y.floor())
        };

        let brush_type = BrushType::Line(sz, angle);

        Self { shape, bb, offset, brush_type }
    }

    #[must_use]
    pub fn follow_stroke(&self, stroke: &Pixels) -> Option<Pixels> {
        let mut ret = Pixels::new();
        for Pixel { point, color } in &stroke.0 {
            if let Some(pixs) = self.to_canvas_pixels(*point, *color) {
                ret.extend(&pixs);
            }
        }
        Some(ret)
    }

    /// convert brush shape to actual pixel on canvas
    pub fn to_canvas_pixels(&self, cursor: Vec2f, color: Color) -> Option<Pixels> {
        let Vec2f { x, y } = cursor;
        let (offset_x, offset_y) = self.offset;
        let ret: Vec<Pixel> = self
            .shape
            .iter()
            .map(|Pixel { point, .. }| Pixel {
                point: Vec2f {
                    x: point.x + x + offset_x,
                    y: point.y + y + offset_y,
                },
                color,
            })
            .collect();
        Some(Pixels::from_slice(&ret))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_circle_brush4() {
        use super::*;
        assert_eq!(
            Brush::circle(4, Color::red()),
            Brush {
                shape: pixels!(
                    pixel!(1, 0, Color::red()),
                    pixel!(1, 1, Color::red()),
                    pixel!(1, 2, Color::red()),
                    pixel!(1, 3, Color::red()),
                    pixel!(2, 0, Color::red()),
                    pixel!(2, 1, Color::red()),
                    pixel!(2, 2, Color::red()),
                    pixel!(2, 3, Color::red()),
                    pixel!(0, 1, Color::red()),
                    pixel!(0, 2, Color::red()),
                    pixel!(3, 1, Color::red()),
                    pixel!(3, 2, Color::red())
                ),
                bb: (4.0, 4.0),
                offset: (-2.0, -2.0),
                brush_type: BrushType::Circle(4),
            }
        );
    }

    #[test]
    fn test_square_brush2() {
        use super::*;
        assert_eq!(
            Brush::square(2),
            Brush {
                shape: pixels!(
                    pixel!(0, 0, Color::red()),
                    pixel!(0, 1, Color::red()),
                    pixel!(1, 0, Color::red()),
                    pixel!(1, 1, Color::red())
                ),
                bb: (2.0, 2.0),
                offset: (-1.0, -1.0),
                brush_type: BrushType::Square(2),
            }
        );
    }

    #[test]
    fn test_line_brush() {
        use super::*;
        assert_eq!(
            Brush::line(3, 45, Color::red()),
            Brush {
                shape: pixels! {
                    pixel!(2,0,Color::red()),
                    pixel!(1,1,Color::red()),
                    pixel!(0,2,Color::red())
                },
                bb: (3.0, 3.0),
                offset: (-1.0, -1.0),
                brush_type: BrushType::Line(3, 45),
            }
        );
    }
}
