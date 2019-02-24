use crate::prelude::*;

use std::cell::RefCell;
use std::rc::Rc;

use crate::tools::{
    autoshade::AutoShade, color_picker::ColorPicker, ellipse::Ellipse, eraser::Eraser, line::Line, marquee::Marquee, paint_bucket::PaintBucket, pencil::Pencil,
    rect::Rect, symmetry::Symmetry, texture::Texture, vector::Vector, Tool,
};

#[derive(Default)]
pub struct Toolbox {
    pub pencil: Rc<RefCell<Pencil>>,
    pub paint_bucket: Rc<RefCell<PaintBucket>>,
    pub line: Rc<RefCell<Line>>,
    pub vector: Rc<RefCell<Vector>>,
    pub color_picker: Rc<RefCell<ColorPicker>>,
    pub eraser: Rc<RefCell<Eraser>>,
    pub rect: Rc<RefCell<Rect>>,
    pub texture: Rc<RefCell<Texture>>,
    pub ellipse: Rc<RefCell<Ellipse>>,
    pub marquee: Rc<RefCell<Marquee>>,
    pub symmetry: Rc<RefCell<Symmetry>>,
    pub autoshade: Rc<RefCell<AutoShade>>,

    pub selected: ToolType,
    pub tool_stack: Vec<ToolType>,
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
        let ellipse = Rc::new(RefCell::new(Ellipse::new()));
        let marquee = Rc::new(RefCell::new(Marquee::new()));
        let symmetry = Rc::new(RefCell::new(Symmetry::new()));
        let autoshade = Rc::new(RefCell::new(AutoShade::new()));

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
            ellipse,
            marquee,
            symmetry,
            autoshade,
        }
    }

    pub fn tool(&mut self) -> Rc<RefCell<Tool>> {
        self.get(&self.selected)
    }

    pub fn get(&self, name: &ToolType) -> Rc<RefCell<Tool>> {
        use self::ToolType::*;
        match name {
            Pencil | Settings => self.pencil.clone(),
            Line => self.line.clone(),
            PaintBucket => self.paint_bucket.clone(),
            Vector => self.vector.clone(),
            ColorPicker => self.color_picker.clone(),
            Eraser => self.eraser.clone(),
            Rect | FilledRect => self.rect.clone(),
            Ellipse | FilledEllipse => self.ellipse.clone(),
            Texture => self.texture.clone(),
            Marquee => self.marquee.clone(),
            Symmetry => self.symmetry.clone(),
            AutoShade => self.autoshade.clone(),
        }
    }

    pub fn change_tool(&mut self, tool: ToolType) {
        use self::ToolType::*;
        self.tool_stack.push(self.selected);
        let tool = match tool {
            Rect => match self.selected {
                Rect => {
                    self.rect.borrow_mut().filled = true;
                    FilledRect
                }
                FilledRect => {
                    self.rect.borrow_mut().filled = false;
                    Rect
                }
                _ => Rect,
            },
            Ellipse => match self.selected {
                Ellipse => {
                    self.ellipse.borrow_mut().filled = true;
                    FilledEllipse
                }
                FilledEllipse => {
                    self.ellipse.borrow_mut().filled = false;
                    Ellipse
                }
                _ => Ellipse,
            },
            _ => tool,
        };
        self.selected = tool.clone();
    }

    pub fn pop_tool(&mut self) {
        let tool = self.tool_stack.pop().unwrap();
        self.selected = tool.clone();
    }
}
