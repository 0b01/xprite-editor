use crate::core::xprite::Xprite;
use hex;

#[derive(Debug, Hash, Copy, Clone, Eq)]
pub enum Color {
    Indexed(usize),
    Rgba(XpriteRgba),
}

impl Default for Color {
    fn default() -> Self {
        Color::Rgba(Default::default())
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Color::Indexed(a), Color::Indexed(b)) if a == b => true,
            (Color::Rgba(a), Color::Rgba(b)) if a == b => true,
            _ => false,
        }
    }
}

#[cfg_attr(feature = "python-scripting", pyclass)]
#[derive(Debug, Hash, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct XpriteRgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {

    pub fn from_hex(col: &str) -> Result<Self, hex::FromHexError> {
        XpriteRgba::from_hex(col)
            .map(|i| Color::Rgba(i))
    }

    pub unsafe fn as_rgba(&self) -> XpriteRgba {
        match self {
            Color::Indexed(_) => unsafe {
                use std::hint::unreachable_unchecked;
                unreachable_unchecked();
            }
            Color::Rgba(rgba) => *rgba,
        }
    }

    pub fn to_rgba(&self, xpr: Option<&Xprite>) -> Option<XpriteRgba> {
        match *self {
            Color::Indexed(i) => {
                if xpr.is_none() { return None }
                xpr.unwrap().indexed_palette.get(i).map(|&c| c)
            }
            Color::Rgba(c) => Some(c)
        }
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color::Rgba(XpriteRgba {r, g, b, a})
    }

    pub fn white() -> Color {
        Color::Rgba(XpriteRgba::white())
    }

    pub fn red() -> Color {
        Color::Rgba(XpriteRgba::red())
    }

    pub fn green() -> Color {
        Color::Rgba(XpriteRgba::green())
    }

    pub fn blue() -> Color {
        Color::Rgba(XpriteRgba::blue())
    }

    pub fn black() -> Color {
        Color::Rgba(XpriteRgba::black())
    }

    pub fn orange() -> Color {
        Color::Rgba(XpriteRgba::orange())
    }

    pub fn grey() -> Color {
        Color::Rgba(XpriteRgba::grey())
    }

    pub fn transparent() -> Color {
        Color::Rgba(XpriteRgba::transparent())
    }

    pub fn void() -> Color {
        Color::Rgba(XpriteRgba::void())
    }

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
        match self {
            Color::Indexed(i) => {
                format!("indexed({})", i)
            }
            Color::Rgba(c) => {
                c.to_string()
            }
        }
    }
}

impl ToString for XpriteRgba {
    fn to_string(&self) -> String {
        format!("rgba({}, {}, {}, {})", self.r, self.g, self.b, self.a, )
    }
}

impl From<XpriteRgba> for ase::RGBA256 {
    fn from(c: XpriteRgba) -> Self {
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
        Self::Rgba(XpriteRgba {
            r: c.r,
            g: c.g,
            b: c.b,
            a: c.a,
        })
    }
}

impl From<XpriteRgba> for (i32, i32, i32, i32) {
    fn from(c: XpriteRgba) -> Self {
        (c.r as i32, c.g as i32, c.b as i32, c.a as i32)
    }
}

impl From<(i32, i32, i32, i32)> for Color {
    fn from(c: (i32, i32, i32, i32)) -> Self {
        Color::Rgba( XpriteRgba {
            r: c.0 as u8,
            g: c.1 as u8,
            b: c.2 as u8,
            a: c.3 as u8,
        })
    }
}

/// [f64;4] = [0.,0.,0.,0.9];
impl From<XpriteRgba> for [f32; 4] {
    fn from(c: XpriteRgba) -> Self {
        [c.r as f32 / 255., c.g as f32 / 255., c.b as f32 / 255., c.a as f32 / 255.]
    }
}

/// [f64;4] = [0.,0.,0.,0.9];
impl From<[f32; 4]> for Color {
    fn from(c: [f32; 4]) -> Self {
        Color::Rgba(XpriteRgba {
            r: (c[0] * 255.).floor() as u8,
            g: (c[1] * 255.).floor() as u8,
            b: (c[2] * 255.).floor() as u8,
            a: (c[3] * 255.).floor() as u8,
        })
    }
}

impl From<img::Rgba<u8>> for Color {
    fn from(c: img::Rgba<u8>) -> Self {
        Color::Rgba(XpriteRgba {
            r: c[0],
            g: c[1],
            b: c[2],
            a: c[3],
        })
    }
}

impl From<XpriteRgba> for [u8; 4] {
    fn from(c: XpriteRgba) -> [u8; 4] {
        [c.r, c.g, c.b, c.a]
    }
}

impl XpriteRgba {
    pub fn from_hex(col: &str) -> Result<Self, hex::FromHexError> {
        let r = hex::decode(&col[..2])?[0];
        let g = hex::decode(&col[2..4])?[0];
        let b = hex::decode(&col[4..])?[0];
        Ok(XpriteRgba{r, g, b, a: 255})
    }

    pub fn white() -> XpriteRgba {
        XpriteRgba { r: 255, g: 255, b: 255, a: 255, }
    }

    pub fn red() -> XpriteRgba {
        XpriteRgba { r: 255, g: 0, b: 0, a: 255 }
    }

    pub fn green() -> XpriteRgba {
        XpriteRgba { r: 0, g: 255, b: 0, a: 255 }
    }

    pub fn blue() -> XpriteRgba {
        XpriteRgba { r: 0, g: 0, b: 255, a: 255 }
    }

    pub fn black() -> XpriteRgba {
        XpriteRgba { r: 0, g: 0, b: 0, a: 255 }
    }

    pub fn orange() -> XpriteRgba {
        XpriteRgba { r: 255, g: 128, b: 0, a: 255 }
    }

    pub fn grey() -> XpriteRgba {
        XpriteRgba { r: 1, g: 1, b: 1, a: 255 }
    }

    pub fn transparent() -> XpriteRgba {
        XpriteRgba { r: 0, g: 0, b: 0, a: 0 }
    }

    pub fn void() -> XpriteRgba {
        XpriteRgba::transparent()
    }
}
