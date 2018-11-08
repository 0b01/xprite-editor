use crate::prelude::*;
use crate::rendering::Renderer;

pub struct View {
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
}

impl Default for View {
    fn default() -> Self {
        Self {
            x0: 0.,
            y0: 0.,
            x1: 0.,
            y1: 0.,
        }
    }
}

pub struct Canvas {
    pub scale_w: f32,
    pub scale_h: f32,
    pub win_x: f32,
    pub win_y: f32,
    pub canvas_w: f32,
    pub canvas_h: f32,
    pub art_w: f32,
    pub art_h: f32,
    pub view: View,
    pub show_grid: bool,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            scale_w: 0.,
            scale_h: 0.,

            win_x: 0.,
            win_y: 0.,

            canvas_w: 0.,
            canvas_h: 0.,

            art_w: 0.,
            art_h: 0.,

            view: View::default(),
            show_grid: false,
        }
    }
}

impl Canvas {
    pub fn new(art_w: f32, art_h: f32) -> Self {
        let mut ret = Self::default();
        ret.art_w = art_w;
        ret.art_h = art_h;
        ret.view.x1 = art_w;
        ret.view.y1 = art_h;
        ret
    }

    pub fn update_pos(&mut self, win_x: f32, win_y: f32) {
        self.win_x = win_x;
        self.win_y = win_y;
    }

    pub fn update_sz(&mut self, canvas_w: f32, canvas_h: f32) {
        if self.canvas_w == canvas_w && self.canvas_h == canvas_h { return; }
        println!("Updating canvas size");

        let scale_w =  canvas_w / self.art_w;
        let scale_h = canvas_h / self.art_h;

        self.scale_w = scale_w;
        self.scale_h = scale_h;
        self.canvas_w = canvas_w;
        self.canvas_h = canvas_h;
        self.art_w = self.art_w;
        self.art_h = self.art_h;
    }

    fn is_in_view(&self, x: f32, y: f32) -> bool {
        x >= self.view.x0 &&
        x <= self.view.x1 &&
        y >= self.view.y0 &&
        y <= self.view.y1
    }

    pub fn zoom_in(&mut self, d: f32) {
        if self.view.x1 - self.view.x0 < 2.*d
         ||self.view.y1 - self.view.y0 < 2.*d {
             return;
         }
        self.view.x0 += d;
        self.view.y0 += d;
        self.view.x1 -= d;
        self.view.y1 -= d;
    }

    pub fn zoom_in_at(&mut self, d: f32, point: Point2D<f32>) {
        let x = point.x;
        let y = point.y;
        if self.view.x1 - self.view.x0 < 2.*d
         ||self.view.y1 - self.view.y0 < 2.*d {
             return;
        }

        let w0 = ((x-self.view.x0) / (self.view.x1 - self.view.x0)) * d;
        let w1 = d - w0;
        let h0 = ((y-self.view.y0) / (self.view.y1 - self.view.y0)) * d;
        let h1 = d - h0;

        self.view.x0 += w0;
        self.view.y0 += h0;
        self.view.x1 -= w1;
        self.view.y1 -= h1;
    }

    pub fn zoom_out(&mut self, d: f32) {
        if self.view.x0 <= d { self.view.x0 = 0.; }
        else { self.view.x0 -= d; }

        if self.view.y0 <= d { self.view.y0 = 0.; }
        else { self.view.y0 -= d; }

        if self.view.x1 >= self.art_w { self.view.x1 = self.art_w; }
        else { self.view.x1 += d; }

        if self.view.y1 >= self.art_h { self.view.y1 = self.art_h; }
        else { self.view.y1 += d; }
    }


    pub fn draw(&self, rdr: &Renderer, x: f32, y: f32, color: &str) {
        if x >= self.art_w { return; }
        if y >= self.art_h { return; }

        if !self.is_in_view(x, y) { return; }

        rdr.set_fill_style_color(color);

        let scale_w = self.canvas_w / (self.view.x1 - self.view.x0);
        let scale_h = self.canvas_h / (self.view.y1 - self.view.y0);

        let x = (x-self.view.x0) * scale_w;
        let y = (y-self.view.y0) * scale_h;

        rdr.fill_rect(
            x,
            y,
            scale_w,
            scale_h,
        );
    }

    pub fn clear_all(&self, rdr: &Renderer) {
        rdr.set_fill_style_color("white");
        rdr.fill_rect(
            self.win_x + 0.0 - 10.,
            self.win_y + 0.0 - 10.,
            self.win_x + self.art_w * self.scale_w - 10.,
            self.win_y + self.art_h * self.scale_h - 10.,
        );
    }

    /// same as client_to_grid but for f32
    pub fn shrink_size(&self, p: &Point2D<f32>) -> Point2D<f32> {
        let Point2D {x: cli_x , y: cli_y} = p;
        let scale_w = self.canvas_w / (self.view.x1 - self.view.x0);
        let scale_h = self.canvas_h / (self.view.y1 - self.view.y0);

        let x = cli_x / scale_w as f32 + self.view.x0 as f32;
        let y = cli_y / scale_h as f32 + self.view.y0 as f32;

        Point2D::new(x, y)
    }

    pub fn to_pixels(&self, p: Point2D<f32>, brush: &Brush, color: Color) -> Option<Pixels> {
        let Point2D {x, y} = self.shrink_size(&p);

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
