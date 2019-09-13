use crate::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Frames {
    frames: Vec<Layers>,
    pub current_frame_idx: usize,
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

    pub fn set_frame_index(&mut self, idx: usize) {
        self.current_frame_idx = idx;
    }

    pub fn count(&self) -> usize {
        self.frames.len()
    }

    pub fn add_frame_after_current(&mut self) {
        let idx = self.current_frame_idx;
        let copy = self.frame().clone();
        self.frames.insert(idx, copy);
    }

}