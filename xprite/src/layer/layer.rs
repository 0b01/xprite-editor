use crate::prelude::*;
use std::cell::{Ref, RefMut};

pub struct Layer {
    name: String,
    content: Pixels,
    visible: bool,
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

    pub fn pixels(&self) -> &Pixels {
        &self.content
    }

    pub fn pixels_mut(&mut self) -> &mut Pixels {
        &mut self.content
    }
}
