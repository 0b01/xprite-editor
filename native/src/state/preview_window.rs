use crate::prelude::*;

use std::str::FromStr;

#[derive(Clone, Copy, PartialEq)]
pub enum PreviewWindowMode {
    Fill,
    OneX,
    TwoX,
}

impl PreviewWindowMode {
    pub fn as_str(&self) -> &str {
        match self {
            PreviewWindowMode::Fill => "Fill",
            PreviewWindowMode::OneX => "1x",
            PreviewWindowMode::TwoX => "2x",
        }
    }

    pub const VARIANTS: [PreviewWindowMode; 3] = [
        PreviewWindowMode::Fill,
        PreviewWindowMode::OneX,
        PreviewWindowMode::TwoX,
    ];
}

impl FromStr for PreviewWindowMode {
    type Err = ();
    fn from_str(string: &str) -> Result<Self, ()> {
        match string {
            "Fill" => Ok(PreviewWindowMode::Fill),
            "1x" => Ok(PreviewWindowMode::OneX),
            "2x" => Ok(PreviewWindowMode::TwoX),
            _ => panic!("impossible"),
        }
    }
}

pub struct PreviewWindowState {
    pub mode: PreviewWindowMode,
}

impl Default for PreviewWindowState {
    fn default() -> Self {
        Self {
            mode: PreviewWindowMode::Fill,
        }
    }
}
