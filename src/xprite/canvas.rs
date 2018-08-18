use xprite::{Block, Color};

use stdweb::traits::*;
use stdweb::unstable::TryInto;
use stdweb::web::html_element::CanvasElement;
use stdweb::web::{document, CanvasRenderingContext2d};

pub struct Canvas {
    pub canvas: CanvasElement,
    pub ctx: CanvasRenderingContext2d,

    scaled_w: u32,
    scaled_h: u32,

    canvas_w: u32,
    canvas_h: u32,

    art_w: u32,
    art_h: u32,
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

        Canvas {
            canvas,
            ctx,
            scaled_w,
            scaled_h,
            canvas_w,
            canvas_h,
            art_w,
            art_h,
        }
    }

    pub fn draw(&self, x: u32, y: u32, color: &str) {
        assert!(x < self.art_w);
        assert!(y < self.art_h);

        self.ctx.set_fill_style_color(color);

        let x = x * self.scaled_w;
        let y = y * self.scaled_h;

        self.ctx.fill_rect(
            f64::from(x),
            f64::from(y),
            f64::from(self.scaled_w),
            f64::from(self.scaled_h),
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

    pub fn to_block(&self, cli_x: i32, cli_y: i32, color: Color) -> Option<Block> {
        let x = cli_x as u32 / self.scaled_w;
        let y = cli_y as u32 / self.scaled_h;

        if x >= self.art_w || y >= self.art_h {
            None
        } else {
            Some(Block {
                x, y, color,
            })
        }
    }
}
