use xprite::Pixels;

#[derive(Debug)]
pub struct History {
    stack: Vec<Pixels>,
    redos: Vec<Pixels>,
}

impl History {
    pub fn new() -> Self {
        let stack = vec![Pixels::new()];
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
        let latest = self.current_pixels().clone();
        self.stack.push(latest);
    }

    pub fn current_pixels_mut(&mut self) -> &mut Pixels {
        self.stack.last_mut().unwrap()
    }

    pub fn current_pixels(&self) -> &Pixels {
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
