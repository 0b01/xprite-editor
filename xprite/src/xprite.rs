use crate::prelude::*;
use crate::rendering::Renderer;

pub struct Xprite {
    pub event_queue: Vec<InputEvent>,
    pub history: History,
    pub buffer: Pixels,
    pub canvas: Canvas,
    pub selected_color: Color,
    pub toolbox: Toolbox,
    pub art_h: f32,
    pub art_w: f32,
    pub cursor_pos: Option<Pixel>,
}

impl Xprite {
    pub fn new(art_w: f32, art_h: f32) -> Xprite {
        let event_queue = Vec::new();
        let selected_color = Color {r: 0, g: 0, b: 0, a: 255};
        let history = History::new();
        let cursor_pos = None;
        let toolbox = Toolbox::new();
        let canvas = Canvas::new(art_w, art_h);
        let buffer = Pixels::new();

        Xprite {
            event_queue,
            history,
            buffer,
            canvas,
            selected_color,
            cursor_pos,
            art_h,
            art_w,
            toolbox,
        }
    }


    pub fn undo(&mut self) {
        self.history.undo();
    }

    pub fn redo(&mut self) {
        self.history.redo();
    }

    /// add stroke to temp buffer
    pub fn add_stroke(&mut self, pixels: &[Pixel]) {
        for &pixel in pixels.iter() {
            self.add_pixel(pixel);
        }
    }

    /// add pixels to temp buffer
    pub fn add_pixels(&mut self, pixels: &Pixels) {
        for &pixel in pixels.iter() {
            self.add_pixel(pixel);
        }
    }

    /// add pixel to temp buffer
    pub fn add_pixel(&mut self, pixel: Pixel) {
        self.pixels_mut().push(pixel);
    }

    pub fn pixels_mut(&mut self) -> &mut Pixels {
        &mut self.buffer
    }

    pub fn pixels(&self) -> &Pixels {
        &self.buffer
    }

    pub fn set_option(&mut self, opt: &str, val: &str) -> Option<()> {
        let tool = self.toolbox.tool();
        let mut current_tool = tool.borrow_mut();
        trace!("setting option {}={}", opt, val);
        current_tool.set(self, opt, val)
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

    pub fn color(&self) -> Color {
        self.selected_color
    }

    pub fn set_color(&mut self, r:u8, g:u8, b:u8) {
        self.selected_color = Color::new(r, g, b);
    }

    pub fn new_frame(&mut self) {
        self.pixels_mut().clear();
    }

    pub fn print_cursor_location(&self) {
        if self.cursor_pos.is_none() { return; }
        let pos = self.cursor_pos.unwrap();;
        panic!("cursor: ({}, {})", pos.point.x, pos.point.y);
    }

    pub fn render(&self, rdr: &Renderer) {
        self.canvas.draw_canvas(rdr);
        self.canvas.draw_grid(rdr);
        for &Pixel{point, color: _ } in self.history.current_pixels().iter() {
            let Point2D {x, y} = point;
            self.canvas.draw_pixel(rdr, x, y, BLACK);
        }
        for &Pixel{point, color: _ } in self.pixels().iter() {
            let Point2D {x, y} = point;
            self.canvas.draw_pixel(rdr, x, y, BLACK);
        }
    }
}


/// handle events
impl Xprite {

    pub fn event(&mut self, evt: &InputEvent) -> Option<()> {
        use self::InputEvent::*;
        trace!("{:#?}", evt);
        match evt {
            MouseMove { .. } => self.mouse_move(evt),
            MouseDown { .. } => self.mouse_down(evt),
            MouseUp { .. } => self.mouse_up(evt),
            KeyUp { key } => self.key_up(key),
            KeyDown { key } => self.key_down(key),
        }
    }

    pub fn key_up(&mut self, key: &InputItem) -> Option<()> {
        self.set_option(key.as_str(), "false")
    }

    pub fn key_down(&mut self, key: &InputItem) -> Option<()> {
        self.set_option(key.as_str(), "true")
    }

    pub fn mouse_move(&mut self, evt: &InputEvent) -> Option<()> {
        if let &InputEvent::MouseMove{x, y} = evt {
            let p = Point2D::new(x, y);
            let point = self.canvas.shrink_size(&p);
            let color = ColorOption::Set(self.color());
            self.cursor_pos = Some(Pixel{point, color});

            let tool = self.toolbox.tool();
            tool.borrow_mut().mouse_move(self, p);
        }
        Some(())
    }

    pub fn mouse_up(&mut self, evt: &InputEvent) -> Option<()> {
        if let &InputEvent::MouseUp{x, y} = evt {
            let tool = self.toolbox.tool();
            let p = Point2D::new(x, y);
            tool.borrow_mut().mouse_up(self, p);
        }
        Some(())
    }

    pub fn mouse_down(&mut self, evt: &InputEvent) -> Option<()> {
        if let &InputEvent::MouseDown{x, y, button} = evt {
            let tool = self.toolbox.tool();
            let p = Point2D::new(x, y);
            tool.borrow_mut().mouse_down(self, p, button);
        }
        Some(())
    }

}