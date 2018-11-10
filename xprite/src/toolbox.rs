use crate::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;

use crate::tools::{
    Tool,
    pencil::Pencil,
    line::Line,
    paint_bucket::PaintBucket,
};

pub struct Toolbox {
    pub pencil:         Rc<RefCell<Pencil>>,
    pub paint_bucket:   Rc<RefCell<PaintBucket>>,
    pub line:           Rc<RefCell<Line>>,
    pub selected:       ToolType,
}

impl Toolbox {
    pub fn new() -> Self {
        let pencil = Rc::new(RefCell::new(Pencil::new()));
        let line = Rc::new(RefCell::new(Line::new()));
        let paint_bucket = Rc::new(RefCell::new(PaintBucket::new()));

        let selected = ToolType::Pencil;
        Toolbox {
            pencil,
            line,
            paint_bucket,
            selected,
        }
    }

    pub fn tool(&mut self) -> Rc<RefCell<Tool>> {
        self.get(&self.selected)
    }

    pub fn get(&self, name: &ToolType) -> Rc<RefCell<Tool>> {
        match name {
            Pencil => self.pencil.clone(),
            Line => self.line.clone(),
            PaintBucket => self.paint_bucket.clone(),
        }
    }

    pub fn change_tool(&mut self, tool: &ToolType) {
        self.selected = tool.clone();
    }
}
