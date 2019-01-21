use std::f64;
use xprite::rendering::Renderer;

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

pub struct StdwebRenderer {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,
    pub last: Option<[f32;4]>,
}

#[allow(unused)]
impl Renderer for StdwebRenderer {
    fn reset(&mut self) {
        self.ctx
            .clear_rect(0., 0., self.width() as f64, self.height() as f64);
    }

    fn width(&self) -> f64 {
        self.canvas.width() as f64
    }

    fn height(&self) -> f64 {
        self.canvas.height() as f64
    }

    fn rect(&mut self, p0: [f64; 2], p1: [f64; 2], color: [f32; 4], filled: bool) {
        let a = f64::from(p0[0]);
        let b = f64::from(p0[1]);
        let c = f64::from(p1[0]) - a;
        let d = f64::from(p1[1]) - b;
        if filled {
            if self.last.is_none() {
                let col = format!(
                    "rgba({},{},{},{})",
                    color[0] * 255.,
                    color[1] * 255.,
                    color[2] * 255.,
                    color[3],
                );
                self.ctx.set_fill_style_color(&col);
                self.last = Some(color);
            } else if let Some(last) = self.last.as_ref() {
                let col = format!(
                    "rgba({},{},{},{})",
                    color[0] * 255.,
                    color[1] * 255.,
                    color[2] * 255.,
                    color[3],
                );
                if last != &color {
                    self.ctx.set_fill_style_color(&col);
                    self.last = Some(color);
                }
            }
            self.ctx.fill_rect(a, b, c, d)
        } else {
            self.ctx.rect(a, b, c, d);
            self.ctx.stroke();
        }
    }

    fn pixel(&mut self, x: f64, y: f64, color: [f32; 4], filled: bool) {
        ()
    }

    fn circ(&mut self, p0: [f64; 2], r: f64, color: [f32; 4], filled: bool) {
        let x = f64::from(p0[0]);
        let y = f64::from(p0[1]);
        if filled {
            self.ctx.set_fill_style_color(&format!(
                "rgba({},{},{},{})",
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

    fn line(&mut self, p0: [f64; 2], p1: [f64; 2], color: [f32; 4]) {
        self.ctx.set_fill_style_color(&format!(
            "rgba({},{},{},{})",
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

    fn bezier(
        &mut self,
        p0: [f64; 2],
        cp1: [f64; 2],
        cp2: [f64; 2],
        p1: [f64; 2],
        color: [f32; 4],
        thickness: f64,
    ) {
        self.ctx.set_fill_style_color(&format!(
            "rgba({},{},{},{})",
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
        Self { canvas, ctx, last: None }
    }
}
