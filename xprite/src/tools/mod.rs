use crate::prelude::*;

pub mod color_picker;
pub mod ellipse;
pub mod eraser;
pub mod line;
pub mod marquee;
pub mod paint_bucket;
pub mod pencil;
pub mod rect;
pub mod symmetry;
pub mod texture;
pub mod vector;
pub mod autoshade;

pub mod traits;

pub use self::traits::Tool;
use std::str::FromStr;

#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Copy, Debug)]
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
    Marquee,
    Settings,
    Symmetry,
    AutoShade,
}

impl Default for ToolType {
    fn default() -> Self {
        ToolType::Pencil
    }
}

impl ToolType {
    pub const VARIANTS: [ToolType; 10] = [
        ToolType::Pencil,
        ToolType::Line,
        ToolType::PaintBucket,
        ToolType::Vector,
        ToolType::Eraser,
        ToolType::Rect,
        ToolType::Texture,
        ToolType::Ellipse,
        ToolType::Marquee,
        ToolType::AutoShade,
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
            ToolType::Marquee => "Marquee",
            ToolType::Settings => "Settings",
            ToolType::Symmetry => "Symmetry",
            ToolType::AutoShade => "AutoShade",
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
            "Marquee" => Ok(ToolType::Marquee),
            "Settings" => Ok(ToolType::Settings),
            "Symmetry" => Ok(ToolType::Symmetry),
            "AutoShade" => Ok(ToolType::AutoShade),
            _ => Err(()),
        }
    }
}
