#[macro_use]
mod block;
mod brush;
mod history;
mod color;
mod canvas;
mod stroke;
mod tools;
mod toolbox;

use std::collections::HashSet;
use stdweb::web::event::MouseButton;


use self::toolbox::Toolbox;
use self::block::Block;
use self::history::History;
use self::canvas::Canvas;
use self::color::Color;
use self::brush::Brush;
use self::stroke::Stroke;

pub type Blocks = HashSet<Block>;
pub type BlockOffset = Blocks;

pub struct Xprite {
    history: History,
    canvas: Canvas,
    selected_color: Color,
    toolbox: Toolbox,
    art_h: u32,
    art_w: u32,
    cursor_pos: Option<Block>,
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

    pub fn draw_pixel(&mut self, x: u32, y:u32) {
        let color = self.color();
        self.blocks_mut().insert(Block {x, y, color});
    }

    pub fn add_pixel(&mut self, block: Block) {
        self.blocks_mut().insert(block);
    }

    pub fn remove_pixel(&mut self, block: &Block) {
        self.blocks_mut().remove(block);
    }

    pub fn blocks_mut(&mut self) -> &mut Blocks {
        self.history.current_block_mut()
    }

    pub fn blocks(&self) -> &Blocks {
        self.history.current_block()
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
