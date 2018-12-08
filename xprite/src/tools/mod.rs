use crate::prelude::*;

pub mod pencil;
pub mod vector;
pub mod line;
pub mod paint_bucket;
pub mod color_picker;
pub mod traits;

pub use self::traits::Tool;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Copy)]
pub enum ToolType {
    Pencil,
    Line,
    PaintBucket,
    Vector,
    ColorPicker,
}

impl ToolType {
    pub fn as_str(&self) -> &str {
        match self {
            ToolType::Pencil => "Pencil",
            ToolType::Line => "Line",
            ToolType::PaintBucket => "PaintBucket",
            ToolType::Vector => "Vector",
            ToolType::ColorPicker => "ColorPicker",
        }
    }

    pub const VARIANTS: [ToolType; 5] = [
        ToolType::Pencil,
        ToolType::Line,
        ToolType::PaintBucket,
        ToolType::Vector,
        ToolType::ColorPicker,
    ];
}