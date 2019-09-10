use crate::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Frames {
    frames: Vec<Layers>,
    current_frame_idx: usize,
}

impl Frames {

    pub fn new() -> Self {
        Self {
            frames: vec![Layers::new()],
            current_frame_idx: 0,
        }
    }

    pub fn frame_mut(&mut self) -> &mut Layers {
        &mut self.frames[self.current_frame_idx]
    }

    pub fn frame(&self) -> &Layers {
        &self.frames[self.current_frame_idx]
    }

    pub fn cel(&self) -> Option<Rc<RefCell<Layer>>> {
        self.frame().layer()
    }

}