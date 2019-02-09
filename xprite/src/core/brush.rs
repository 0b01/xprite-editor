use crate::prelude::*;
use std::f64::consts::PI;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum BrushType {
    Pixel,
    Cross,
    Circle,
    Square,
}

impl BrushType {
    pub fn as_str(&self) -> &str {
        match self {
            BrushType::Pixel => ".",
            BrushType::Cross => "+",
            BrushType::Circle => "o",
            BrushType::Square => "s",
        }
    }
    pub const VARIANTS: [BrushType; 4] = [
        BrushType::Pixel,
        BrushType::Cross,
        BrushType::Circle,
        BrushType::Square,
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

    pub fn square(m_size: i32, m_angle: f64) -> Self {
        if m_size == 1 { return Self::pixel() }

        let mut size = m_size;
        if m_angle != 0. && m_size > 2 {
            size = (((2*m_size*m_size)+2) as f64).sqrt() as i32;
        }


        let a = PI * m_angle / 180.;
        let c = size/2;
        let r = m_size as f64/2.;
        let d = m_size as f64;
        let x1 = c as f64 + r*(a-PI/2.).cos() + r*(a-PI).cos();
        let y1 = c as f64 - r*(a-PI/2.).sin() - r*(a-PI).sin();
        let x2 = x1 + d*(a).cos();
        let y2 = y1 - d*(a).sin();
        let x3 = x2 + d*(a+PI/2.).cos();
        let y3 = y2 - d*(a+PI/2.).sin();
        let x4 = x3 + d*(a+PI).cos();
        let y4 = y3 - d*(a+PI).sin();
        let points = [
            vec2f_xy!(y1.round(), x1.round()),
            vec2f_xy!(x2.round(), y2.round()),
            vec2f_xy!(x3.round(), y3.round()),
            vec2f_xy!(x4.round(), y4.round())
        ];

        let shape = crate::algorithms::polygon::polygon(&points);
        let off = (size / 2) as f64;
        Self {
            shape,
            bb: (size as f64, size as f64),
            offset: (off, off),
        }
    }


    pub fn circle(size: i32) -> Self {
        if size == 1 { return Self::pixel() }
        if size == 3 { return Self::cross() }
        let shape = crate::algorithms::ellipse::algo_ellipsefill(0, 0, size - 1, size - 1);
        let off = (size / 2) as f64;
        Self {
            shape,
            bb: (size as f64, size as f64),
            offset: (off, off),
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
    }

    // #[test]
    // fn test_circle_brush2() {
    //     use super::*;
    //     assert_eq!(Brush::circle(2), Brush::pixel());
    // }

    #[test]
    fn test_circle_brush3() {
        use super::*;
        assert_eq!(Brush::circle(3), Brush::cross());
    }

    #[test]
    fn test_circle_brush4() {
        use super::*;
        assert_eq!(Brush::circle(4), Brush {
            shape: pixels!(
                pixel!(3,1,Color::red()),
                pixel!(0,1,Color::red()),
                pixel!(1,0,Color::red()),
                pixel!(1,1,Color::red()),
                pixel!(1,2,Color::red()),
                pixel!(2,0,Color::red()),
                pixel!(2,1,Color::red()),
                pixel!(2,2,Color::red())
            ),
            bb: (4.0, 4.0),
            offset: (2.0, 2.0)
        });
    }

    #[test]
    fn test_square_brush1() {
        use super::*;
        let brush = Brush::square(1, 0.);
        assert_eq!(brush, Brush::pixel());
    }
}