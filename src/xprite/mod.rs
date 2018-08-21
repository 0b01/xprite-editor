#[macro_use]
mod common;
mod tools;
mod history;
mod canvas;
mod toolbox;

use stdweb::web::event::MouseButton;
use lyon_geom::euclid::Point2D;

use self::toolbox::Toolbox;
use self::common::pixel::Pixel;
use self::common::pixels::Pixels;
use self::history::History;
use self::canvas::Canvas;
use self::common::path::Path;
use self::common::color::Color;
use self::common::brush::Brush;
use self::common::polyline::Polyline;


pub type PixelOffsets = Pixels;

pub struct Xprite {
    history: History,
    canvas: Canvas,
    selected_color: Color,
    toolbox: Toolbox,
    art_h: u32,
    art_w: u32,
    cursor_pos: Option<Pixel>,
}

impl Xprite {
    pub fn new(name: &str, art_w: u32, art_h: u32) -> Xprite {
        let canvas = Canvas::new(name, art_w, art_h);
        let selected_color = Color {r: 0, g: 0, b: 0, a: 255};
        let history = History::new();
        let cursor_pos = None;
        let toolbox = Toolbox::new();

        Xprite {
            history,
            canvas,
            selected_color,
            cursor_pos,
            art_h,
            art_w,
            toolbox,
        }
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) {
        let x_y = self.canvas.client_to_grid(x, y);
        self.cursor_pos = Some(Pixel::from_tuple(x_y, Some(self.color())));

        let tool = self.toolbox.tool();
        tool.borrow_mut().mouse_move(self, x, y);
    }

    pub fn mouse_up(&mut self, x: i32, y: i32) {
        let tool = self.toolbox.tool();
        tool.borrow_mut().mouse_up(self, x, y);
    }

    pub fn mouse_down(&mut self, x: i32, y: i32, button: MouseButton) {
        let tool = self.toolbox.tool();
        tool.borrow_mut().mouse_down(self, x, y, button);
    }

    pub fn get_height(&self) -> u32 {
        self.art_h
    }

    pub fn get_width(&self) -> u32 {
        self.art_w
    }

    pub fn zoom_in(&mut self) {
        if let Some(cursor_pos) = self.cursor_pos {
            self.canvas.zoom_in_at(5, cursor_pos.point)
        } else {
            self.canvas.zoom_in(5);
        }
        self.draw();
    }

    pub fn zoom_out(&mut self) {
        self.canvas.zoom_out(5);
        self.draw();
    }

    pub fn undo(&mut self) {
        self.history.undo();
        self.draw();
    }

    pub fn redo(&mut self) {
        self.history.redo();
        self.draw();
    }

    pub fn add_pixels(&mut self, pixels: &Pixels) {
        for &pixel in pixels.iter() {
            self.add_pixel(pixel);
        }
    }

    pub fn remove_pixels(&mut self, pixels: &Pixels) {
        for &pixel in pixels.iter() {
            self.remove_pixel(&pixel);
        }
    }

    pub fn draw_pixel(&mut self, x: u32, y:u32, color: Option<Color>) {
        let point = Point2D::new(x, y);
        let color = if color.is_none() { Some(self.color()) } else { color };
        self.pixels_mut().insert(Pixel {point, color});
    }

    pub fn add_pixel(&mut self, pixel: Pixel) {
        self.pixels_mut().insert(pixel);
    }

    pub fn remove_pixel(&mut self, pixel: &Pixel) {
        self.pixels_mut().remove(pixel);
    }

    pub fn pixels_mut(&mut self) -> &mut Pixels {
        self.history.current_pixels_mut()
    }

    pub fn pixels(&self) -> &Pixels {
        self.history.current_pixels()
    }

    pub fn set_option(&self, opt: &str, val: &str) {
        let tool = self.toolbox.tool();
        tool.borrow_mut().set(opt, val);
    }

    pub fn set_option_for_tool(&self, name: &str, opt: &str, val: &str) {
        if let Some(tool) = self.toolbox.get(name) {
            tool.borrow_mut().set(opt, val);
        } else {
            console!(error, "toolbox does not have ", name);
        }
    }

    pub fn change_tool(&mut self, name: &str) {
        self.toolbox.change_to(name);
    }

    pub fn draw(&self) {
        let tool = self.toolbox.tool();
        tool.borrow().draw(self);
    }

    pub fn color(&self) -> Color {
        self.selected_color
    }

    pub fn set_color(&mut self, r:u8, g:u8, b:u8) {
        self.selected_color = Color::new(r, g, b);
    }
}
