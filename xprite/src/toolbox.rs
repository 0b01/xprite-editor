use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use crate::tools::{
    Tool,
    pencil::Pencil,
    line::Line,
    paint_bucket::PaintBucket,
};

pub struct Toolbox {
    /// tool singletons
    tools: HashMap<&'static str, Rc<RefCell<Tool>>>,
    selected: Rc<RefCell<Tool>>,
}

impl Toolbox {
    pub fn new() -> Self {
        let mut tools: HashMap<&'static str, Rc<RefCell<Tool>>> = HashMap::new();

        let pencil = Rc::new(RefCell::new(Pencil::new()));
        tools.insert("pencil", pencil.clone());

        let line = Rc::new(RefCell::new(Line::new()));
        tools.insert("line", line.clone());

        let paint_bucket = Rc::new(RefCell::new(PaintBucket::new()));
        tools.insert("paint_bucket", paint_bucket.clone());

        let selected = line;

        Toolbox {
            tools,
            selected,
        }
    }

    pub fn tool(&self) -> Rc<RefCell<Tool>> {
        self.selected.clone()
    }

    pub fn get(&self, name: &str) -> Option<Rc<RefCell<Tool>>> {
        if let Some(tool) = self.tools.get(name) {
            Some(tool.clone())
        } else {
            None
        }
    }

    pub fn change_to(&mut self, name: &str) {
        if let Some(tool) = self.tools.get(name) {
            self.selected = tool.clone();
        }
    }
}
