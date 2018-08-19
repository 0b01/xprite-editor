use xprite::Blocks;
use std::collections::HashSet;

pub struct History {
    stack: Vec<Blocks>,
    redos: Vec<Blocks>,
}

impl History {
    pub fn new() -> Self {
        let stack = vec![HashSet::new()];
        let redos = vec![];
        History {
            stack,
            redos,
        }
    }

    pub fn on_new_stroke_start(&mut self) {
        self.duplicate();
        self.clear_redo();
    }

    pub fn duplicate(&mut self) {
        let latest = self.current_block().clone();
        self.stack.push(latest);
    }

    pub fn current_block_mut(&mut self) -> &mut Blocks {
        self.stack.last_mut().unwrap()
    }

    pub fn current_block(&self) -> &Blocks {
        self.stack.last().unwrap()
    }

    pub fn clear_redo(&mut self) {
        self.redos = Vec::new();
    }

    pub fn undo(&mut self) {
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
