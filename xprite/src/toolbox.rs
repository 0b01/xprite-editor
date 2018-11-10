use crate::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::tools::{
    Tool,
    pencil::Pencil,
    line::Line,
    paint_bucket::PaintBucket,
};

pub struct Toolbox {
    /// tool singletons
    pub tools: BTreeMap<ToolType, Rc<RefCell<Tool>>>,
    pub selected: Rc<RefCell<Tool>>,
}

impl Toolbox {
    pub fn new() -> Self {
        let mut tools: BTreeMap<ToolType, Rc<RefCell<Tool>>> = BTreeMap::new();

        let pencil = Rc::new(RefCell::new(Pencil::new()));
        tools.insert(ToolType::Pencil, pencil.clone());

        let line = Rc::new(RefCell::new(Line::new()));
        tools.insert(ToolType::Line, line.clone());

        let paint_bucket = Rc::new(RefCell::new(PaintBucket::new()));
        tools.insert(ToolType::PaintBucket, paint_bucket.clone());

        let selected = pencil;

        Toolbox {
            tools,
            selected,
        }
    }

    pub fn tool(&mut self) -> Rc<RefCell<Tool>> {
        self.selected.clone()
    }

    pub fn get(&self, name: &ToolType) -> Option<Rc<RefCell<Tool>>> {
        if let Some(tool) = self.tools.get(name) {
            Some(tool.clone())
        } else {
            None
        }
    }

    pub fn change_to(&mut self, name: &ToolType) {
        if let Some(tool) = self.tools.get(name) {
            self.selected = tool.clone();
        }
    }
}
