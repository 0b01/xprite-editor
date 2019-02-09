use crate::prelude::*;
use std::f64::consts::PI;
use crate::algorithms::{ellipse, rect, line};

#[derive(PartialEq, Eq, Clone, Debug, Copy)]
pub enum BrushType {
    Pixel,
    Cross,
    Circle,
    Square,
    Line,
}

impl BrushType {
    pub fn as_str(&self) -> &str {
        match self {
            BrushType::Pixel => ".",
            BrushType::Cross => "+",
            BrushType::Circle => "o",
            BrushType::Square => "s",
            BrushType::Line => "/",
        }
    }
    pub const VARIANTS: [BrushType; 5] = [
        BrushType::Pixel,
        BrushType::Cross,
        BrushType::Circle,
        BrushType::Square,
        BrushType::Line,
    ];
}

impl Default for BrushType {
    fn default() -> Self {
        BrushType::Pixel
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Brush {
    pub shape: PixelOffsets,
    pub bb: (f64, f64),
    pub offset: (f64, f64),
}

impl Default for Brush {
    fn default() -> Self {
        Self::pixel()
    }
}

impl Brush {
    pub fn new() -> Self {
        Brush::pixel()
    }

    pub fn pixel() -> Self {
        let mut shape = Pixels::new();
        shape.push(pixel!(0., 0., Color::red()));
        Self {
            shape,
            bb: (1., 1.),
            offset: (0., 0.),
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
        }
    }

    pub fn square(size: i32) -> Self {
        let shape = rect::filled_rect(0, 0, size, size, Color::red()).unwrap();
        let off = (size / 2) as f64;
        Self {
            shape,
            bb: (size as f64, size as f64),
            offset: (-off, -off),
        }
    }

    pub fn circle(size: i32) -> Self {
        if size == 1 { return Self::pixel() }
        if size == 3 { return Self::cross() }
        let shape = ellipse::algo_ellipsefill(
            0, 0, size, size-1
        );
        let off = (size / 2) as f64;
        Self {
            shape,
            bb: (size as f64, size as f64),
            offset: (-off, -off),
        }
    }

    pub fn line(size: i32, angle: f64) -> Self {
        let size = size as f64;

        let a = PI * angle / 180.;
        let r = size as f64 / 2.;
        let d = size as f64;
        let x1 = (r + r*(a+PI).cos()) as i32;
        let y1 = (r - r*(a+PI).sin()) as i32;
        let x2 = (x1 as f64 + d*(a).cos()) as i32;
        let y2 = (y1 as f64 - d*(a).sin()) as i32;


        let p1 = vec2f_xy!(x1, y1);
        let p2 = vec2f_xy!(x2, y2);
        let shape = line::continuous_line(p1, p2);

        let bb = {
            let rect = shape.bounding_rect();
            let w = rect.w();
            let h = rect.h();
            (w, h)
        };
        let offset = {
            let off_x = bb.0/2.;
            let off_y = bb.1/2.;
            (-off_x.floor(), -off_y.floor())
        };

        Self {
            shape,
            bb,
            offset,
        }


    }

    pub fn follow_stroke(&self, stroke: &Pixels) -> Option<Pixels> {
        let mut ret = Pixels::new();
        for Pixel { point, .. } in &stroke.0 {
            if let Some(pixs) = self.to_canvas_pixels(*point, Color::red()) {
                ret.extend(&pixs);
            }
        }
        return Some(ret);
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
                color: color,
            })
            .collect();
        Some(Pixels::from_slice(&ret))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_circle_brush1() {
        use super::*;
        assert_eq!(Brush::circle(1), Brush::pixel());
        assert_eq!(Brush::circle(2), Brush::square(2));
        assert_eq!(Brush::circle(3), Brush::cross());
    }

    #[test]
    fn test_circle_brush4() {
        use super::*;
        assert_eq!(Brush::circle(4), Brush {
            shape: pixels!(
                pixel!(1,0,Color::red()),
                pixel!(1,1,Color::red()),
                pixel!(1,2,Color::red()),
                pixel!(1,3,Color::red()),
                pixel!(2,0,Color::red()),
                pixel!(2,1,Color::red()),
                pixel!(2,2,Color::red()),
                pixel!(2,3,Color::red()),
                pixel!(0,1,Color::red()),
                pixel!(0,2,Color::red()),
                pixel!(3,1,Color::red()),
                pixel!(3,2,Color::red())
            ),
            bb: (4.0, 4.0),
            offset: (-2.0, -2.0)
        });
    }


    #[test]
    fn test_square_brush2() {
        use super::*;
        assert_eq!(Brush::square(1), Brush::pixel());
        assert_eq!(Brush::square(2), Brush {
            shape: pixels!(
                pixel!(0,0,Color::red()),
                pixel!(0,1,Color::red()),
                pixel!(1,0,Color::red()),
                pixel!(1,1,Color::red())
            ),
            bb: (2.0, 2.0),
            offset: (-1.0, -1.0)
        });
    }

    #[test]
    fn test_line_brush() {
        use super::*;
        assert_eq!(Brush::line(3, 45.), Brush {
            shape: pixels!{
                pixel!(2,0,Color::red()),
                pixel!(1,1,Color::red()),
                pixel!(0,2,Color::red())
            },
            bb: ( 3.0, 3.0 ),
            offset: ( -1.0, -1.0 )
        }

        );
    }
}