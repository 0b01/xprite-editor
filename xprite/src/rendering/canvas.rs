use crate::prelude::*;
use crate::rendering::Renderer;

#[derive(Clone, Copy, Debug)]
pub struct Scroll {
    pub x: f64,
    pub y: f64,
}

impl Default for Scroll {
    fn default() -> Self {
        Self { x: 0., y: 0. }
    }
}

#[derive(Debug)]
pub struct Canvas {
    pub bg: Color,
    pub scale: f64,
    pub win_x: f64,
    pub win_y: f64,
    pub canvas_w: f64,
    pub canvas_h: f64,
    pub art_w: f64,
    pub art_h: f64,
    pub scroll: Scroll,
    pub show_grid: bool,
    pub initialized: bool,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
            bg: Color::black(),
            scale: 10.,

            win_x: 0.,
            win_y: 0.,

            canvas_w: 0.,
            canvas_h: 0.,

            art_w: 0.,
            art_h: 0.,

            scroll: Scroll::default(),
            show_grid: false,

            initialized: false,
        }
    }
}

impl Canvas {
    pub fn new(art_w: f64, art_h: f64) -> Self {
        Self {
            art_w,
            art_h,
            ..Default::default()
        }
    }

    pub fn get_art_dimension(&self) -> (f64, f64) {
        (self.art_w, self.art_h)
    }

    pub fn update_pos(&mut self, win_x: f64, win_y: f64) {
        self.win_x = win_x;
        self.win_y = win_y;
    }

    pub fn update_sz(&mut self, canvas_w: f64, canvas_h: f64) {
        self.canvas_w = canvas_w;
        self.canvas_h = canvas_h;
    }

    pub fn draw_circle(&self, rdr: &mut dyn Renderer, p0: Vec2f, radius: f64, color: [f32; 4], filled: bool) {
        let p0 = self.to_cli(p0).into();
        let rad = self.scale * radius;
        rdr.circ(p0, rad, color, filled);
    }

    pub fn draw_bezier(&self, rdr: &mut dyn Renderer, from: Vec2f, ctrl1: Vec2f, ctrl2: Vec2f, to: Vec2f, c: [f32; 4], thickness: f64) {
        let p0 = self.to_cli(from).into();
        let p1 = self.to_cli(to).into();
        let cp0 = self.to_cli(ctrl1).into();
        let cp1 = self.to_cli(ctrl2).into();
        rdr.bezier(p0, cp0, cp1, p1, c, thickness);
    }

    pub fn to_cli(&self, p: Vec2f) -> Vec2f {
        let o = self.origin();
        Vec2f {
            x: o.x + self.scale * p.x,
            y: o.y + self.scale * p.y,
        }
    }

    pub fn within_circle(&self, point: Vec2f, mouse: Vec2f) -> bool {
        let radius = 2.;
        let Vec2f { x, y } = point;
        let o = self.origin();
        let p0 = Vec2f {
            x: o.x + self.scale * x,
            y: o.y + self.scale * y,
        };
        let rad = self.scale * radius;

        mouse.x < p0.x + rad && mouse.x > p0.x - rad && mouse.y < p0.y + rad && mouse.y > p0.y - rad
    }

    /// draw line around pixel
    pub fn draw_pixel_outline(&self, rdr: &mut dyn Renderer, p: Vec2f, outline: Outline) {
        let Vec2f { x, y } = p;
        let o = self.origin();
        if oob(x, y, self.art_w, self.art_h) {
            return;
        }
        // top left
        let p0 = [o.x + self.scale * x, o.y + self.scale * y];
        // top right
        let p1 = [o.x + self.scale * (x + 1.), o.y + self.scale * y];
        let p2 = [o.x + self.scale * (x + 1.), o.y + self.scale * (y + 1.)];
        let p3 = [o.x + self.scale * x, o.y + self.scale * (y + 1.)];

        let color = XpriteRgba::red().into();

        if outline.contains(Outline::TOP) {
            rdr.line(p0, p1, color);
        }
        if outline.contains(Outline::BOTTOM) {
            rdr.line(p3, p2, color);
        }
        if outline.contains(Outline::LEFT) {
            rdr.line(p0, p3, color);
        }
        if outline.contains(Outline::RIGHT) {
            rdr.line(p1, p2, color);
        }
    }

    /// draw an outlined pixel
    pub fn draw_pixel_marqee(&self, rdr: &mut dyn Renderer, p: Vec2f, outline: Outline, ith: usize) {
        let Vec2f { x, y } = p;
        let o = self.origin();
        if oob(x, y, self.art_w, self.art_h) {
            return;
        }
        // top left
        let p0 = [o.x + self.scale * x, o.y + self.scale * y];
        // top right
        let p1 = [o.x + self.scale * (x + 1.), o.y + self.scale * y];
        let p2 = [o.x + self.scale * (x + 1.), o.y + self.scale * (y + 1.)];
        let p3 = [o.x + self.scale * x, o.y + self.scale * (y + 1.)];

        let t = rdr.time() % 1.;
        let color =
            if (t < 0.25 && ith % 4 == 0) || (t > 0.25 && t < 0.50 && ith % 4 == 1) || (t > 0.50 && t < 0.75 && ith % 4 == 2) || (t > 0.75 && ith % 4 == 3) {
                XpriteRgba::white().into()
            } else {
                XpriteRgba::black().into()
            };

        if outline.contains(Outline::TOP) {
            rdr.line(p0, p1, color);
        }
        if outline.contains(Outline::BOTTOM) {
            rdr.line(p3, p2, color);
        }
        if outline.contains(Outline::LEFT) {
            rdr.line(p0, p3, color);
        }
        if outline.contains(Outline::RIGHT) {
            rdr.line(p1, p2, color);
        }
    }

    /// draw a rectangular pixel using draw list(as opposed to rendering to texture)
    pub fn draw_pixel_rect(&self, rdr: &mut dyn Renderer, p: Vec2f, color: [f32; 4], filled: bool) {
        let Vec2f { x, y } = p;
        let o = self.origin();
        if oob(x, y, self.art_w, self.art_h) {
            return;
        }
        let p0 = [o.x + self.scale * x, o.y + self.scale * y];
        let p1 = [o.x + self.scale * (x + 1.), o.y + self.scale * (y + 1.)];

        rdr.rect(p0, p1, color, filled);
    }

    pub fn origin(&self) -> Vec2f {
        Vec2f {
            x: self.win_x + self.scroll.x,
            y: self.win_y + self.scroll.y,
        }
    }

    pub fn draw_grid(&self, rdr: &mut dyn Renderer) {
        if !self.show_grid {
            return;
        }
        let o = self.origin();

        let color = XpriteRgba::black().into();
        let mut x = 0.;
        while x < self.scale * self.art_w {
            rdr.line([o.x + x, o.y], [o.x + x, o.y + self.scale * self.art_h], color);
            x += self.scale;
        }

        let mut y = 0.;
        while y < self.scale * self.art_h {
            rdr.line([o.x, o.y + y], [o.x + self.scale * self.art_w, o.y + y], color);
            y += self.scale;
        }
    }

    pub fn draw_line(&self, rdr: &mut dyn Renderer, p0: Vec2f, p1: Vec2f, c: [f32; 4]) {
        let p0 = self.to_cli(p0).into();
        let p1 = self.to_cli(p1).into();

        rdr.line(p0, p1, c);
    }

    pub fn update_zoom(&mut self, wheel_delta: f64, (cursor_x, cursor_y): (f64, f64)) {
        if wheel_delta == 0. {
            return;
        }
        let mut new_scale = wheel_delta + self.scale;
        if new_scale < 0.33 {
            new_scale = 0.33;
        } else if new_scale > 10. {
            new_scale = 10.;
        }
        let ratio_x = (cursor_x - self.win_x - self.scroll.x) / (self.scale * self.art_w);
        let ratio_y = (cursor_y - self.win_y - self.scroll.y) / (self.scale * self.art_h);

        self.scroll.x = cursor_x - ratio_x * (new_scale * self.art_w) - self.win_x;
        self.scroll.y = cursor_y - ratio_y * (new_scale * self.art_h) - self.win_y;
        self.scale = new_scale;
    }

    /// convert screen pos to pixel location
    pub fn shrink_size_no_floor(&self, p: Vec2f) -> Vec2f {
        let Vec2f { x: cli_x, y: cli_y } = p;
        let o = self.origin();
        let x = (cli_x - o.x) / self.scale;
        let y = (cli_y - o.y) / self.scale;
        Vec2f { x, y }
    }

    /// convert screen pos to pixel location
    pub fn shrink_size(&self, p: Vec2f) -> Vec2f {
        let Vec2f { x: cli_x, y: cli_y } = p;
        let o = self.origin();
        let x = ((cli_x - o.x) / self.scale).floor();
        let y = ((cli_y - o.y) / self.scale).floor();
        Vec2f { x, y }
    }

    /// snap point to grid
    pub fn snap(p: Vec2f) -> Vec2f {
        let Vec2f { x: cli_x, y: cli_y } = p;
        Vec2f {
            x: cli_x.floor(),
            y: cli_y.floor(),
        }
    }

    pub fn get_aspect_ratio(&self) -> f64 {
        self.art_w / self.art_h
    }

    /// returns a fraction for simplified aspect ratio
    pub fn get_aspect_ratio_human(&self) -> (u32, u32) {
        let w = self.art_w as u32;
        let h = self.art_h as u32;
        // TODO:
        (w, h)
    }
}
