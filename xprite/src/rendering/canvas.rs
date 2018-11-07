use crate::prelude::*;
use crate::rendering::Renderer;

pub struct View {
    pub x0: u32,
    pub y0: u32,
    pub x1: u32,
    pub y1: u32,
}

pub struct Canvas<'a> {
    pub renderer: Box<Renderer + 'a>,

    scaled_w: u32,
    scaled_h: u32,

    canvas_w: u32,
    canvas_h: u32,

    art_w: u32,
    art_h: u32,

    view: View,
}

impl<'a> Canvas<'a> {
    pub fn new(renderer: Box<Renderer + 'a>, art_w: u32, art_h: u32) -> Canvas<'a> {
        let canvas_w = renderer.width();
        let canvas_h = renderer.height();

        let scaled_w =  canvas_w / art_w;
        let scaled_h = canvas_h / art_h;

        let view = View {
            x0: 0,
            y0: 0,
            x1: art_w,
            y1: art_h,
        };

        Canvas {
            renderer,
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
        if self.view.x1 - self.view.x0 < 2*d
         ||self.view.y1 - self.view.y0 < 2*d {
             return;
         }
        self.view.x0 += d;
        self.view.y0 += d;
        self.view.x1 -= d;
        self.view.y1 -= d;
    }

    pub fn zoom_in_at(&mut self, d: u32, point: Point2D<u32>) {
        let x = point.x;
        let y = point.y;
        if self.view.x1 - self.view.x0 < 2*d
         ||self.view.y1 - self.view.y0 < 2*d {
             return;
        }

        let w0 = ((x-self.view.x0) as f32 / (self.view.x1 - self.view.x0) as f32) * d as f32;
        let w1 = d as f32 - w0;
        let h0 = ((y-self.view.y0) as f32 / (self.view.y1 - self.view.y0) as f32) * d as f32;
        let h1 = d  as f32- h0;

        self.view.x0 += w0 as u32;
        self.view.y0 += h0 as u32;
        self.view.x1 -= w1 as u32;
        self.view.y1 -= h1 as u32;
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
    }


    pub fn draw(&self, x: u32, y: u32, color: &str) {
        if x >= self.art_w { return; }
        if y >= self.art_h { return; }

        if !self.is_in_view(x, y) { return; }

        self.renderer.set_fill_style_color(color);

        let scaled_w = self.canvas_w / (self.view.x1 - self.view.x0);
        let scaled_h = self.canvas_h / (self.view.y1 - self.view.y0);

        let x = (x-self.view.x0) * scaled_w;
        let y = (y-self.view.y0) * scaled_h;

        self.renderer.fill_rect(
            f64::from(x),
            f64::from(y),
            f64::from(scaled_w),
            f64::from(scaled_h),
        );
    }

    pub fn clear_all(&self) {
        self.renderer.set_fill_style_color("white");
        self.renderer.fill_rect(
            0.0,
            0.0,
            f64::from(self.art_w * self.scaled_w),
            f64::from(self.art_h * self.scaled_h),
        );
    }

    /// same as client_to_grid but for f32
    pub fn shrink_size(&self, cli_x: f32, cli_y: f32) -> Point2D<f32> {
        let scaled_w = self.canvas_w / (self.view.x1 - self.view.x0);
        let scaled_h = self.canvas_h / (self.view.y1 - self.view.y0);

        let x = cli_x / scaled_w as f32 + self.view.x0 as f32;
        let y = cli_y / scaled_h as f32 + self.view.y0 as f32;

        Point2D::new(x, y)
    }

    pub fn client_to_grid(&self, p: Point2D<i32>) -> Point2D<u32> {
        let Point2D {x: cli_x, y: cli_y} = p;
        let scaled_w = self.canvas_w / (self.view.x1 - self.view.x0);
        let scaled_h = self.canvas_h / (self.view.y1 - self.view.y0);

        let x = cli_x as u32 / scaled_w + self.view.x0;
        let y = cli_y as u32 / scaled_h + self.view.y0;

        Point2D::new(x, y)
    }

    pub fn to_pixels(&self, p: Point2D<i32>, brush: &Brush, color: Color) -> Option<Pixels> {
        let Point2D {x, y} = self.client_to_grid(p);

        let (brush_w, brush_h) = brush.size;

        if (x + brush_w) >= self.art_w || (y + brush_h) >= self.art_h {
            None
        } else {
            let ret = brush.shape.iter().map(
                |Pixel {point,..}| Pixel {
                    point: Point2D::new(point.x+x, point.y+y),
                    color: ColorOption::Set(color),
                }
            ).collect();
            Some(Pixels(ret))
        }
    }
}
