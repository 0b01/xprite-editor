pub mod layers;

pub use self::layers::Layers;
use crate::prelude::*;
use crate::rendering::Renderer;

#[derive(PartialEq, Debug, Clone, Hash)]
pub struct Layer {
    pub name: String,
    pub content: Pixels,
    pub visible: bool,
}

impl Layer {
    pub fn new(name: String) -> Self {
        let content = Pixels::new();
        let visible = true;

        Self { name, content, visible }
    }

    pub fn with_pixels(mut self, content: &Pixels) -> Self {
        self.content = content.to_owned();
        self
    }

    pub fn pixels(&self) -> &Pixels {
        &self.content
    }

    pub fn pixels_mut(&mut self) -> &mut Pixels {
        &mut self.content
    }

    pub fn toggle_visible(&mut self) {
        self.visible = !self.visible;
        info!("toggled {} to: {}", self.name, self.visible);
    }

    pub fn get_color(&self, p: Vec2f) -> Option<Color> {
        self.content.iter().find(|i| i.point == p).map(|i| i.color)
    }

    pub fn draw(&self, rdr: &mut Renderer) {
        for &Pixel { point, color } in self.content.iter() {
            let Vec2f { x, y } = point;
            if oob(x, y, rdr.width(), rdr.height()) {
                continue;
            }
            rdr.pixel(x, y, color.into(), true);
        }
    }
}
