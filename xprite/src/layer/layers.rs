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

impl Default for Layers {
    fn default() -> Self {
        Self::new()
    }
}

impl Layers {
    pub fn deepcopy(&self) -> Option<Layers> {
        let layers: Vec<_> = self.layers.iter().map(|i|
            Rc::new(RefCell::new(i.borrow().clone()))
        ).collect();

        let selected_layer = Layers::_find(&layers, &self.selected_layer)?;

        Some(Self {
            layers,
            selected_layer,
        })
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

    /// find a layer that is structurally equal
    pub fn find(&self, old: &Rc<RefCell<Layer>>) -> Option<Rc<RefCell<Layer>>> {
        Layers::_find(&self.layers, old)
    }

    /// find a layer that is structurally equal
    fn _find(layers: &[Rc<RefCell<Layer>>], old: &Rc<RefCell<Layer>>) -> Option<Rc<RefCell<Layer>>> {
        let new = layers.iter()
            .find(|i|
                *i.borrow() == *old.borrow()
            )?
            .clone();
        Some(new)
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

    pub fn duplicate_layer(&mut self, orig: &Rc<RefCell<Layer>>) {
        let new_layer = orig.borrow().clone();
        self.layers.push(Rc::new(RefCell::new(new_layer)));
    }

    pub fn duplicate_current(&mut self) {
        self.duplicate_layer(&self.selected_layer.clone());
    }

    pub fn remove_layer(&mut self, to_remove: &Rc<RefCell<Layer>>) {
        let i = self.find(to_remove).unwrap();
        self.layers.remove_item(&i);
    }

    pub fn pixels(&self) -> Ref<'_, Pixels> {
        Ref::map(self.selected_layer.borrow(), |layer| layer.pixels())
    }

    pub fn pixels_mut(&self) -> RefMut<'_, Pixels> {
        RefMut::map(self.selected_layer.borrow_mut(), |layer| layer.pixels_mut())
    }
}
