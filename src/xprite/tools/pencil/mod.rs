use xprite::tools::Tool;

use stdweb::web::event::MouseButton;
use xprite::{Xprite, Stroke, Pixel, Pixels, Brush, Color};

pub struct Pencil {
    is_mouse_down: Option<MouseButton>,
    current_stroke: Stroke,
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
        let current_stroke = Stroke::new();

        Self {
            is_mouse_down,
            current_stroke,
            cursor,
            cursor_pos,
            brush,
            simplify: true,
            tolerence: 2.0,
        }
    }

    pub fn draw_stroke(&mut self, xpr: &mut Xprite, stroke: &Stroke) {
        for &Pixel{x, y, ..} in stroke.rasterize().iter() {
            xpr.draw_pixel(x, y);
        }
    }

    fn draw_cursor(&self, xpr: &Xprite) {
        if self.cursor.is_none() { return; }

        let cursor = self.cursor.clone().unwrap();
        for &pos in cursor.iter() {
            xpr.canvas.draw(
                pos.x,
                pos.y,
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
        let x_y = xpr.canvas.get_cursor(x, y);
        self.cursor_pos = Some(Pixel::from_tuple(x_y, xpr.color()));

        // if mouse is done
        if self.is_mouse_down.is_none() || pixels.is_none() {
            self.draw(xpr);
            return;
        }

        self.current_stroke.push(x_y.0, x_y.1);

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
        xpr.history.on_new_stroke_start();
        let (stroke_pos_x, stroke_pos_y) = xpr.canvas.get_cursor(x, y);
        self.current_stroke.push(stroke_pos_x, stroke_pos_y);

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
            if let Some(simplified) = self.current_stroke.reumann_witkam(self.tolerence) {
                xpr.history.undo();
                xpr.history.on_new_stroke_start();
                self.draw_stroke(xpr, &simplified);
            }
        }

        self.current_stroke.clear();
        self.is_mouse_down = None;

        self.draw(xpr);
    }

    fn draw(&self, xpr: &Xprite) {
        xpr.canvas.clear_all();
        for &Pixel{x, y, color} in xpr.pixels().iter() {
            xpr.canvas.draw(x, y, &color.to_string());
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