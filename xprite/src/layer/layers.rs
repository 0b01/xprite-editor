use std::rc::Rc;
use std::cell::RefCell;

use std::cell::{Ref, RefMut};

use crate::prelude::*;

pub struct Layers {
    layers: Vec<Rc<RefCell<Layer>>>,
    current: Rc<RefCell<Layer>>,
    count: u32,
}

impl Layers {
    pub fn new() -> Self {
        let count = 0;
        let current = Rc::new(RefCell::new(Layer::new("0")));
        let layers = vec![current.clone()];
        Self {
            layers,
            current,
            count,
        }
    }

    pub fn pixels(&self) -> Ref<'_, Pixels> {
        Ref::map(self.current.borrow(), |layer| layer.pixels())
    }

    pub fn pixels_mut(&self) -> RefMut<'_, Pixels> {
        RefMut::map(self.current.borrow_mut(), |layer| layer.pixels_mut())
    }
}
