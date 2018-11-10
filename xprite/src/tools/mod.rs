use crate::prelude::*;

pub mod pencil;
pub mod line;
pub mod paint_bucket;
pub mod traits;

pub use self::traits::Tool;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ToolType {
    Pencil,
    Line,
    PaintBucket,
}

impl ToolType {
    pub fn as_str(&self) -> &str {
        match self {
            ToolType::Pencil => "Pencil",
            ToolType::Line => "Line",
            ToolType::PaintBucket => "PaintBucket",
        }
    }
}