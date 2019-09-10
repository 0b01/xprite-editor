use crate::prelude::*;

/// History
///     ~> Frames
///         ~> Layers
#[derive(Debug)]
pub struct History {
    stack: Vec<Frames>,
    redos: Vec<Frames>,
}

impl Default for History {
    fn default() -> Self {
        Self::new()
    }
}

impl History {
    pub fn new() -> Self {
        let stack = vec![Frames::new()];
        let redos = vec![];
        History { stack, redos }
    }

    pub fn duplicate(&mut self) {
        trace!("duplicate history");
        let latest = self.top().clone();
        self.stack.push(latest);
    }

    pub fn top_mut(&mut self) -> &mut Frames {
        self.stack.last_mut().unwrap()
    }

    pub fn top(&self) -> &Frames {
        self.stack.last().unwrap()
    }

    pub fn clear_redo(&mut self) {
        self.redos.clear();
    }

    pub fn undo(&mut self) {
        info!("undo");
        // invariant: must have 1 item(empty canvas)
        if self.stack.len() == 1 {
            return;
        }

        if let Some(last) = self.stack.pop() {
            self.redos.push(last);
        }
    }

    pub fn redo(&mut self) {
        if let Some(last) = self.redos.pop() {
            self.stack.push(last);
        }
    }
}
