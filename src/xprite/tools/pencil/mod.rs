use xprite::tools::Tool;

use stdweb::web::event::MouseButton;
use xprite::{Xprite, Polyline, Pixel, Pixels, Brush, Color};

pub struct Pencil {
    is_mouse_down: Option<MouseButton>,
    current_polyline: Polyline,
    cursor: Option<Pixels>,
    cursor_pos: Option<Pixel>,
    brush: Brush,
    tolerence: f32,
    simplify: bool,
}
impl Pencil {
    pub fn new() -> Self {
        let is_mouse_down = None;
        let cursor = None;
        let cursor_pos = None;
        let brush = Brush::pixel();
        let current_polyline = Polyline::new();

        Self {
            is_mouse_down,
            current_polyline,
            cursor,
            cursor_pos,
            brush,
            simplify: true,
            tolerence: 2.0,
        }
    }

    pub fn draw_polyline(&mut self, xpr: &mut Xprite, polyline: &Polyline) {

        let path = polyline.interp();
        for &Pixel{point, ..} in path.rasterize(xpr).iter() {
            xpr.draw_pixel(point.x, point.y, Some(Color::new(200, 200, 200)));
        }

        // plot simplified points
        for &p in polyline.pos.iter() {
            let (x,y) = xpr.canvas.client_to_grid(p.x as i32, p.y as i32);
            // console!(log, x, y);
            xpr.draw_pixel(x, y, Some(Color::blue()));
        }

        // // plot control points
        // for seg in &path.segments {
        //     let CubicBezierSegment { ctrl1, ctrl2, .. } = seg;
        //     for point in vec![ctrl1, ctrl2] {
        //         let (x, y) = xpr.canvas.client_to_grid(point.x as i32, point.y as i32);
        //         xpr.draw_pixel(x, y, Some(Color::red()));
        //     }
        // }

    }

    fn draw_cursor(&self, xpr: &Xprite) {
        if self.cursor.is_none() { return; }

        let cursor = self.cursor.clone().unwrap();
        for &pos in cursor.iter() {
            xpr.canvas.draw(
                pos.point.x,
                pos.point.y,
                &Color::red().to_string()
            );
        }
    }

}

impl Tool for Pencil {

    fn get_name(&self) -> &'static str {
        "pencil"
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, x: i32, y: i32) {
        let pixels = xpr.canvas.to_pixels(x, y, &self.brush, xpr.color());
        self.cursor = pixels.clone();
        let x_y = xpr.canvas.client_to_grid(x, y);
        self.cursor_pos = Some(Pixel::from_tuple(x_y, Some(xpr.color())));

        // if mouse is done
        if self.is_mouse_down.is_none() || pixels.is_none() {
            self.draw(xpr);
            return;
        }

        self.current_polyline.push(x as f32, y as f32);

        let button = self.is_mouse_down.clone().unwrap();
        if button == MouseButton::Left {
            xpr.add_pixels(&pixels.unwrap());
        } else if button == MouseButton::Right {
            xpr.remove_pixels(&pixels.unwrap());
        }
        self.draw(xpr);
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, x: i32, y: i32, button: MouseButton) {
        self.is_mouse_down = Some(button);
        xpr.history.on_new_polyline_start();

        self.current_polyline.push(x as f32, y as f32);

        let pixels = xpr.canvas.to_pixels(x, y, &self.brush, xpr.color());
        if let Some(pixels) = pixels {
            if button == MouseButton::Left {
                xpr.add_pixels(&pixels);
            } else {
                xpr.remove_pixels(&pixels);
            }
        }
        self.draw(xpr);
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, _x: i32, _y: i32) {
        if self.is_mouse_down.is_none() {return; }
        let button = self.is_mouse_down.clone().unwrap();
        if button == MouseButton::Right { return; }

        if self.simplify {
            if let Some(simplified) = self.current_polyline.reumann_witkam(self.tolerence) {
                xpr.history.undo();
                xpr.history.on_new_polyline_start();
                self.draw_polyline(xpr, &simplified);
            }
        }

        self.current_polyline.clear();
        self.is_mouse_down = None;

        self.draw(xpr);
    }

    fn draw(&self, xpr: &Xprite) {
        xpr.canvas.clear_all();
        for &Pixel{point, color} in xpr.pixels().iter() {
            xpr.canvas.draw(point.x, point.y, &color.unwrap_or(xpr.color()).to_string());
        }
        self.draw_cursor(xpr);
    }

    fn set(&mut self, option: &str, value: &str) {
        match option {
            "simplify" => {
                match value {
                    "true" => self.simplify = true,
                    "false" => self.simplify = false,
                    _ => console!(error, "malformed value: ", value),
                }
            }
            "tolerence" => {
                if let Ok(val) = value.parse() {
                    self.tolerence = val;
                } else {
                    console!(error, "cannot parse val:", value);
                }
            }
            "brush" => {
                match value {
                    "cross" => self.brush = Brush::cross(),
                    "pixel" => self.brush = Brush::pixel(),
                    _ => console!(error, "malformed value: ", value),
                }
            }
            _ => (),
        }
    }
}
