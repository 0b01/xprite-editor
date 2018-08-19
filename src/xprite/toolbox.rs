use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use xprite::tools::Tool;
use xprite::tools::pencil::Pencil;

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
        let selected = pencil;

        Toolbox {
            tools,
            selected,
        }
    }

    pub fn tool(&self) -> Rc<RefCell<Tool>> {
        self.selected.clone()
    }

    pub fn change_to(&mut self, name: &str) {
        if let Some(tool) = self.tools.get(name) {
            self.selected = tool.clone();
        }
    }
}
