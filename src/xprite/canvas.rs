use xprite::{Blocks, Block, Color, Brush};

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

#[derive(Debug)]
pub struct View {
    pub x0: u32,
    pub y0: u32,
    pub x1: u32,
    pub y1: u32,
}

pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,

    scaled_w: u32,
    scaled_h: u32,

    canvas_w: u32,
    canvas_h: u32,

    art_w: u32,
    art_h: u32,

    view: View,
}

impl Canvas {
    pub fn new(attr_id: &str, art_w: u32, art_h: u32) -> Canvas {
        let canvas: CanvasElement = document()
            .query_selector(attr_id)
            .unwrap()
            .unwrap()
            .try_into()
            .unwrap();

        let ctx: CanvasRenderingContext2d = canvas.get_context().unwrap();

        let canvas_w = canvas.width();
        let canvas_h = canvas.height();

        let scaled_w =  canvas_w / art_w;
        let scaled_h = canvas_h / art_h;

        let view = View {
            x0: 0,
            y0: 0,
            x1: art_w,
            y1: art_h,
        };

        Canvas {
            canvas,
            ctx,
            scaled_w,
            scaled_h,
            canvas_w,
            canvas_h,
            art_w,
            art_h,
            view,
        }
    }

    fn is_in_view(&self, x: u32, y: u32) -> bool {
        x >= self.view.x0 &&
        x <= self.view.x1 &&
        y >= self.view.y0 &&
        y <= self.view.y1
    }

    pub fn zoom_in(&mut self, d: u32) {
        self.view.x0 += d;
        self.view.y0 += d;
        self.view.x1 -= d;
        self.view.y1 -= d;

        console!(log,
            self.view.x0,
            self.view.y0,
            self.view.x1,
            self.view.y1
        );
    }

    pub fn zoom_in_at(&mut self, d: u32, x: u32, y: u32) {
        let w0 = ((x-self.view.x0) as f32 / (self.view.x1 - self.view.x0) as f32) * d as f32;
        let w1 = d as f32 - w0;
        let h0 = ((y-self.view.y0) as f32 / (self.view.y1 - self.view.y0) as f32) * d as f32;
        let h1 = d  as f32- h0;

        self.view.x0 += w0 as u32;
        self.view.y0 += h0 as u32;
        self.view.x1 -= w1 as u32;
        self.view.y1 -= h1 as u32;

        console!(log,
            self.view.x0,
            self.view.y0,
            self.view.x1,
            self.view.y1
        );
    }

    pub fn zoom_out(&mut self, d: u32) {
        if self.view.x0 <= d { self.view.x0 = 0; }
        else { self.view.x0 -= d; }

        if self.view.y0 <= d { self.view.y0 = 0; }
        else { self.view.y0 -= d; }

        if self.view.x1 >= self.art_w { self.view.x1 = self.art_w; }
        else { self.view.x1 += d; }

        if self.view.y1 >= self.art_h { self.view.y1 = self.art_h; }
        else { self.view.y1 += d; }

        console!(log,
            self.view.x0,
            self.view.y0,
            self.view.x1,
            self.view.y1
        );
    }


    pub fn draw(&self, x: u32, y: u32, color: &str) {
        assert!(x < self.art_w);
        assert!(y < self.art_h);

        if !self.is_in_view(x, y) { return; }

        self.ctx.set_fill_style_color(color);

        let scaled_w = self.canvas_w / (self.view.x1 - self.view.x0);
        let scaled_h = self.canvas_h / (self.view.y1 - self.view.y0);

        let x = (x-self.view.x0) * scaled_w;
        let y = (y-self.view.y0) * scaled_h;

        self.ctx.fill_rect(
            f64::from(x),
            f64::from(y),
            f64::from(scaled_w),
            f64::from(scaled_h),
        );
    }

    pub fn clear_all(&self) {
        self.ctx.set_fill_style_color("white");
        self.ctx.fill_rect(
            0.0,
            0.0,
            f64::from(self.art_w * self.scaled_w),
            f64::from(self.art_h * self.scaled_h),
        );
    }

    pub fn get_cursor(&self, cli_x: i32, cli_y: i32) -> (u32, u32) {
        let scaled_w = self.canvas_w / (self.view.x1 - self.view.x0);
        let scaled_h = self.canvas_h / (self.view.y1 - self.view.y0);

        let x = cli_x as u32 / scaled_w + self.view.x0;
        let y = cli_y as u32 / scaled_h + self.view.y0;

        (x, y)
    }

    pub fn to_blocks(&self, cli_x: i32, cli_y: i32, brush: &Brush, color: Color) -> Option<Blocks> {
        let (x, y) = self.get_cursor(cli_x, cli_y);

        let (brush_w, brush_h) = brush.size;

        if (x + brush_w) >= self.art_w || (y + brush_h) >= self.art_h {
            None
        } else {
            let ret = brush.shape.iter().map(
                |Block {x:dx, y:dy,..}| Block {x: x+dx, y: y+dy, color}
            ).collect();
            Some(ret)
        }
    }
}
