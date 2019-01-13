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

    fn circ(&mut self, p0:[f32;2], r:f32, color:[f32;4], filled: bool) {
        let x = f64::from(p0[0]); let y = f64::from(p0[1]);
        if filled {
            self.set_fill_style_color(
                &format!("rgba({},{},{},{})",
                    color[0] * 255.,
                    color[1] * 255.,
                    color[2] * 255.,
                    color[3],
                ));
            self.ctx.begin_path();
            self.ctx.arc(x, y, r as f64, 0., 2. * 3.141592653, false);
            self.ctx.fill(Default::default());
        } else {
            self.ctx.begin_path();
            self.ctx.arc(x, y, r as f64, 0., 2. * 3.141592653, false);
            self.ctx.stroke();
        }

    }

    fn line(&mut self, p0:[f32;2], p1:[f32;2], color:[f32;4]) {
        self.set_fill_style_color(
            &format!("rgba({},{},{},{})",
                color[0] * 255.,
                color[1] * 255.,
                color[2] * 255.,
                color[3],
            ));
        self.ctx.begin_path();
        self.ctx.move_to(p0[0] as f64, p0[1] as f64);
        self.ctx.line_to(p1[0] as f64, p1[1] as f64);
        self.ctx.stroke();
    }

    fn bezier(&mut self, p0:[f32;2], cp1:[f32;2], cp2: [f32;2], p1:[f32;2],
        color:[f32;4], thickness: f32
    ) {
        self.set_fill_style_color(
            &format!("rgba({},{},{},{})",
                color[0] * 255.,
                color[1] * 255.,
                color[2] * 255.,
                color[3],
            ));
        self.ctx.begin_path();
        self.ctx.move_to(p0[0] as f64, p0[1] as f64);
        self.ctx.bezier_curve_to(
            cp1[0] as f64,
            cp1[1] as f64,
            cp2[0] as f64,
            cp2[1] as f64,
            p1[0] as f64,
            p1[1] as f64,
        );
        self.ctx.stroke();

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

    pub fn set_fill_style_color(&self, color: &str) {
        self.ctx.set_fill_style_color(color)
    }
}