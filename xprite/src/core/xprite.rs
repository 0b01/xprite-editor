use crate::prelude::*;
use crate::rendering::Renderer;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

#[derive(Serialize, Deserialize, Debug)]
pub struct Xprite {
    pub history: History,

    #[serde(skip_serializing, skip_deserializing)]
    pub im_buf: Pixels,
    #[serde(skip_serializing, skip_deserializing)]
    pub bz_buf: Vec<CubicBezierSegment>,

    pub canvas: Canvas,
    pub selected_color: Color,
    #[serde(skip_serializing, skip_deserializing)]
    pub palette_man: PaletteManager,

    #[serde(skip_serializing, skip_deserializing)]
    pub toolbox: Toolbox,
    pub cursor: Pixels,
    pub last_mouse_pos: Vec2f,

    #[serde(skip_serializing, skip_deserializing)]
    pub log: Arc<Mutex<String>>,

    pub redraw: bool,
}

impl Xprite {
    pub fn new(art_w:f32, art_h: f32) -> Xprite {
        let palette_man = PaletteManager::new().expect("Cannot initialize palettes");
        let selected_color = Color { r: 0, g: 0, b: 0, a: 255 };
        let history = History::new();
        let cursor = Pixels::new();
        let toolbox = Toolbox::new();
        let canvas = Canvas::new(art_w, art_h);
        let im_buf = Pixels::new();
        let bz_buf = Vec::new();
        let log = Arc::new(Mutex::new(String::new()));
        let redraw = true;

        Xprite {
            last_mouse_pos: Vec2f { x:0., y:0. },
            history,
            im_buf,
            bz_buf,
            canvas,
            selected_color,
            cursor,
            toolbox,
            log,
            palette_man,
            redraw,
        }
    }

    pub fn undo(&mut self) {
        self.history.undo();
        self.redraw = true;
    }

    pub fn redo(&mut self) {
        self.history.redo();
        self.redraw = true;
    }

    pub fn update_mouse_pos(&mut self, x: f32, y: f32) {
        self.last_mouse_pos.x = x;
        self.last_mouse_pos.y = y;
    }

    /// add pixels to temp im_buf
    pub fn add_pixels(&mut self, pixels: &Pixels) {
        self.pixels_mut().extend(pixels);
    }

    /// add pixel to temp im_buf
    pub fn add_pixel(&mut self, pixel: Pixel) {
        self.pixels_mut().push(pixel)
    }

    /// remove pixels from temp im_buf
    pub fn remove_pixels(&mut self, pixels: &Pixels) {
        self.pixels_mut().sub_(pixels);
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

    pub fn set_option_for_tool(
        &mut self,
        name: &ToolType,
        opt: &str,
        val: &str,
    ) -> Result<(), String> {
        let tool = self.toolbox.get(name);
        tool.borrow_mut().set(self, opt, val).unwrap();
        Ok(())
    }

    pub fn change_tool(&mut self, name: ToolType) -> Result<(), String> {
        self.toolbox.change_tool(name);
        self.draw()
    }

    pub fn draw(&mut self) -> Result<(), String> {
        self.redraw = self.toolbox.tool().borrow_mut().draw(self)?;
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), String> {
        self.redraw = self.toolbox.tool().borrow_mut().update(self)?;
        Ok(())
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
        self.cursor = pos.clone();
    }
}

impl Xprite {
    pub fn switch_layer(&mut self, group_id: usize, layer: usize) {
        self.history.top_mut().sel_group = group_id;
        self.history.top_mut().selected = layer;
    }

    pub fn current_layer(&self) -> Option<&Layer> {
        self.history.top().selected_layer()
    }

    pub fn current_layer_mut(&mut self) -> Option<&mut Layer> {
        self.history.top_mut().selected_layer_mut()
    }

    pub fn toggle_layer_visibility(&mut self, group: usize, layer: usize) -> Result<(), String> {
        self.history.enter()?;
        self.history.top_mut().toggle_layer_visibility(group, layer);
        self.redraw = true;
        Ok(())
    }

    pub fn remove_layer(&mut self, group: usize, old: usize) -> Result<(), String> {
        self.history.enter()?;
        let layers = self.history.top_mut();
        layers.remove_layer(group, old);
        Ok(())
    }

    pub fn rename_layer(&mut self, name: &str) -> Result<(), String> {
        self.history.enter()?;
        let layers = self.history.top_mut();
        layers.selected_layer_mut().unwrap().name = name.to_owned();
        Ok(())
    }
}

impl Xprite {

    pub fn render_cursor(&self, rdr: &mut Renderer) {
        for p in self.cursor.iter() {
            let Vec2f {x, y} = p.point;
            self.canvas.draw_pixel_rect(rdr, x, y, p.color.into(), true);
        }
    }

    /// render to canvas
    pub fn render(&self, rdr: &mut Renderer) {
        rdr.reset();
        self.canvas.draw_canvas(rdr);


        self.canvas.draw_grid(rdr);

        for seg in &self.bz_buf {
            let &CubicBezierSegment {
                ctrl1,
                ctrl2,
                from,
                to,
            } = seg;
            self.canvas
                .draw_bezier(rdr, from, ctrl1, ctrl2, to, Color::grey().into(), 1.);
            let red = Color::red().into();
            let blue = Color::blue().into();
            self.canvas.draw_circle(rdr, from, 0.3, blue, true);
            self.canvas.draw_circle(rdr, ctrl1, 0.3, red, true);
            self.canvas.draw_circle(rdr, ctrl2, 0.3, red, true);
            self.canvas.draw_circle(rdr, to, 0.3, blue, true);
            self.canvas.draw_line(rdr, from, ctrl1, blue);
            self.canvas.draw_line(rdr, to, ctrl2, blue);

            if self.canvas.within_circle(from, self.last_mouse_pos)
                || self.canvas.within_circle(ctrl1, self.last_mouse_pos)
                || self.canvas.within_circle(ctrl2, self.last_mouse_pos)
                || self.canvas.within_circle(to, self.last_mouse_pos)
            {
                rdr.set_mouse_cursor(crate::rendering::MouseCursorType::Hand);
            }
        }
    }
}

impl Xprite {
    pub fn layer_as_im(&mut self) -> img::DynamicImage {
        let layer = self.history.top_mut().selected_layer().unwrap();
        let mut rdr = ImageRenderer::new(self.canvas.art_w, self.canvas.art_h);
        layer.draw(&mut rdr);
        rdr.render();
        rdr.image
    }

    #[deprecated]
    pub fn img_hash(&mut self) -> u64 {
        let mut s = DefaultHasher::new();
        let top = self.history.top();
        top.hash(&mut s);
        self.im_buf.hash(&mut s);
        s.finish()
    }

    pub fn preview(&mut self, rdr: &mut Renderer) -> Result<(), String> {
        let top = self.history.top();
        // draw layers
        for layer in top.iter_layers() {
            // skip invisible layers
            if !layer.visible {
                continue;
            }
            layer.draw(rdr);
        }

        // draw current layer pixels
        for &Pixel { point, color } in self.pixels().iter() {
            let Vec2f { x, y } = point;
            rdr.pixel(x, y, color.into(), true);
        }

        Ok(())
    }

    /// export pixels to an image via renderer
    pub fn export(&mut self, rdr: &mut Renderer) -> Result<(), String> {
        let top = self.history.top();
        // draw layers
        for layer in top.iter_layers() {
            // skip invisible layers
            if !layer.visible {
                continue;
            }
            layer.draw(rdr);
        }
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
        if let &InputEvent::MouseMove { x, y } = evt {
            let p = Vec2f { x, y };
            let tool = self.toolbox.tool();
            tool.borrow_mut().mouse_move(self, p)?;
        }
        Ok(())
    }

    pub fn mouse_up(&mut self, evt: &InputEvent) -> Result<(), String> {
        if let &InputEvent::MouseUp { x, y, .. } = evt {
            let tool = self.toolbox.tool();
            let p = Vec2f { x, y };
            tool.borrow_mut().mouse_up(self, p)?;
        }
        Ok(())
    }

    pub fn mouse_down(&mut self, evt: &InputEvent) -> Result<(), String> {
        if let &InputEvent::MouseDown { x, y, button } = evt {
            let tool = self.toolbox.tool();
            let p = Vec2f { x, y };
            tool.borrow_mut().mouse_down(self, p, button)?;
        }
        Ok(())
    }
}
