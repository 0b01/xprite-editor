pub use crate::algorithms::{path::Path, polyline::Polyline};
pub use crate::core::{
    brush::{Brush, BrushType},
    color::Color,
    geom::{CubicBezierSegment, Rect, Vec2f},
    history::History,
    input::{InputEvent, InputItem, InputState},
    outline::{MarqueePixel, Outline},
    palette::PaletteManager,
    pixels::{Pixel, Pixels},
    toolbox::Toolbox,
    xprite::Xprite,
};
pub use crate::rendering::{
    canvas::Canvas,
    image_renderer::{save_img, ImageRenderer},
    traits::Renderer,
};
/// re-exports
pub use palette::Srgb;
pub use std::rc::Rc;

#[cfg(feature = "python-scripting")]
pub use pyo3::prelude::*;
pub use std::f64::consts::PI;

pub use crate::layer::{Layer, Layers};
pub use crate::tools::{Tool, ToolType};

// type aliases
pub type PixelOffsets = Pixels;
pub type Circles = Pixels;

// constants
pub const BGCOLOR: [f32; 4] = [0., 0., 0., 0.];

#[cfg(not(debug_assertions))]
pub const BACKGROUND: [f32; 4] = [0.2, 0.2, 0.3, 1.];
#[cfg(debug_assertions)]
pub const BACKGROUND: [f32; 4] = [0.106, 0.118, 0.125, 1.000];

pub const BLACK: [f32; 4] = [0., 0., 0., 1.];
pub const LIGHT_GREY: [f32; 4] = [0.8, 0.8, 0.8, 1.];

/// out of bounds checking
pub fn oob(x: f64, y: f64, w: f64, h: f64) -> bool {
    if x < 0. || x >= w {
        return true;
    }
    if y < 0. || y >= h {
        return true;
    }
    false
}
