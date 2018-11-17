use std::rc::Rc;
use std::cell::RefCell;

use std::cell::{Ref, RefMut};

use crate::prelude::*;

pub struct Layers {
    layers: Vec<Rc<RefCell<Layer>>>,
    current: Rc<RefCell<Layer>>,
}

impl Layers {
    pub fn new() -> Self {
        let current = Rc::new(RefCell::new(Layer::new("0")));
        let layers = vec![current.clone()];
        Self {
            layers,
            current,
        }
    }

    pub fn add(&mut self, name: &str) {
        let layer = Rc::new(RefCell::new(Layer::new(name)));
        self.layers.push(layer);
    }

    pub fn duplicate_layer(&mut self, name: &str) -> Option<()> {
        let new_layer = {
            let layer = self.layers.iter().find(|&layer| layer.borrow().name == name)?;
            let b = layer.borrow();
            let old_content = b.pixels();
            Layer::new(&format!("{} Copy", name)).with_pixels(old_content)
        };
        self.layers.push(Rc::new(RefCell::new(new_layer)));
        Some(())
    }

    pub fn duplicate_top(&mut self) -> Option<()> {
        let current_name = {
            let b = self.current.borrow();
            b.name.to_owned()
        };
        self.duplicate_layer(&current_name)
    }

    pub fn pixels(&self) -> Ref<'_, Pixels> {
        Ref::map(self.current.borrow(), |layer| layer.pixels())
    }

    pub fn pixels_mut(&self) -> RefMut<'_, Pixels> {
        RefMut::map(self.current.borrow_mut(), |layer| layer.pixels_mut())
    }
}
