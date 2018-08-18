mod block;
mod color;
mod canvas;

use std::collections::HashSet;
use stdweb::web::event::MouseButton;

use self::canvas::Canvas;
use self::color::Color;
use self::block::Block;

pub struct Xprite {
    history: Vec<HashSet<Block>>,
    redos: Vec<HashSet<Block>>,
    canvas: Canvas,
    selected_color: Color,
    is_mouse_down: Option<MouseButton>,
    cursor: Option<Block>,
}

impl Xprite {
    pub fn new(name: &str, art_w: u32, art_h: u32) -> Xprite {
        let history = vec![HashSet::new()];
        let redos = vec![];
        let canvas = Canvas::new(name, art_w, art_h);

        let selected_color = Color {r: 0, g: 0, b: 0, a: 255};
        let is_mouse_down = None;
        let cursor = None;
        Xprite {
            history,
            redos,
            canvas,
            selected_color,
            is_mouse_down,
            cursor,
        }
    }

    pub fn zoom_in(&mut self) {
        if let Some(cursor) = self.cursor {
            self.canvas.zoom_in_at(5, cursor.x, cursor.y)
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

        self.push_history();
        self.clear_redo();

        let block = self.canvas.to_block(x, y, self.color());
        if let Some(block) = block {
            self.add_pixel(block);
        }
        self.draw();
    }

    fn push_history(&mut self) {
        let latest = self.current_block().clone();
        self.history.push(latest);
    }

    fn clear_redo(&mut self) {
        self.redos = Vec::new();
    }

    pub fn undo(&mut self) {
        if let Some(last) = self.history.pop() {
            self.redos.push(last);
        }
        self.draw();
    }

    pub fn redo(&mut self) {
        if let Some(last) = self.redos.pop() {
            self.history.push(last);
        }
        self.draw();
    }

    pub fn add_pixel(&mut self, block: Block) {
        self.current_block_mut().insert(block);
    }

    fn current_block_mut(&mut self) -> &mut HashSet<Block> {
        self.history.last_mut().unwrap()
    }

    fn current_block(&self) -> &HashSet<Block> {
        self.history.last().unwrap()
    }

    pub fn remove_pixel(&mut self, block: &Block) {
        self.current_block_mut().remove(block);
    }

    pub fn mouse_move(&mut self, x: i32, y: i32) {
        let block = self.canvas.to_block(x, y, self.color());
        self.cursor = block;
        self.draw_cursor();
        self.draw();

        if self.is_mouse_down.is_none() { return; }
        if block.is_none() { return; }

        let button = self.is_mouse_down.clone().unwrap();
        if button == MouseButton::Left {
            self.add_pixel(block.unwrap());
        } else if button == MouseButton::Right {
            self.remove_pixel(&block.unwrap());
        }
        self.draw();
    }

    pub fn mouse_up(&mut self, x: i32, y: i32) {

        console!(log, "up", x, y);
        let block = self.canvas.to_block(x, y, self.color());
        if block.is_none() { return; }

        let button = self.is_mouse_down.clone().unwrap();
        if button == MouseButton::Left {
            self.add_pixel(block.unwrap());
        } else if button == MouseButton::Right {
            self.remove_pixel(&block.unwrap());
        }

        self.draw();

        self.is_mouse_down = None;
    }

    pub fn draw(&self) {
        self.canvas.clear_all();
        for &Block{x, y, color} in self.current_block().iter() {
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
