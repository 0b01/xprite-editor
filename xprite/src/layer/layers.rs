use crate::prelude::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Layers {
    pub layers: Vec<Layer>,
    pub selected: usize,
}

impl Default for Layers {
    fn default() -> Self {
        Self::new()
    }
}

impl Layers {
    pub fn new() -> Self {
        let layers = vec![Layer::new("Layer 0".to_owned())];
        let selected = 0;
        Self {
            layers,
            selected,
        }
    }

    pub fn selected_layer(&self) -> Option<&Layer> {
        self.layers.get(self.selected)
    }

    pub fn selected_layer_mut(&mut self) -> Option<&mut Layer> {
        self.layers.get_mut(self.selected)
    }


    pub fn add(&mut self, name: Option<&str>) {
        let name = name
                .and_then(|i: &str| Some(i.to_owned()))
                .unwrap_or_else(||
                    format!("Layer {}", self.layers.len())
                );
        let layer = Layer::new(name);
        self.layers.push(layer);
    }

    pub fn duplicate_current(&mut self) {
        let selected = self.selected_layer().unwrap();
        let new_layer = selected.clone();
        self.layers.push(new_layer);
    }

    pub fn remove_layer(&mut self, to_remove: usize) {
        // let i = self.layers.iter().find(|&i| i == to_remove).unwrap();
        self.layers.remove(to_remove);
    }
}
