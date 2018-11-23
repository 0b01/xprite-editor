pub use crate::tools::{Tool, ToolType};
pub use crate::toolbox::Toolbox;
pub use crate::history::History;
pub use crate::pixels::{Pixel, Pixels};
pub use crate::algorithms::polyline::Polyline;
pub use crate::xprite::Xprite;
pub use crate::algorithms::path::Path;
pub use crate::color::{Color, ColorOption};
pub use crate::brush::Brush;
pub use crate::geom::{Point2D, CubicBezierSegment};
pub use crate::input::{InputEvent, InputItem};
pub use crate::rendering::canvas::Canvas;
pub use crate::layer::{Layer, Layers};

pub type PixelOffsets = Pixels;

pub const WHITE: [f32;4] = [1.,1.,1.,1.];
pub const RED: [f32;4] = [1.,0.,0.,1.];
pub const GREY: [f32;4] = [0.2,0.2,0.2,1.];
pub const LIGHT_GREY: [f32;4] = [0.8,0.8,0.8,1.];
pub const BLACK: [f32;4] = [0.,0.,0.,1.];