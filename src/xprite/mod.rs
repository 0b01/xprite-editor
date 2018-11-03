#[macro_use]
mod common;
mod tools;
mod history;
mod canvas;
mod toolbox;
mod xprite;


pub use self::toolbox::Toolbox;
pub use self::history::History;
pub use self::canvas::Canvas;
pub use self::xprite::{Xprite, Event};
pub use self::common::pixel::Pixel;
pub use self::common::pixels::Pixels;
pub use self::common::path::Path;
pub use self::common::color::{Color, ColorOption};
pub use self::common::brush::Brush;
pub use self::common::polyline::Polyline;


pub type PixelOffsets = Pixels;

use stdweb::web::event::MouseButton;
use self::common::geom::{Point2D, Size2D, CubicBezierSegment};
