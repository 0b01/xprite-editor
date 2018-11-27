use crate::prelude::*;
use crate::rendering::Renderer;

#[derive(Clone, Copy)]
pub struct Scroll {
    pub x: f32,
    pub y: f32,
}

impl Default for Scroll {
    fn default() -> Self {
        Self {
            x: 0.,
            y: 0.,
        }
    }
}

pub struct Canvas {
    pub scale: f32,
    pub win_x: f32,
    pub win_y: f32,
    pub canvas_w: f32,
    pub canvas_h: f32,
    pub art_w: f32,
    pub art_h: f32,
    pub scroll: Scroll,
    pub show_grid: bool,
    pub initialized: bool,
}

impl Default for Canvas {
    fn default() -> Self {
        Self {
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
    pub fn new(art_w: f32, art_h: f32) -> Self {
        let mut ret = Self::default();
        ret.art_w = art_w;
        ret.art_h = art_h;
        ret
    }

    pub fn update_pos(&mut self, win_x: f32, win_y: f32) {
        self.win_x = win_x;
        self.win_y = win_y;
    }

    pub fn update_sz(&mut self, canvas_w: f32, canvas_h: f32) {
        self.canvas_w = canvas_w;
        self.canvas_h = canvas_h;
    }

    pub fn draw_circle(&self, rdr: &Renderer, p0: Point2D<f32>, radius: f32, color: [f32;4], filled: bool) {
        let p0 = self.to_cli(p0).into();
        let rad = self.scale * radius;
        rdr.circ(p0, rad, color, filled);
    }

    pub fn draw_bezier(&self,
                       rdr: &Renderer,
                       from: Point2D<f32>,
                       ctrl1:Point2D<f32>,
                       ctrl2: Point2D<f32>,
                       to:Point2D<f32>,
                       c: [f32;4],
                       thickness: f32,
                       ) {
        let p0 = self.to_cli(from).into();
        let p1 = self.to_cli(to).into();
        let cp0 = self.to_cli(ctrl1).into();
        let cp1 = self.to_cli(ctrl2).into();
        rdr.bezier(p0, cp0, cp1, p1, c, thickness);
    }

    pub fn to_cli(&self, p: Point2D<f32>) -> Point2D<f32> {
        let o = self.origin();
        let p0 = Point2D::new(
            o.0 + self.scale * p.x,
            o.1 + self.scale * p.y,
        );
        p0
    }

    pub fn within_circle(&self, x: f32, y: f32, radius: f32, mouse: (f32, f32)) -> bool {
        let o = self.origin();
        let p0 = (
            o.0 + self.scale * x,
            o.1 + self.scale * y,
        );
        let rad = self.scale * radius;

        if mouse.0 < p0.0 + rad
        && mouse.0 > p0.0 - rad
        && mouse.1 < p0.1 + rad
        && mouse.1 > p0.1 - rad {
            true
        } else {
            false
        }
    }

    pub fn draw_pixel(&self, rdr: &Renderer, x: f32, y: f32, color: [f32;4], filled: bool) {
        let o = self.origin();
        if x >= self.art_w { return; }
        if y >= self.art_h { return; }
        let p0 = [
            o.0 + self.scale * x,
            o.1 + self.scale * y,
        ];
        let p1 = [
            o.0 + self.scale * (x+1.),
            o.1 + self.scale * (y+1.),
        ];

        rdr.rect(p0, p1, color, filled);
    }

    pub fn origin(&self) -> (f32, f32) {
        (
            self.win_x + self.scroll.x,
            self.win_y + self.scroll.y
        )
    }

    pub fn draw_canvas(&self, rdr: &Renderer) {
        let o = self.origin();
        rdr.rect(
            [o.0, o.1],
            [
                o.0 + self.art_w * self.scale,
                o.1 + self.art_h * self.scale,
            ],
            LIGHT_GREY,
            true,
        );
    }

    pub fn draw_grid(&self, rdr: &Renderer) {
        if !self.show_grid { return }
        let o = self.origin();

        let color = BLACK;
        let mut x = 0.;
        while x < self.scale * self.art_w {
            rdr.line(
                [o.0 + x, o.1],
                [o.0 + x, o.1 + self.scale * self.art_h],
                color
            );
            x += self.scale;
        }

        let mut y = 0.;
        while y < self.scale * self.art_h {
            rdr.line(
                [o.0, o.1 + y],
                [o.0 + self.scale * self.art_w, o.1 + y],
                color
            );
            y += self.scale;
        }
    }

    pub fn draw_line(&self, rdr: &Renderer, p0: Point2D<f32>, p1: Point2D<f32>, c:[f32;4]) {
        let p0 = self.to_cli(p0).into();
        let p1 = self.to_cli(p1).into();

        rdr.line(p0, p1, c);
    }

    /// convert screen pos to pixel location
    pub fn shrink_size_no_floor(&self, p: &Point2D<f32>) -> Point2D<f32> {
        let Point2D {x: cli_x , y: cli_y} = p;
        let o = self.origin();
        Point2D {
            x: ((cli_x - o.0) / self.scale),
            y: ((cli_y - o.1) / self.scale),
        }
    }

    /// convert screen pos to pixel location
    pub fn shrink_size(&self, p: &Point2D<f32>) -> Point2D<f32> {
        let Point2D {x: cli_x , y: cli_y} = p;
        let o = self.origin();
        Point2D {
            x: ((cli_x - o.0) / self.scale).floor(),
            y: ((cli_y - o.1) / self.scale).floor(),
        }
    }
    /// snap point to grid
    pub fn snap(&self, p: &Point2D<f32>) -> Point2D<f32> {
        let Point2D {x: cli_x , y: cli_y} = p;
        let o = self.origin();
        Point2D {
            x: cli_x.floor(),
            y: cli_y.floor(),
        }
    }


}
