use crate::prelude::*;
use std::cell::{Ref, RefMut};
use std::rc::Rc;
use std::cell::RefCell;

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
    pub fn find(&self, old: &Rc<RefCell<Layer>>) -> Result<Rc<RefCell<Layer>>, String> {
        Layers::_find(&self.layers, old)
    }

    /// find a layer that is structurally equal
    fn _find(layers: &[Rc<RefCell<Layer>>], old: &Rc<RefCell<Layer>>) -> Result<Rc<RefCell<Layer>>, String> {
        let new = layers.iter()
            .find(|i|
                *i.borrow() == *old.borrow()
            )
            .ok_or("Cannot `_find` same layer".to_owned())?
            .clone();
        Ok(new)
    }

    pub fn deepcopy(&self) -> Result<Layers, String> {
        let layers: Vec<_> = self.layers.iter().map(|i|
            Rc::new(RefCell::new(i.borrow().clone()))
        ).collect();
        let selected_layer = Layers::_find(&layers, &self.selected_layer)?;
        Ok(Self {
            layers,
            selected_layer,
        })
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
