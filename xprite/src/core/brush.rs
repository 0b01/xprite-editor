use crate::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum BrushType {
    Pixel,
    Cross,
}

impl BrushType {
    pub fn as_str(&self) -> &str {
        match self {
            BrushType::Pixel => ".",
            BrushType::Cross => "+",
        }
    }
    pub const VARIANTS: [BrushType; 2] = [
        BrushType::Pixel,
        BrushType::Cross,
    ];
}

#[derive(Debug)]
pub struct Brush {
    pub shape: PixelOffsets,
    pub bb: (f32, f32),
    pub offset: (f32, f32),
}

impl Brush {
    pub fn pixel() -> Self {
        let mut pxs = Pixels::new();
        pxs.push(pixel!(0., 0.,Color::red()));

        Self {
            shape: pxs,
            bb: (1., 1.),
            offset: (0., 0.),
        }
    }

    pub fn cross() -> Self {
        let mut pxs = Pixels::new();
        pxs.push(pixel!(0., 1., Color::red()));
        pxs.push(pixel!(1., 0., Color::red()));
        pxs.push(pixel!(1., 1., Color::red()));
        pxs.push(pixel!(1., 2., Color::red()));
        pxs.push(pixel!(2., 1., Color::red()));

        Self {
            shape: pxs,
            bb: (3., 3.),
            offset: (-1., -1.),
        }
    }

    pub fn follow_stroke(&self, stroke: &Pixels) -> Option<Pixels> {
        let mut ret = Pixels::new();
        for Pixel{point, ..} in &stroke.0 {
            if let Some(pixs) = self.to_canvas_pixels(*point, Color::red()) {
                ret.extend(&pixs);
            }
        }
        return Some(ret)
    }

    /// convert brush shape to actual pixel on canvas
    pub fn to_canvas_pixels(&self, cursor: Vec2D, color: Color) -> Option<Pixels> {
        let Vec2D {x, y} = cursor;
        let (offset_x, offset_y) = self.offset;
        let ret: Vec<Pixel> = self.shape.iter().map(
            |Pixel {point,..}| Pixel {
                point: Vec2D::new(point.x+x + offset_x, point.y+y + offset_y),
                color: color,
            }
        ).collect();
        Some(Pixels::from_slice(&ret))
    }
}
