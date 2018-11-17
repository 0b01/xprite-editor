use crate::prelude::*;

pub struct Layer {
    pub name: String,
    pub content: Pixels,
    pub visible: bool,
}

impl Layer {
    pub fn new(name: &str) -> Self {
        let name = name.to_owned();
        let content = Pixels::new();
        let visible = true;

        Self {
            name,
            content,
            visible,
        }
    }

    pub fn with_pixels(mut self, content: &Pixels) -> Self {
        self.content = content.to_owned();
        self
    }

    pub fn pixels(&self) -> &Pixels {
        &self.content
    }

    pub fn pixels_mut(&mut self) -> &mut Pixels {
        &mut self.content
    }
}
