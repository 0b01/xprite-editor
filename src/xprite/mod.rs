#[macro_use]
mod block;
mod brush;
mod history;
mod color;
mod canvas;

use std::collections::HashSet;
use stdweb::web::event::MouseButton;

use self::block::Block;
use self::history::History;
use self::canvas::Canvas;
use self::color::Color;
use self::brush::Brush;

pub type Blocks = HashSet<Block>;
pub type BlockOffset = Blocks;

pub struct Xprite {
    history: History,
    canvas: Canvas,
    selected_color: Color,
    is_mouse_down: Option<MouseButton>,
    cursor: Option<Blocks>,
    cursor_pos: Option<Block>,
    brush: Brush,
}

impl Xprite {
    pub fn new(name: &str, art_w: u32, art_h: u32) -> Xprite {
        let canvas = Canvas::new(name, art_w, art_h);
        let selected_color = Color {r: 0, g: 0, b: 0, a: 255};
        let is_mouse_down = None;
        let cursor = None;
        let cursor_pos = None;
        let history = History::new();
        let brush = Brush::pixel();

        Xprite {
            history,
            canvas,
            selected_color,
            is_mouse_down,
            cursor,
            cursor_pos,
            brush,
        }
    }

    pub fn zoom_in(&mut self) {
        if let Some(cursor_pos) = self.cursor_pos {
            self.canvas.zoom_in_at(5, cursor_pos.x, cursor_pos.y)
        } else {
            self.canvas.zoom_in(5);
        }
        self.draw();
    }

    pub fn zoom_out(&mut self) {
        self.canvas.zoom_out(5);
        self.draw();
    }

    pub fn mouse_down(&mut self, x: i32, y: i32, button: MouseButton) {
        self.is_mouse_down = Some(button);
        self.history.on_new_stroke_start();
        let blocks = self.canvas.to_blocks(x, y, &self.brush, self.color());
        if let Some(blocks) = blocks {
            self.add_pixels(&blocks);
        }
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

    pub fn add_pixels(&mut self, blocks: &Blocks) {
        for &pixel in blocks.iter() {
            self.add_pixel(pixel);
        }
    }

    pub fn remove_pixels(&mut self, blocks: &Blocks) {
        for &pixel in blocks.iter() {
            self.remove_pixel(&pixel);
        }
    }

    pub fn add_pixel(&mut self, block: Block) {
        self.blocks_mut().insert(block);
    }

    pub fn remove_pixel(&mut self, block: &Block) {
        self.blocks_mut().remove(block);
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) {
        let blocks = self.canvas.to_blocks(x, y, &self.brush, self.color());
        self.cursor = blocks.clone();
        self.cursor_pos = Some(Block::from_tuple(self.canvas.get_cursor(x, y), self.color()));
        self.draw();

        if self.is_mouse_down.is_none() { return; }
        if blocks.is_none() { return; }

        let button = self.is_mouse_down.clone().unwrap();
        if button == MouseButton::Left {
            self.add_pixels(&blocks.unwrap());
        } else if button == MouseButton::Right {
            self.remove_pixels(&blocks.unwrap());
        }
        self.draw();
    }

    pub fn mouse_up(&mut self, x: i32, y: i32) {
        console!(log, "up", x, y);
        self.is_mouse_down = None;
    }

    pub fn blocks_mut(&mut self) -> &mut Blocks {
        self.history.current_block_mut()
    }

    pub fn blocks(&self) -> &Blocks {
        self.history.current_block()
    }

    pub fn draw(&self) {
        self.canvas.clear_all();
        for &Block{x, y, color} in self.blocks().iter() {
            self.canvas.draw(x, y, &color.to_string());
        }
        self.draw_cursor();
    }

    fn draw_cursor(&self) {
        if self.cursor.is_none() { return; }

        let cursor = self.cursor.clone().unwrap();
        for &pos in cursor.iter() {
            self.canvas.draw(
                pos.x,
                pos.y,
                &Color::red().to_string()
            );
        }
    }

    pub fn color(&self) -> Color {
        self.selected_color
    }
}
