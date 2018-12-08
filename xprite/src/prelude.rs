// re-exports
pub use crate::core::toolbox::Toolbox;
pub use crate::core::history::History;
pub use crate::core::pixels::{Pixel, Pixels};
pub use crate::core::color::{Color, ColorOption};
pub use crate::core::input::{InputEvent, InputItem, InputState};
pub use crate::core::brush::{Brush, BrushType};
pub use crate::core::geom::{Point2D, CubicBezierSegment};
pub use crate::core::xprite::Xprite;
pub use crate::algorithms::polyline::Polyline;
pub use crate::tools::{Tool, ToolType};
pub use crate::algorithms::path::Path;
pub use crate::rendering::canvas::Canvas;
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
