// re-exports
pub use crate::core::{
    toolbox::Toolbox,
    history::History,
    pixels::{Pixel, Pixels},
    color::Color,
    input::{InputEvent, InputItem, InputState},
    brush::{Brush, BrushType},
    geom::{Vec2D, CubicBezierSegment},
    xprite::Xprite,
    palette::PaletteManager,
};
pub use crate::algorithms::{
    polyline::Polyline,
    path::Path,
};
pub use crate::rendering::{
    traits::Renderer,
    canvas::Canvas,
    image_renderer::{ImageRenderer, save_img},
};

pub use crate::tools::{Tool, ToolType};
pub use crate::layer::{Layer, Layers};

// type aliases
pub type PixelOffsets = Pixels;
pub type Circles = Pixels;

// constants
pub const BGCOLOR: [f32; 4] = [0.,0.,0.,0.];


#[cfg(not(debug_assertions))]
pub const BACKGROUND: [f32;4] = [0.2,0.2,0.3,1.];
#[cfg(debug_assertions)]
pub const BACKGROUND: [f32;4] = [0.2,0.2,0.2,1.];

pub const LIGHT_GREY: [f32;4] = [0.8,0.8,0.8,1.];

/// out of bounds checking
pub fn oob(x: f32, y: f32, w: f32, h: f32) -> bool {
    if x < 0. || x >= w { return true; }
    if y < 0. || y >= h { return true; }
    false
}