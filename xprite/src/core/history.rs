use crate::prelude::*;


#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    stack: Vec<Layers>,
    redos: Vec<Layers>,
}

impl History {
    pub fn new() -> Self {
        let stack = vec![Layers::new()];
        let redos = vec![];
        History {
            stack,
            redos,
        }
    }

    pub fn enter(&mut self) -> Result<(), String> {
        self.duplicate()?;
        self.clear_redo();
        Ok(())
    }

    pub fn duplicate(&mut self) -> Result<(), String> {
        trace!("duplicate history");
        let latest = self.top().clone();
        self.stack.push(latest);
        Ok(())
    }

    pub fn top_mut(&mut self) -> &mut Layers {
        self.stack.last_mut().unwrap()
    }

    pub fn top(&self) -> &Layers {
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
