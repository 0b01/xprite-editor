use crate::prelude::*;
use crate::rendering::Renderer;

pub struct Xprite {
    pub event_queue: Vec<MouseEvent>,
    pub history: History,
    pub canvas: Canvas,
    pub selected_color: Color,
    pub toolbox: Toolbox,
    pub art_h: u32,
    pub art_w: u32,
    pub cursor_pos: Option<Pixel>,
}

impl Xprite {
    pub fn new(art_w: u32, art_h: u32) -> Xprite {
        let event_queue = Vec::new();
        let selected_color = Color {r: 0, g: 0, b: 0, a: 255};
        let history = History::new();
        let cursor_pos = None;
        let toolbox = Toolbox::new();
        let canvas = Canvas::new(art_w, art_h);

        Xprite {
            event_queue,
            history,
            canvas,
            selected_color,
            cursor_pos,
            art_h,
            art_w,
            toolbox,
        }
    }

    pub fn update_canvas_dims(&mut self, canvas_w: u32, canvas_h: u32) {
        self.canvas.update(canvas_w, canvas_h)
    }

    pub fn mouse_move(&mut self, evt: &MouseEvent) -> Option<()> {
        if let &MouseEvent::MouseMove{x, y} = evt {
            if out_of_bounds(x, y) {return Some(());}
            let p = Point2D::new(x, y);
            let point = self.canvas.client_to_grid(p);
            let color = ColorOption::Set(self.color());
            self.cursor_pos = Some(Pixel{point, color});

            let tool = self.toolbox.tool();
            tool.borrow_mut().mouse_move(self, p);
        }
        Some(())
    }

    pub fn mouse_up(&mut self, evt: &MouseEvent) -> Option<()> {
        if let &MouseEvent::MouseUp{x, y} = evt {
            if out_of_bounds(x, y) { return Some(()); }
            let tool = self.toolbox.tool();
            let p = Point2D::new(x, y);
            tool.borrow_mut().mouse_up(self, p);
        }
        Some(())
    }

    pub fn mouse_down(&mut self, evt: &MouseEvent) -> Option<()> {
        if let &MouseEvent::MouseDown{x, y, button} = evt {
            if out_of_bounds(x, y) {return Some(());}
            let tool = self.toolbox.tool();
            let p = Point2D::new(x, y);
            tool.borrow_mut().mouse_down(self, p, button);
        }
        Some(())
    }


    pub fn get_height(&self) -> u32 {
        self.art_h
    }

    pub fn get_width(&self) -> u32 {
        self.art_w
    }

    pub fn zoom_in(&mut self) -> Option<()> {
        if let Some(cursor_pos) = self.cursor_pos {
            self.canvas.zoom_in_at(5, cursor_pos.point)
        } else {
            self.canvas.zoom_in(5);
        }
        self.draw()
    }

    pub fn zoom_out(&mut self) -> Option<()> {
        self.canvas.zoom_out(5);
        self.draw()
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

    pub fn draw_pixel(&mut self, x: u32, y:u32, color: ColorOption) {
        let point = Point2D::new(x, y);
        let color = match color {
            ColorOption::Unset => self.color(),
            ColorOption::Set(c) => c
        };
        let color = ColorOption::Set(color);
        self.pixels_mut().push(Pixel {point, color});
    }

    pub fn add_pixel(&mut self, pixel: Pixel) {
        self.pixels_mut().push(pixel);
    }

    pub fn pixels_mut(&mut self) -> &mut Pixels {
        self.history.current_pixels_mut()
    }

    pub fn pixels(&self) -> &Pixels {
        self.history.current_pixels()
    }

    pub fn set_option(&mut self, opt: &str, val: &str) {
        let tool = self.toolbox.tool();
        tool.borrow_mut().set(self, opt, val);
    }

    pub fn set_option_for_tool(&mut self, name: &str, opt: &str, val: &str) {
        if let Some(tool) = self.toolbox.get(name) {
            tool.borrow_mut().set(self, opt, val);
        } else {
            panic!("toolbox does not contain {}", name);
        }
    }

    pub fn change_tool(&mut self, name: &str) {
        self.toolbox.change_to(name);
    }

    pub fn draw(&self) -> Option<()> {
        let tool = self.toolbox.tool();
        tool.borrow().draw(self);
        Some(())
    }

    pub fn color(&self) -> Color {
        self.selected_color
    }

    pub fn set_color(&mut self, r:u8, g:u8, b:u8) {
        self.selected_color = Color::new(r, g, b);
    }

    pub fn print_cursor_location(&self) {
        if self.cursor_pos.is_none() { return; }
        let pos = self.cursor_pos.unwrap();;
        panic!("cursor: ({}, {})", pos.point.x, pos.point.y);
    }
}

fn out_of_bounds(x: i32, y: i32) -> bool {
    if x < 0 || y < 0 {
        return true;
    }
    return false;
}
