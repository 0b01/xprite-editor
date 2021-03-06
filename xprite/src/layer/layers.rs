use crate::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Layers {
    pub groups: Vec<(String, Vec<Rc<RefCell<Layer>>>)>,
    pub layer_idx: usize,
    pub group_idx: usize,
}

impl Clone for Layers {
    fn clone(&self) -> Self {
        let mut groups = self.groups.clone();
        for v in groups.iter_mut() {
            for l in v.1.iter_mut() {
                let layer = l.borrow();
                let new_layer = layer.clone();
                drop(layer);
                *l = Rc::new(RefCell::new(new_layer));
            }
        }
        Self {
            groups,
            layer_idx: self.layer_idx,
            group_idx: self.group_idx,
        }
    }
}

impl Default for Layers {
    fn default() -> Self {
        Self::new()
    }
}

impl Layers {
    pub fn new() -> Self {
        let groups = vec![("Group 1".to_owned(), vec![Rc::new(RefCell::new(Layer::new("Layer 0".to_owned())))])];
        let selected = 0;
        let sel_group = 0;
        Self {
            groups,
            layer_idx: selected,
            group_idx: sel_group,
        }
    }

    pub fn layer(&self) -> Option<Rc<RefCell<Layer>>> {
        let gp = self.group()?;
        gp.1.get(self.layer_idx).cloned()
    }

    pub fn group(&self) -> Option<&(String, Vec<Rc<RefCell<Layer>>>)> {
        self.groups.get(self.group_idx)
    }

    pub fn group_mut(&mut self) -> Option<&mut (String, Vec<Rc<RefCell<Layer>>>)> {
        self.groups.get_mut(self.group_idx)
    }

    pub fn swap_group(&mut self, first_idx: usize, second_idx: usize) {
        self.groups.swap(first_idx, second_idx);
    }

    pub fn add_group(&mut self, name: Option<&str>) {
        let name = name
            .and_then(|i: &str| Some(i.to_owned()))
            .unwrap_or_else(|| format!("Group {}", self.groups.len() + 1));
        let new_group = (name, vec![Rc::new(RefCell::new(Layer::new("Layer 1".to_owned())))]);
        self.groups.push(new_group);
    }

    pub fn insert_layer(&mut self, name: Option<&str>, visible: bool, idx: usize) {
        let name = name.map(|i| i.to_owned()).unwrap_or_else(|| format!("Layer {}", self.group().unwrap().1.len()));
        let mut new_layer = Layer::new(name);
        new_layer.visible = visible;

        self.group_mut().unwrap().1.insert(idx, Rc::new(RefCell::new(new_layer)));
    }

    pub fn add_layer(&mut self, name: Option<&str>, visible: bool) {
        let idx = self.group_mut().unwrap().1.len();
        self.insert_layer(name, visible, idx);
    }

    pub fn swap_layer(&mut self, first_idx: usize, second_idx: usize) {
        self.group_mut().unwrap().1.swap(first_idx, second_idx);
    }

    pub fn duplicate_current(&mut self) {
        let selected = self.layer().unwrap();
        let new_layer = selected.clone();
        self.group_mut().unwrap().1.push(new_layer);
    }

    pub fn remove_layer(&mut self, group: usize, to_remove: usize) {
        self.groups[group].1.remove(to_remove);
    }

    pub fn toggle_group_visibility(&mut self, group: usize) -> Option<()> {
        for layer in &self.groups.get_mut(group)?.1 {
            layer.borrow_mut().toggle_visible();
        }
        Some(())
    }

    pub fn iter_layers(&self) -> impl DoubleEndedIterator<Item = &Rc<RefCell<Layer>>> {
        self.groups.iter().map(|g| g.1.iter()).flatten()
    }
}
