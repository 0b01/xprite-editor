use crate::prelude::*;
use crate::rendering::Renderer;
use std::rc::Rc;
use std::cell::RefCell;

pub struct Xprite {
    pub history: History,

    pub im_buf: Pixels,
    pub bz_buf: Vec<CubicBezierSegment>,

    pub canvas: Canvas,
    pub selected_color: Color,
    pub toolbox: Toolbox,
    pub cursor_pos: Pixels,
    pub last_mouse_pos: (f32, f32),

    pub scripting: Rc<RefCell<Scripting>>,

    pub rdr: ImageRenderer,

    pub log: String,
}

impl Xprite {
    pub fn new(art_w: f32, art_h: f32) -> Xprite {
        let selected_color = Color {r: 100, g: 100, b: 100, a: 255};
        let history = History::new();
        let cursor_pos = Pixels::new();
        let toolbox = Toolbox::new();
        let canvas = Canvas::new(art_w, art_h);
        let im_buf = Pixels::new();
        let bz_buf = Vec::new();

        let scripting = Rc::new(RefCell::new(Scripting::new()));
        let log = String::new();

        let rdr = ImageRenderer::new(art_w, art_h);

        Xprite {
            scripting,
            last_mouse_pos: (0., 0.),
            history,
            im_buf,
            bz_buf,
            canvas,
            selected_color,
            cursor_pos,
            toolbox,
            log,
            rdr,
        }
    }

    pub fn undo(&mut self) {
        self.history.undo();
    }

    pub fn redo(&mut self) {
        self.history.redo();
    }

    pub fn update_mouse_pos(&mut self, x: f32, y: f32) {
        self.last_mouse_pos.0 = x;
        self.last_mouse_pos.1 = y;
    }

    /// add pixels to temp im_buf
    pub fn add_pixels(&mut self, pixels: &Pixels) {
        self.pixels_mut().extend(pixels);
    }

    /// add pixel to temp im_buf
    pub fn add_pixel(&mut self, pixel: Pixel) {
        self.pixels_mut().push(pixel);
    }

    /// remove pixels from temp im_buf
    pub fn remove_pixels(&mut self, pixels: &Pixels) {
        self.pixels_mut().sub(pixels);
    }

    pub fn pixels_mut(&mut self) -> &mut Pixels {
        &mut self.im_buf
    }

    pub fn pixels(&self) -> &Pixels {
        &self.im_buf
    }

    pub fn set_option(&mut self, opt: &str, val: &str) -> Result<(), String> {
        let tool = self.toolbox.tool();
        let mut current_tool = tool.borrow_mut();
        trace!("setting option {}={}", opt, val);
        current_tool.set(self, opt, val)
    }

    pub fn set_option_for_tool(&mut self, name: &ToolType, opt: &str, val: &str) {
        let tool = self.toolbox.get(name);
        tool.borrow_mut().set(self, opt, val);
    }

    pub fn change_tool(&mut self, name: &ToolType) {
        self.toolbox.change_tool(name);
        self.draw();
    }

    pub fn draw(&mut self) {
        self.toolbox.tool().borrow_mut().draw(self);
    }

    pub fn color(&self) -> Color {
        self.selected_color
    }

    pub fn set_color(&mut self, color: &Color) {
        self.selected_color = *color;
    }

    pub fn new_frame(&mut self) {
        self.pixels_mut().clear();
        self.bz_buf.clear();
    }

    pub fn set_cursor(&mut self, pos: &Pixels) {
        self.cursor_pos = pos.clone();
    }
}

impl Xprite {
    pub fn execute_script(&mut self, path: &str) -> Result<(), String> {
        let s = Rc::clone(&self.scripting);
        let mut scripting = s.borrow_mut();
        scripting.fname = Some(path.to_owned());
        scripting.execute(self)
    }
}


impl Xprite {

    pub fn switch_layer(&mut self, layer: Rc<RefCell<Layer>>) {
        self.history.top_mut().selected_layer = layer;
    }

    pub fn current_layer(&self) -> Rc<RefCell<Layer>> {
        self.history.top().selected_layer.clone()
    }


    pub fn toggle_layer_visibility(&mut self, old: &Rc<RefCell<Layer>>) -> Result<(), String> {
        self.history.enter()?;
        let layers = self.history.top();
        let new_layer = layers.find(&old).unwrap();
        new_layer.borrow_mut().toggle_visible();
        Ok(())
    }

    pub fn remove_layer(&mut self, old: &Rc<RefCell<Layer>>) -> Result<(), String> {
        self.history.enter()?;
        let layers = self.history.top_mut();
        layers.remove_layer(&old);
        Ok(())
    }

    pub fn rename_layer(&mut self, name: &str) -> Result<(), String> {
        self.history.enter()?;
        let layers = self.history.top_mut();
        layers.selected_layer.borrow_mut().name = name.to_owned();
        Ok(())
    }

}

impl Xprite {
    /// render to canvas
    pub fn render(&self, rdr: &mut Renderer) {
        self.canvas.draw_canvas(rdr);
        self.canvas.draw_grid(rdr);

        let top = self.history.top();

        // draw layers
        for layer in top.layers.iter() {
            // skip invisible layers
            if !layer.borrow().visible {
                continue;
            }
            for &Pixel{point, color } in layer.borrow().content.iter() {
                let Vec2D {x, y} = point;
                self.canvas.draw_pixel(rdr, x, y, color.into(), true);
            }

        }

        // info!("------------");
        // for (polyline, path) in cc_buf.iter() {
        //     // let mut circ_buf = Pixels::new();
        //     // circ_buf.extend(&polyline.anchors(self));
        //     // circ_buf.extend(&path.control_points(self));

        // }


        // draw current layer pixels
        for &Pixel{point, color} in self.pixels().iter() {
            let Vec2D {x, y} = point;
            self.canvas.draw_pixel(rdr, x, y, color.into(), true);
        }

        // draw cursor
        for p in self.cursor_pos.iter() {
            let Vec2D {x, y} = p.point;
            self.canvas.draw_pixel(rdr, x, y, RED, false); // draw a rectangle
        }

        for seg in &self.bz_buf {
            let &CubicBezierSegment { ctrl1, ctrl2, from, to } = seg;
            self.canvas.draw_bezier(rdr, from, ctrl1, ctrl2, to, Color::grey().into(), 4.);

            let red = Color::red().into();
            let blue = Color::blue().into();
            self.canvas.draw_circle(rdr, ctrl1, 0.3, red, true);
            self.canvas.draw_circle(rdr, ctrl2, 0.3, red, true);
            self.canvas.draw_circle(rdr, from, 0.3, blue, true);
            self.canvas.draw_circle(rdr, to, 0.3, blue, true);

            self.canvas.draw_line(rdr, from, ctrl1, blue);
            self.canvas.draw_line(rdr, to, ctrl2, blue);
        }

        // // draw circles
        // for p in self.cc_buf.iter() {
        //     let Vec2D {x, y} = p.point;
        //     let c = if let ColorOption::Set(c) = p.color {c.into()}
        //             else {self.color().into()};
        //     self.canvas.draw_circle(rdr, x, y, 0.5, c, true);
        //     // if mouse position is near a circle
        //     if self.canvas.within_circle(x, y, 0.5, self.last_mouse_pos) {
        //         rdr.set_mouse_cursor(crate::rendering::MouseCursorType::Move);
        //     }
        // }

    }
}

impl Xprite {
    pub fn layer_as_im(&mut self) -> Option<&img::DynamicImage> {
        let top = self.history.top();
        // draw layers
        let layer = Rc::clone(&top.selected_layer);
        layer.borrow().draw(&mut self.rdr);
        Some(self.rdr.img())
    }
    /// export pixels to an image via renderer
    pub fn export(&mut self, rdr: &mut Renderer) -> Result<(), String> {
        let top = self.history.top();
        // draw layers
        for layer in top.layers.iter() {
            // skip invisible layers
            if !layer.borrow().visible {
                continue;
            }
            layer.borrow().draw(rdr);
        }
/*
        // draw current layer pixels
        for &Pixel{point, color} in self.pixels().iter() {
            let Vec2D {x, y} = point;
            rdr.rect([x,y],[x+1.,y+1.],color.into(), true);
        }
*/
        Ok(())
    }
}


/// handle events
impl Xprite {

    pub fn event(&mut self, evt: &InputEvent) -> Result<(), String> {
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

    pub fn key_up(&mut self, key: &InputItem) -> Result<(), String> {
        self.set_option(key.as_str(), "false")
    }

    pub fn key_down(&mut self, key: &InputItem) -> Result<(), String> {
        self.set_option(key.as_str(), "true")
    }

    pub fn mouse_move(&mut self, evt: &InputEvent) -> Result<(), String> {
        if let &InputEvent::MouseMove{x, y} = evt {
            let p = Vec2D::new(x, y);
            let tool = self.toolbox.tool();
            tool.borrow_mut().mouse_move(self, p);
        }
        Ok(())
    }

    pub fn mouse_up(&mut self, evt: &InputEvent) -> Result<(), String> {
        if let &InputEvent::MouseUp{x, y, ..} = evt {
            let tool = self.toolbox.tool();
            let p = Vec2D::new(x, y);
            tool.borrow_mut().mouse_up(self, p);
        }
        Ok(())
    }

    pub fn mouse_down(&mut self, evt: &InputEvent) -> Result<(), String> {
        if let &InputEvent::MouseDown{x, y, button} = evt {
            let tool = self.toolbox.tool();
            let p = Vec2D::new(x, y);
            tool.borrow_mut().mouse_down(self, p, button);
        }
        Ok(())
    }
}
