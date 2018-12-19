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
pub use crate::scripting::Scripting;
pub use crate::layer::{Layer, Layers};

// type aliases
pub type PixelOffsets = Pixels;
pub type Circles = Pixels;

// constants
pub const BGCOLOR: [f32; 4] = [0.,0.,0.,0.];
pub const WHITE: [f32;4] = [1.,1.,1.,1.];
pub const RED: [f32;4] = [1.,0.,0.,1.];
pub const GREY: [f32;4] = [0.2,0.2,0.2,1.];
pub const LIGHT_GREY: [f32;4] = [0.8,0.8,0.8,1.];
pub const BLACK: [f32;4] = [0.,0.,0.,1.];
