pub use crate::tools::Tool;
pub use crate::toolbox::Toolbox;
pub use crate::history::History;
pub use crate::pixels::{Pixel, Pixels};
pub use crate::algorithms::polyline::Polyline;
pub use crate::xprite::Xprite;
pub use crate::algorithms::path::Path;
pub use crate::color::{Color, ColorOption};
pub use crate::brush::Brush;
pub use crate::geom::{Point2D, CubicBezierSegment};
pub use crate::input::{MouseButton, MouseEvent};
pub use crate::rendering::canvas::Canvas;

pub type PixelOffsets = Pixels;

pub const WHITE: [f32;4] = [255.,255.,255.,0.9];
pub const RED: [f32;4] = [255.,0.,0.,0.9];
pub const GREY: [f32;4] = [241.,241.,241.,0.9];
pub const BLACK: [f32;4] = [0.,0.,0.,0.9];