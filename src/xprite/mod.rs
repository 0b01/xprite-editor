mod block;
mod color;
mod canvas;

use std::collections::HashSet;

use self::canvas::Canvas;
use self::color::Color;
use self::block::Block;

// use direction::Direction;
// use stdweb::unstable::TryInto;

pub struct Xprite {
    art_h: u32,
    art_w: u32,
    blocks: HashSet<Block>,
    canvas: Canvas,
    selected_color: Color,
    is_mouse_down: bool,

    cursor: Option<Block>,
}

impl Xprite {
    pub fn new(name: &str, art_w: u32, art_h: u32) -> Xprite {
        let blocks = HashSet::new();
        let canvas = Canvas::new(name, art_w, art_h);

        let selected_color = Color {r: 0, g: 0, b: 0, a: 255};
        let is_mouse_down = false;
        let cursor = None;

        Xprite {
            art_h,
            art_w,
            blocks,
            canvas,
            selected_color,
            is_mouse_down,

            cursor,
        }
    }

    pub fn mouse_down(&mut self, x: i32, y: i32) {
        self.is_mouse_down = true;

        let block = self.canvas.to_block(x, y, self.color());
        if block.is_none() { return; }
        self.blocks.insert(block.unwrap());
        self.draw();
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) {
        let block = self.canvas.to_block(x, y, self.color());
        self.cursor = block;
        self.draw_cursor();
        self.draw();

        if !self.is_mouse_down { return; }
        if block.is_none() { return; }
        self.blocks.insert(block.unwrap());
        self.draw();
    }

    pub fn mouse_up(&mut self, x: i32, y: i32) {

        self.is_mouse_down = false;

        console!(log, "up", x, y);
        let block = self.canvas.to_block(x, y, self.color());
        if block.is_none() { return; }
        self.blocks.insert(block.unwrap());
        self.draw();
    }

    pub fn draw(&self) {
        self.canvas.clear_all();
        for &Block{x, y, color} in &self.blocks {
            self.canvas.draw(x, y, &color.to_string());
        }
        self.draw_cursor();
    }

    fn draw_cursor(&self) {
        if self.cursor.is_none() { return; }
        let cursor = self.cursor.unwrap();
        self.canvas.draw(
            cursor.x,
            cursor.y,
            &Color::red().to_string()
        );
    }

    pub fn color(&self) -> Color {
        self.selected_color
    }
}
