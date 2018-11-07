use crate::prelude::*;
use crate::rendering::Renderer;

pub struct View {
    pub x0: u32,
    pub y0: u32,
    pub x1: u32,
    pub y1: u32,
}

impl Default for View {
    fn default() -> Self {
        Self {
            x0: 0,
            y0: 0,
            x1: 0,
            y1: 0,
        }
    }
}

pub struct Canvas {
    scale_w: u32,
    scale_h: u32,

    canvas_w: u32,
    canvas_h: u32,

    art_w: u32,
    art_h: u32,

    view: View,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            scale_w: 0,
            scale_h: 0,

            canvas_w: 0,
            canvas_h: 0,

            art_w: 0,
            art_h: 0,

            view: View::default(),
        }
    }
}

impl Canvas {
    pub fn new(art_w: u32, art_h: u32) -> Self {
        let mut ret = Self::default();
        ret.art_w = art_w;
        ret.art_h = art_h;
        ret.view.x1 = art_w;
        ret.view.y1 = art_h;
        ret
    }

    pub fn update(&mut self, canvas_w: u32, canvas_h: u32) {
        let scale_w =  canvas_w / self.art_w;
        let scale_h = canvas_h / self.art_h;

        self.scale_w = scale_w;
        self.scale_h = scale_h;
        self.canvas_w = canvas_w;
        self.canvas_h = canvas_h;
        self.art_w = self.art_w;
        self.art_h = self.art_h;
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
        unimplemented!()
        // if x >= self.art_w { return; }
        // if y >= self.art_h { return; }

        // if !self.is_in_view(x, y) { return; }

        // self.renderer.set_fill_style_color(color);

        // let scale_w = self.canvas_w / (self.view.x1 - self.view.x0);
        // let scale_h = self.canvas_h / (self.view.y1 - self.view.y0);

        // let x = (x-self.view.x0) * scale_w;
        // let y = (y-self.view.y0) * scale_h;

        // self.renderer.fill_rect(
        //     f64::from(x),
        //     f64::from(y),
        //     f64::from(scale_w),
        //     f64::from(scale_h),
        // );
    }

    pub fn clear_all(&self) {
        unimplemented!()
        // self.renderer.set_fill_style_color("white");
        // self.renderer.fill_rect(
        //     0.0,
        //     0.0,
        //     f64::from(self.art_w * self.scale_w),
        //     f64::from(self.art_h * self.scale_h),
        // );
    }

    /// same as client_to_grid but for f32
    pub fn shrink_size(&self, cli_x: f32, cli_y: f32) -> Point2D<f32> {
        let scale_w = self.canvas_w / (self.view.x1 - self.view.x0);
        let scale_h = self.canvas_h / (self.view.y1 - self.view.y0);

        let x = cli_x / scale_w as f32 + self.view.x0 as f32;
        let y = cli_y / scale_h as f32 + self.view.y0 as f32;

        Point2D::new(x, y)
    }

    pub fn client_to_grid(&self, p: Point2D<i32>) -> Point2D<u32> {
        let Point2D {x: cli_x, y: cli_y} = p;
        let scale_w = self.canvas_w / (self.view.x1 - self.view.x0);
        let scale_h = self.canvas_h / (self.view.y1 - self.view.y0);

        let x = cli_x as u32 / scale_w + self.view.x0;
        let y = cli_y as u32 / scale_h + self.view.y0;

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
