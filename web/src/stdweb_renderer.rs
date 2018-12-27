use xprite::rendering::Renderer;
use std::f64;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

pub struct StdwebRenderer {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
}

#[allow(unused)]
impl Renderer for StdwebRenderer {

    fn reset(&mut self) {
        self.ctx.clear_rect(0., 0., self.width() as f64, self.height() as f64);
    }

    fn width(&self) -> f32 {
        self.canvas.width() as f32
    }
    fn height(&self) -> f32 {
        self.canvas.height() as f32
    }
    fn rect(&mut self, p0:[f32;2], p1:[f32;2], color:[f32;4], filled: bool) {
        let a = f64::from(p0[0]); let b = f64::from(p0[1]);
        let c = f64::from(p1[0]) - a; let d = f64::from(p1[1]) - b;
        if filled {
            self.set_fill_style_color(
                &format!("rgba({},{},{},{})",
                    color[0] * 255.,
                    color[1] * 255.,
                    color[2] * 255.,
                    color[3],
                ));
            self.ctx.fill_rect(a,b,c,d)
        } else {
            self.ctx.rect(a,b,c,d);
            self.ctx.stroke();
        }
    }
    fn circ(&mut self, p0:[f32;2], r:f32, color:[f32;4], filled: bool) {}
    fn line(&mut self, p0:[f32;2], p1:[f32;2], color:[f32;4]) {}
    fn bezier(&mut self, p0:[f32;2], cp1:[f32;2], cp2: [f32;2], p1:[f32;2],
        color:[f32;4], thickness: f32) {}
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

    pub fn set_fill_style_color(&self, color: &str) {
        self.ctx.set_fill_style_color(color)
    }
}