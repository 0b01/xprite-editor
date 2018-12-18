use crate::prelude::*;

use std::rc::Rc;
use std::cell::RefCell;

use crate::tools::{
    Tool,
    pencil::Pencil,
    line::Line,
    paint_bucket::PaintBucket,
    vector::Vector,
    color_picker::ColorPicker,
    eraser::Eraser,
    rect::Rect,
    texture::Texture,
};

pub struct Toolbox {
    pub pencil:         Rc<RefCell<Pencil>>,
    pub paint_bucket:   Rc<RefCell<PaintBucket>>,
    pub line:           Rc<RefCell<Line>>,
    pub vector:         Rc<RefCell<Vector>>,
    pub color_picker:   Rc<RefCell<ColorPicker>>,
    pub eraser:         Rc<RefCell<Eraser>>,
    pub rect:           Rc<RefCell<Rect>>,
    pub texture:        Rc<RefCell<Texture>>,

    pub selected:       ToolType,

    pub tool_stack:     Vec<ToolType>,
}

impl Toolbox {
    pub fn new() -> Self {
        let pencil = Rc::new(RefCell::new(Pencil::new()));
        let line = Rc::new(RefCell::new(Line::new()));
        let paint_bucket = Rc::new(RefCell::new(PaintBucket::new()));
        let vector = Rc::new(RefCell::new(Vector::new()));
        let color_picker = Rc::new(RefCell::new(ColorPicker::new()));
        let eraser = Rc::new(RefCell::new(Eraser::new()));
        let rect = Rc::new(RefCell::new(Rect::new()));
        let texture = Rc::new(RefCell::new(Texture::new()));

        let selected = ToolType::Pencil;

        let tool_stack = Vec::with_capacity(1);
        Toolbox {
            tool_stack,
            pencil,
            line,
            paint_bucket,
            selected,
            vector,
            color_picker,
            eraser,
            rect,
            texture,
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
            ColorPicker => self.color_picker.clone(),
            Eraser => self.eraser.clone(),
            Rect | FilledRect => self.rect.clone(),
            Texture => self.texture.clone(),

        }
    }

    pub fn change_tool(&mut self, tool: &ToolType) {
        self.tool_stack.push(self.selected);
        self.selected = tool.clone();
        match tool {
            ToolType::Rect => { self.rect.borrow_mut().filled = false; }
            ToolType::FilledRect => {self.rect.borrow_mut().filled = true; }
            _ => (),
        }
    }

    pub fn pop_tool(&mut self) {
        let tool = self.tool_stack.pop().unwrap();
        self.selected = tool.clone();
    }
}
