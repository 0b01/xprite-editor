use crate::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;

use crate::tools::{
    Tool,
    pencil::Pencil,
    line::Line,
    paint_bucket::PaintBucket,
    vector::Vector,
};

pub struct Toolbox {
    pub pencil:         Rc<RefCell<Pencil>>,
    pub paint_bucket:   Rc<RefCell<PaintBucket>>,
    pub line:           Rc<RefCell<Line>>,
    pub vector:         Rc<RefCell<Vector>>,
    pub selected:       ToolType,
}

impl Toolbox {
    pub fn new() -> Self {
        let pencil = Rc::new(RefCell::new(Pencil::new()));
        let line = Rc::new(RefCell::new(Line::new()));
        let paint_bucket = Rc::new(RefCell::new(PaintBucket::new()));
        let vector = Rc::new(RefCell::new(Vector::new()));

        let selected = ToolType::Pencil;
        Toolbox {
            pencil,
            line,
            paint_bucket,
            selected,
            vector,
        }
    }

    pub fn tool(&mut self) -> Rc<RefCell<Tool>> {
        self.get(&self.selected)
    }

    pub fn get(&self, name: &ToolType) -> Rc<RefCell<Tool>> {
        use self::ToolType::*;
        match name {
            Pencil => self.pencil.clone(),
            Line => self.line.clone(),
            PaintBucket => self.paint_bucket.clone(),
            Vector => self.vector.clone(),
        }
    }

    pub fn change_tool(&mut self, tool: &ToolType) {
        self.selected = tool.clone();
    }
}
