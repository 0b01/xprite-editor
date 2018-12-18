use crate::prelude::*;

pub mod pencil;
pub mod eraser;
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
    Eraser,
}

impl ToolType {
    pub fn as_str(&self) -> &str {
        match self {
            ToolType::Pencil => "Pencil",
            ToolType::Line => "Line",
            ToolType::PaintBucket => "PaintBucket",
            ToolType::Vector => "Vector",
            ToolType::ColorPicker => "ColorPicker",
            ToolType::Eraser => "Eraser",
        }
    }

    pub const VARIANTS: [ToolType; 6] = [
        ToolType::Pencil,
        ToolType::Line,
        ToolType::PaintBucket,
        ToolType::Vector,
        ToolType::ColorPicker,
        ToolType::Eraser,
    ];
}