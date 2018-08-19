use xprite::tools::Tool;

use stdweb::web::event::MouseButton;
use xprite::{Xprite, Stroke, Block, Blocks, Brush, Color};

pub struct Pencil {
    is_mouse_down: Option<MouseButton>,
    current_stroke: Stroke,
    cursor: Option<Blocks>,
    cursor_pos: Option<Block>,
    brush: Brush,
}

impl Tool for Pencil {

    fn get_name(&self) -> &'static str {
        "pencil"
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, x: i32, y: i32) {
        let blocks = xpr.canvas.to_blocks(x, y, &self.brush, xpr.color());
        self.cursor = blocks.clone();
        let x_y = xpr.canvas.get_cursor(x, y);
        self.cursor_pos = Some(Block::from_tuple(x_y, xpr.color()));

        // if mouse is done
        if self.is_mouse_down.is_none() || blocks.is_none() {
            self.draw(xpr);
            return;
        }

        self.current_stroke.push(x_y.0, x_y.1);

        let button = self.is_mouse_down.clone().unwrap();
        if button == MouseButton::Left {
            xpr.add_pixels(&blocks.unwrap());
        } else if button == MouseButton::Right {
            xpr.remove_pixels(&blocks.unwrap());
        }
        self.draw(xpr);
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, x: i32, y: i32, button: MouseButton) {
        self.is_mouse_down = Some(button);
        xpr.history.on_new_stroke_start();
        let (stroke_pos_x, stroke_pos_y) = xpr.canvas.get_cursor(x, y);
        self.current_stroke.push(stroke_pos_x, stroke_pos_y);

        let blocks = xpr.canvas.to_blocks(x, y, &self.brush, xpr.color());
        if let Some(blocks) = blocks {
            if button == MouseButton::Left {
                xpr.add_pixels(&blocks);
            } else {
                xpr.remove_pixels(&blocks);
            }
        }
        self.draw(xpr);
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, x: i32, y: i32) {
        console!(log, "up", x, y);

        if self.is_mouse_down.is_none() {return; }
        let button = self.is_mouse_down.clone().unwrap();
        if button == MouseButton::Right { return; }

        if let Some(simplified) = self.current_stroke.reumann_witkam(2.0) {
            console!(log, &simplified);
            xpr.history.undo();
            xpr.history.on_new_stroke_start();
            self.draw_stroke(xpr, &simplified);
        }

        self.current_stroke.clear();
        self.is_mouse_down = None;

        self.draw(xpr);
    }

    fn draw(&self, xpr: &Xprite) {
        xpr.canvas.clear_all();
        for &Block{x, y, color} in xpr.blocks().iter() {
            xpr.canvas.draw(x, y, &color.to_string());
        }
        self.draw_cursor(xpr);
    }
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
        }
    }

    pub fn draw_stroke(&mut self, xpr: &mut Xprite, stroke: &Stroke) {
        for &(ref x, ref y) in stroke.pos.iter() {
            xpr.draw_pixel(*x, *y);
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
