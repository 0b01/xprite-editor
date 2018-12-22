use crate::prelude::*;

pub mod pencil;
pub mod eraser;
pub mod vector;
pub mod line;
pub mod paint_bucket;
pub mod color_picker;
pub mod rect;
pub mod ellipse;
pub mod texture;

pub mod traits;

pub use self::traits::Tool;
use std::str::FromStr;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Copy, Debug, Serialize, Deserialize)]
pub enum ToolType {
    Pencil,
    Line,
    PaintBucket,
    Vector,
    ColorPicker,
    Eraser,
    Rect,
    FilledRect,
    Texture,
    Ellipse,
    FilledEllipse,
}

impl Default for ToolType {
    fn default() -> Self {
        ToolType::Pencil
    }
}

impl ToolType {
    pub const VARIANTS: [ToolType; 9] = [
        ToolType::Pencil,
        ToolType::Line,
        ToolType::PaintBucket,
        ToolType::Vector,
        ToolType::ColorPicker,
        ToolType::Eraser,
        ToolType::Rect,
        ToolType::Texture,
        ToolType::Ellipse,
    ];

    pub fn as_str(&self) -> &str {
        match self {
            ToolType::Pencil => "Pencil",
            ToolType::Line => "Line",
            ToolType::PaintBucket => "PaintBucket",
            ToolType::Vector => "Vector",
            ToolType::ColorPicker => "ColorPicker",
            ToolType::Eraser => "Eraser",
            ToolType::Rect => "Rect",
            ToolType::FilledRect => "FilledRect",
            ToolType::Ellipse => "Ellipse",
            ToolType::FilledEllipse => "FilledEllipse",
            ToolType::Texture => "Texture",
        }
    }
}

impl FromStr for ToolType {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, ()> {
        match string {
            "Pencil" => Ok(ToolType::Pencil),
            "Line" => Ok(ToolType::Line),
            "PaintBucket" => Ok(ToolType::PaintBucket),
            "Vector" => Ok(ToolType::Vector),
            "ColorPicker" => Ok(ToolType::ColorPicker),
            "Eraser" => Ok(ToolType::Eraser),
            "Rect" => Ok(ToolType::Rect),
            "FilledRect" => Ok(ToolType::FilledRect),
            "Ellipse" => Ok(ToolType::Ellipse),
            "FilledEllipse" => Ok(ToolType::FilledEllipse),
            "Texture" => Ok(ToolType::Texture),
            _ => Err(()),
        }
    }
}
