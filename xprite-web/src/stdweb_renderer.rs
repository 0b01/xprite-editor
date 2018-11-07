use xprite::rendering::Renderer;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

pub struct StdwebRenderer {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
}

impl Renderer for StdwebRenderer {
    fn width(&self) -> u32 {
        self.canvas.width()
    }
    fn height(&self) -> u32 {
        self.canvas.height()
    }
    fn set_fill_style_color(&self, color: &str) {
        self.ctx.set_fill_style_color(color)
    }
    fn fill_rect(&self, a: f64, b: f64, c: f64,d: f64) {
        self.ctx.fill_rect(a,b,c,d)
    }
}

impl StdwebRenderer {
    pub fn new(attr_id: &str) -> Self {
        let canvas: CanvasElement = document()
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();
        Self {
            canvas,
            ctx
        }
    }
}