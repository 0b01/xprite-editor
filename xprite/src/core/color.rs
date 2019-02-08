use crate::prelude::*;
use hex;

#[cfg_attr(feature = "python-scripting", pyclass)]
#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[cfg(feature = "python-scripting")]
impl<'a> pyo3::FromPyObject<'a> for Color {
    fn extract(ob: &'a pyo3::types::PyObjectRef) -> PyResult<Color> {
        let tup: &pyo3::types::PyTuple = ob.extract()?;
        let ret: Color = Color::new(
            tup.get_item(0).extract().unwrap(),
            tup.get_item(1).extract().unwrap(),
            tup.get_item(2).extract().unwrap(),
        );
        Ok(ret)
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a,)
    }
}

impl From<Color> for ase::RGBA256 {
    fn from(c: Color) -> Self {
        ase::RGBA256 {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        }
    }
}

impl From<ase::RGBA256> for Color {
    fn from(c: ase::RGBA256) -> Self {
        Self {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        }
    }
}

impl From<Color> for (i32, i32, i32, i32) {
    fn from(c: Color) -> Self {
        (c.r as i32, c.g as i32, c.b as i32, c.a as i32)
    }
}

impl From<(i32, i32, i32, i32)> for Color {
    fn from(c: (i32, i32, i32, i32)) -> Self {
        Color {
            r: c.0 as u8,
            g: c.1 as u8,
            b: c.2 as u8,
            a: c.3 as u8,
        }
    }
}

/// [f64;4] = [0.,0.,0.,0.9];
impl From<Color> for [f32; 4] {
    fn from(c: Color) -> Self {
        [
            c.r as f32 / 255.,
            c.g as f32 / 255.,
            c.b as f32 / 255.,
            c.a as f32 / 255.,
        ]
    }
}

/// [f64;4] = [0.,0.,0.,0.9];
impl From<[f32; 4]> for Color {
    fn from(c: [f32; 4]) -> Self {
        Color {
            r: (c[0] * 255.).floor() as u8,
            g: (c[1] * 255.).floor() as u8,
            b: (c[2] * 255.).floor() as u8,
            a: (c[3] * 255.).floor() as u8,
        }
    }
}

impl From<img::Rgba<u8>> for Color {
    fn from(c: img::Rgba<u8>) -> Self {
        Color {
            r: c[0],
            g: c[1],
            b: c[2],
            a: c[3],
        }
    }
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Color { r, g, b, a: 255 }
    }

    pub fn from_hex(col: &str) -> Result<Self, hex::FromHexError> {
        let r = hex::decode(&col[..2])?[0];
        let g = hex::decode(&col[2..4])?[0];
        let b = hex::decode(&col[4..])?[0];
        Ok(Self::new(r, g, b))
    }

    pub fn white() -> Color {
        Color {
            r: 255,
            g: 255,
            b: 255,
            a: 255,
        }
    }

    pub fn red() -> Color {
        Color {
            r: 255,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub fn green() -> Color {
        Color {
            r: 0,
            g: 255,
            b: 0,
            a: 255,
        }
    }

    pub fn blue() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 255,
            a: 255,
        }
    }

    pub fn black() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        }
    }

    pub fn orange() -> Color {
        Color {
            r: 255,
            g: 128,
            b: 0,
            a: 255,
        }
    }

    pub fn grey() -> Color {
        Color {
            r: 1,
            g: 1,
            b: 1,
            a: 255,
        }
    }

    pub fn transparent() -> Color {
        Color {
            r: 0,
            g: 0,
            b: 0,
            a: 0,
        }
    }
}
