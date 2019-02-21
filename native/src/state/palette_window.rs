use std::borrow::Cow;

pub struct PaletteWindowState<'a> {
    pub palette_color_name: Option<Cow<'a, str>>,
    pub palette_idx: i32,
}

impl<'a> Default for PaletteWindowState<'a> {
    fn default() -> Self {
        Self {
            palette_color_name: None,
            palette_idx: 0,
        }
    }
}