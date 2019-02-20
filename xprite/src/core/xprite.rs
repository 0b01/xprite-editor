use crate::prelude::*;
use crate::rendering::Renderer;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::path::Path;
use img::GenericImageView;

pub struct Xprite {
    pub name: String,

    pub history: History,

    pub im_buf: Pixels,
    pub line_buf: Vec<Rect>,
    pub bz_buf: Vec<CubicBezierSegment>,
    marq_buf: Vec<MarqueePixel>,

    pub canvas: Canvas,
    pub color_picker_color: Option<Color>,
    pub selected_color: Color,
    pub palette_man: PaletteManager,

    pub toolbox: Toolbox,
    pub cursor: Pixels,
    pub last_mouse_pos: Vec2f,

    pub log: Arc<Mutex<String>>,

    pub redraw: bool,
}

impl Default for Xprite {
    fn default() -> Self {
        let palette_man =
            PaletteManager::new().expect("Cannot initialize palettes");
        let selected_color = Color {
            r: 0,
            g: 0,
            b: 0,
            a: 255,
        };
        Self {
            name: "Untitled".to_owned(),
            palette_man,
            selected_color,
            color_picker_color: None,
            history: Default::default(),
            im_buf: Default::default(),
            line_buf: Default::default(),
            bz_buf: Default::default(),
            marq_buf: Default::default(),
            canvas: Default::default(),
            toolbox: Default::default(),
            cursor: Default::default(),
            last_mouse_pos: Default::default(),
            log: Arc::new(Mutex::new(String::new())),
            redraw: true,
        }
    }
}

impl Xprite {
    pub fn new(name: String, art_w: f64, art_h: f64) -> Xprite {
        let canvas = Canvas::new(art_w, art_h);
        Xprite {
            name,
            canvas,
            ..Default::default()
        }
    }

    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    pub fn undo(&mut self) {
        self.history.undo();
        self.redraw = true;
    }

    pub fn redo(&mut self) {
        self.history.redo();
        self.redraw = true;
    }

    pub fn update_mouse_pos(&mut self, x: f64, y: f64) {
        self.last_mouse_pos.x = x;
        self.last_mouse_pos.y = y;
    }

    /// add pixels to temp im_buf
    pub fn add_pixels(&mut self, orig: &Pixels) {
        self.pixels_mut().extend(&orig);
        let reflected = self.toolbox.symmetry.borrow_mut().process(&orig);
        self.pixels_mut().extend(&reflected);
    }

    pub fn finalize_pixels(&mut self, pixs: &Pixels) -> Result<(), String>{
        self.history.enter()?;
        self.current_layer_mut().unwrap()
            .content .extend(&pixs);
        let reflected = self.toolbox.symmetry.borrow_mut().process(&pixs);
        self.current_layer_mut().unwrap()
            .content.extend(&reflected);
        Ok(())
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
        let mut redraw = false;
        redraw |= Rc::clone(&self.toolbox.symmetry).borrow_mut().update(self)?;
        redraw |= self.toolbox.tool().borrow_mut().update(self)?;
        self.redraw = redraw;
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
        self.marq_buf.clear();
    }

    pub fn set_cursor(&mut self, pos: &Pixels) {
        self.cursor = pos.clone();
    }

    pub fn add_marquee(&mut self, marq: &[MarqueePixel]) {
        self.marq_buf.extend(marq);
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

    pub fn toggle_layer_visibility(
        &mut self,
        group: usize,
        layer: usize,
    ) -> Result<(), String> {
        self.history.enter()?;
        self.history.top_mut().toggle_layer_visibility(group, layer);
        self.redraw = true;
        Ok(())
    }

    pub fn remove_layer(
        &mut self,
        group: usize,
        old: usize,
    ) -> Result<(), String> {
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
    pub fn render(&self, rdr: &mut Renderer) {
        self.render_cursor(rdr);
        self.render_bezier(rdr);
        self.render_line(rdr);
        self.render_marquee(rdr);
        self.render_canvas_extras(rdr);
    }

    pub fn render_line(&self, rdr: &mut Renderer) {
        for Rect(p0, p1) in &self.line_buf {
            self.canvas.draw_line(rdr, *p0, *p1, Color::red().into());
        }
    }

    pub fn render_cursor(&self, rdr: &mut Renderer) {
        let outline = self.cursor.outline();
        for p in self.cursor.iter() {
            self.canvas.draw_pixel_rect(
                rdr,
                p.point,
                p.color.into(),
                true,
            );
        }
        for (point, outline) in outline.iter() {
            self.canvas.draw_pixel_outline(rdr, *point, *outline);
        }
    }

    pub fn render_canvas_extras(&self, rdr: &mut Renderer) {
        rdr.reset();
        // self.canvas.draw_canvas(rdr);
        self.canvas.draw_grid(rdr);
    }

    pub fn render_bezier(&self, rdr: &mut Renderer) {
        for seg in &self.bz_buf {
            let &CubicBezierSegment {
                ctrl1,
                ctrl2,
                from,
                to,
            } = seg;
            self.canvas.draw_bezier(
                rdr,
                from,
                ctrl1,
                ctrl2,
                to,
                Color::grey().into(),
                1.,
            );
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

    pub fn render_marquee(&self, rdr: &mut Renderer) {
        for (ith, (p, outline)) in self.marq_buf.iter().enumerate() {
            self.canvas.draw_pixel_marqee(rdr, *p, *outline, ith);
        }
    }
}

impl Xprite {
    pub fn as_img(&mut self) -> Result<img::DynamicImage, String> {
        let mut rdr = ImageRenderer::new(self.canvas.art_w, self.canvas.art_h);
        self.export(&mut rdr)?;
        Ok(rdr.to_img())
    }

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

    pub fn preview(&self, rdr: &mut Renderer) -> Result<(), String> {
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
    pub fn export(&self, rdr: &mut Renderer) -> Result<(), String> {
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

/// aseprite file format converter
impl Xprite {
    pub fn as_ase(&self) -> ase::Aseprite {
        let header = ase::Header::new(
            self.canvas.art_w as u16,
            self.canvas.art_h as u16,
        );
        let mut frame = ase::Frame::new();
        for (i, layer) in self.history.top().iter_layers().rev().enumerate() {
            frame.add_chunk(ase::Chunk::new(ase::ChunkData::LayerChunk(
                ase::chunk::LayerChunk::new(layer.name.as_str(), layer.visible),
            )));
            if !layer.content.is_empty() {
                frame.add_chunk(ase::Chunk::new(ase::ChunkData::CelChunk({
                    let Rect(Vec2f { x: x0, y: y0 }, Vec2f { x: x1, y: y1 }) =
                        layer.content.bounding_rect();
                    let w = x1 - x0 + 1.;
                    let h = y1 - y0 + 1.;
                    let pixels: ase::Pixels = layer.content.clone().into();
                    ase::chunk::CelChunk::new(
                        i as u16, x0 as i16, y0 as i16, w as u16, h as u16,
                        pixels,
                    )
                })));
            }
        }
        ase::Aseprite::new(header, vec![frame])
    }

    pub fn from_ase(name: String, aseprite: &ase::Aseprite) -> Self {
        let ase::Aseprite { header, frames } = aseprite;
        let ase::Header {
            width_in_pixels,
            height_in_pixels,
            ..
        } = &header;
        let canvas = Canvas::new(
            f64::from(*width_in_pixels),
            f64::from(*height_in_pixels),
        );
        let mut history = History::empty();

        let frame = &frames[0];
        let ase::Frame { chunks, .. } = frame;
        for ase::Chunk { chunk_data, .. } in chunks {
            match chunk_data {
                ase::ChunkData::LayerChunk(ase::chunk::LayerChunk {
                    flags,
                    layer_type,
                    layer_child_level,
                    blend_mode,
                    opacity,
                    layer_name,
                }) => {
                    if *layer_type == ase::chunk::LayerType::Normal {
                        // image layer
                        history.top_mut().add_layer(Some(layer_name));
                    } else {
                        // group layer
                        history.top_mut().add_group(Some(layer_name));
                    }
                }
                ase::ChunkData::CelChunk(ase::chunk::CelChunk {
                    layer_index,
                    x_position,
                    y_position,
                    opacity_level,
                    cel,
                }) => {
                    let ase_pixs = cel.pixels(&header.color_depth).unwrap();
                    let x = f64::from(*x_position);
                    let y = f64::from(*y_position);
                    let x_ = x + f64::from(cel.w().unwrap() - 1); // TODO: FIXME: off by 1 error from Pixels::bounding_box
                    let y_ = y + f64::from(cel.h().unwrap() - 1);
                    let bb = Rect(Vec2f { x, y }, Vec2f { x: x_, y: y_ });
                    let pixs = Pixels::from_ase_pixels(&ase_pixs, bb);
                    let layer = &mut history.top_mut().groups[0].1
                        [usize::from(*layer_index)];
                    layer.content.extend(&pixs);

                    // dbg!(pixs);
                }
                _ => (),
            };
        }

        let mut xpr = Xprite {
            name,
            canvas,
            history,
            ..Default::default()
        };
        xpr.switch_layer(0, 0);
        xpr
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

impl Xprite {
    pub fn save_layer_img(&self, group_idx: usize, layer_idx: usize, img_path: &str) {
        // todo
    }

    pub fn save_img(&self, img_path: &str, rescale: u32) {
        let mut rdr =
            ImageRenderer::new(self.canvas.art_w, self.canvas.art_h);
        self.export(&mut rdr).unwrap();
        rdr.render();
        let im = rdr.as_img();

        //rescale image

        let nwidth = im.width() * rescale;
        let nheight = im.height() * rescale;
        let filter = img::FilterType::Nearest;
        let im = img::imageops::resize(im, nwidth, nheight, filter);

        info!("writing file to {}", img_path);
        im.save(img_path).unwrap();
    }

    pub fn load_img(png_path: &str) -> Xprite {
        info!("loading png file {}", png_path);
        let img = img::open(png_path).unwrap();
        let (w, h) = img.dimensions();
        let name = Path::new(png_path)
            .file_stem()
            .unwrap()
            .to_str().unwrap()
            .to_owned();
        Xprite::from_img(name, w, h, img)
    }

    pub fn from_img(name: String, w: u32, h: u32, img: img::DynamicImage) -> Xprite {
        let mut xpr = Xprite::new(name, w as f64, h as f64);
        xpr.current_layer_mut().unwrap().content = img.into();
        xpr
    }

    pub fn save_ase(&self, file_path: &str) {
        info!("saving ase file to {}", file_path);
        let mut f = File::create(file_path).unwrap();
        let aseprite = self.as_ase();
        aseprite.write(&mut f).unwrap();
    }

    pub fn load_ase(file_path: &str) -> Xprite {
        info!("loading ase file {}", file_path);
        let mut f = File::open(file_path).unwrap();
        let ase = ase::Aseprite::from_read(&mut f).unwrap();
        let name = Path::new(file_path)
            .file_stem()
            .unwrap()
            .to_str().unwrap()
            .to_owned();
        Xprite::from_ase(name, &ase)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_as_ase() {
        use super::*;
        use std::fs::File;
        let mut xpr = Xprite::new(100., 100.);
        xpr.current_layer_mut().unwrap().content.extend(&pixels!(
            pixel!(0, 0, Color::red()),
            pixel!(0, 1, Color::red())
        ));
        let aseprite = xpr.as_ase();
        let mut f = File::create("test.ase").unwrap();
        aseprite.write(&mut f).unwrap();
        std::fs::remove_file("test.ase").unwrap();
    }

    #[test]
    fn test_as_ase2() {
        use super::*;
        use std::fs::File;
        let mut xpr = Xprite::new(100., 100.);
        xpr.current_layer_mut().unwrap().content.extend(&pixels!(
            pixel!(1, 1, Color::red()),
            pixel!(1, 2, Color::red())
        ));
        let aseprite = xpr.as_ase();
        let mut f = File::create("test2.ase").unwrap();
        aseprite.write(&mut f).unwrap();
        std::fs::remove_file("test2.ase").unwrap();
    }

    #[test]
    fn test_from_ase() {
        use super::*;
        use std::fs::File;
        let fname = "../ase-rs/sample_aseprite_files/simple.aseprite";
        let mut f = File::open(fname).unwrap();
        let mut aseprite = ase::Aseprite::from_read(&mut f).unwrap();
        let _ = Xprite::from_ase(&mut aseprite);
        // dbg!(&xpr.history.top().groups[0].1[0]);
        // dbg!(xpr);
    }

}
