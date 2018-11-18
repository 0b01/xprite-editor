use std::rc::Rc;
use std::cell::RefCell;
// use std::slice::{Iter, IterMut};

use std::cell::{Ref, RefMut};

use crate::prelude::*;

#[derive(Debug)]
pub struct Layers {
    pub layers: Vec<Rc<RefCell<Layer>>>,
    pub selected_layer: Rc<RefCell<Layer>>,
}

impl Clone for Layers {
    fn clone(&self) -> Self {
        let layers: Vec<_> = self.layers.iter().map(|i|
            Rc::new(RefCell::new(i.borrow().clone()))
        ).collect();

        let selected_layer = Layers::_find(&layers, &self.selected_layer);

        Self {
            layers,
            selected_layer,
        }
    }
}

impl Layers {
    pub fn new() -> Self {
        let selected_layer = Rc::new(RefCell::new(Layer::new("Layer 0".to_owned())));
        let layers = vec![selected_layer.clone()];
        Self {
            layers,
            selected_layer,
        }
    }

    pub fn is_selected(&self, layer: &Rc<RefCell<Layer>>) -> bool {
        &self.selected_layer == layer
    }

    pub fn find(&self, old: &Rc<RefCell<Layer>>) -> Rc<RefCell<Layer>> {
        Layers::_find(&self.layers, old)
    }

    fn _find(layers: &[Rc<RefCell<Layer>>], old: &Rc<RefCell<Layer>>) -> Rc<RefCell<Layer>> {
        let new = layers.iter()
            .find(|i|
                *i.borrow() == *old.borrow()
            )
            .unwrap()
            .clone();
        new
    }


    pub fn add(&mut self, name: Option<&str>) {
        let name = name
                .and_then(|i: &str| Some(i.to_owned()))
                .unwrap_or_else(||
                    format!("Layer {}", self.layers.len())
                );
        let layer = Rc::new(RefCell::new(Layer::new(name)));
        self.layers.push(layer);
    }

    pub fn duplicate_layer(&mut self, name: &str) -> Option<()> {
        let new_layer = {
            let layer = self.layers.iter().find(|&layer| layer.borrow().name == name)?;
            let b = layer.borrow();
            let old_content = b.pixels();
            Layer::new(format!("{}(copy)", name)).with_pixels(old_content)
        };
        self.layers.push(Rc::new(RefCell::new(new_layer)));
        Some(())
    }

    pub fn duplicate_current(&mut self) -> Option<()> {
        let current_name = {
            let b = self.selected_layer.borrow();
            b.name.to_owned()
        };
        self.duplicate_layer(&current_name)
    }

    pub fn pixels(&self) -> Ref<'_, Pixels> {
        Ref::map(self.selected_layer.borrow(), |layer| layer.pixels())
    }

    pub fn pixels_mut(&self) -> RefMut<'_, Pixels> {
        RefMut::map(self.selected_layer.borrow_mut(), |layer| layer.pixels_mut())
    }
}
