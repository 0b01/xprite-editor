use xprite::{Pixel, Pixels};
use xprite::Color;
use xprite::PixelOffsets;
use lyon_geom::euclid::Point2D;

pub struct Brush {
    pub shape: PixelOffsets,
    pub size: (u32, u32),
}

impl Brush {
    pub fn pixel() -> Self {
        let mut pxs = Pixels::new();
        pxs.insert(pixel!(0, 0));

        Self {
            shape: pxs,
            size: (1, 1),
        }
    }

    pub fn cross() -> Self {
        let mut pxs = Pixels::new();
        pxs.insert(pixel!(0, 1));
        pxs.insert(pixel!(1, 0));
        pxs.insert(pixel!(1, 1));
        pxs.insert(pixel!(1, 2));
        pxs.insert(pixel!(2, 1));

        Self {
            shape: pxs,
            size: (3, 3),
        }
    }
}
