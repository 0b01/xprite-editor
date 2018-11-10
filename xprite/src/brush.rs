use crate::prelude::*;

pub struct Brush {
    pub shape: PixelOffsets,
    pub size: (f32, f32),
    pub offset: (f32, f32),
}

impl Brush {
    pub fn pixel() -> Self {
        let mut pxs = Pixels::new();
        pxs.push(pixel!(0., 0.));

        Self {
            shape: pxs,
            size: (1., 1.),
            offset: (0., 0.),
        }
    }

    pub fn cross() -> Self {
        let mut pxs = Pixels::new();
        pxs.push(pixel!(0., 1.));
        pxs.push(pixel!(1., 0.));
        pxs.push(pixel!(1., 1.));
        pxs.push(pixel!(1., 2.));
        pxs.push(pixel!(2., 1.));

        Self {
            shape: pxs,
            size: (3., 3.),
            offset: (-1., -1.),
        }
    }
}
