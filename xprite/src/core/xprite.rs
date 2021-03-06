use crate::prelude::*;
use crate::rendering::Renderer;
use img::GenericImageView;
use std::cell::RefCell;
use std::fs::File;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

pub struct Xprite {
    pub name: String,
    history: History,

    im_buf: Pixels,
    line_buf: Vec<Rect>,
    pub bz_buf: Vec<CubicBezierSegment>,
    marquee_buf: Vec<MarqueePixel>,

    pub canvas: Canvas,
    pub color_picker_color: Option<Color>,
    pub palette: PaletteManager,

    pub toolbox: Toolbox,
    pub cursor: Pixels,
    pub last_mouse_pos: Vec2f,

    pub log: Arc<Mutex<String>>,

    redraw: bool,
}

impl Default for Xprite {
    fn default() -> Self {
        let palette_man = PaletteManager::new().expect("Cannot initialize palettes");
        Self {
            name: "Untitled".to_owned(),
            palette: palette_man,
            color_picker_color: None,
            history: Default::default(),
            im_buf: Default::default(),
            line_buf: Default::default(),
            bz_buf: Default::default(),
            marquee_buf: Default::default(),
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
        self.set_redraw(true);
    }

    pub fn redo(&mut self) {
        self.history.redo();
        self.set_redraw(true);
    }

    pub fn update_mouse_pos(&mut self, x: f64, y: f64) {
        self.last_mouse_pos.x = x;
        self.last_mouse_pos.y = y;
    }

    pub fn update_lines(&mut self, lines: Vec<Rect>) {
        self.line_buf = lines;
    }

    /// add pixels to temp im_buf
    pub fn add_pixels(&mut self, orig: &Pixels) {
        self.pixels_mut().extend(&orig);
        let reflected = self.toolbox.symmetry.borrow_mut().process(&orig);
        self.pixels_mut().extend(&reflected);
    }

    pub fn finalize_pixels(&mut self, pixs: &Pixels) -> Result<(), String> {
        self.commit();
        let layer = self.cel().unwrap();
        let mut layer = layer.borrow_mut();
        layer.content.extend(&pixs);
        let reflected = self.toolbox.symmetry.borrow_mut().process(&pixs);
        layer.content.extend(&reflected);
        Ok(())
    }

    /// add pixel to temp im_buf
    pub fn add_pixel(&mut self, pixel: Pixel) {
        self.pixels_mut().push(pixel)
    }

    /// remove pixels from temp im_buf
    pub fn remove_pixels(&mut self, pixels: &Pixels) {
        self.pixels_mut().sub_mut(pixels);
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

    pub fn set_option_for_tool(&mut self, name: ToolType, opt: &str, val: &str) -> Result<(), String> {
        let tool = self.toolbox.get(name);
        tool.borrow_mut().set(self, opt, val).unwrap();
        Ok(())
    }

    pub fn change_tool(&mut self, name: ToolType) -> Result<(), String> {
        self.toolbox.change_tool(name);
        self.draw()
    }

    pub fn draw(&mut self) -> Result<(), String> {
        let to_redraw = Rc::clone(&self.toolbox.tool()).borrow_mut().draw(self)?;
        self.set_redraw(to_redraw);
        Ok(())
    }

    pub fn update(&mut self) -> Result<(), String> {
        let mut to_redraw = false;
        to_redraw |= Rc::clone(&self.toolbox.symmetry).borrow_mut().update(self)?;
        // XXX: investigate this call ^
        to_redraw |= self.toolbox.tool().borrow_mut().update(self)?;
        self.set_redraw(to_redraw);
        Ok(())
    }

    pub fn color(&self) -> Color {
        Color::Indexed(self.palette.current_palette().idx)
    }

    pub fn new_frame(&mut self) {
        self.pixels_mut().clear();
        self.bz_buf.clear();
        self.marquee_buf.clear();
    }

    pub fn set_cursor(&mut self, pos: &Pixels) {
        self.cursor = pos.clone();
    }

    pub fn add_marquee(&mut self, marq: &[MarqueePixel]) {
        self.marquee_buf.extend(marq);
    }
}

impl Xprite {
    pub fn frames(&self) -> &Frames {
        self.history.top()
    }

    pub fn frames_mut(&mut self) -> &mut Frames {
        self.history.top_mut()
    }

    pub fn frame(&self) -> &Layers {
        self.history.top().frame()
    }

    pub fn frame_mut(&mut self) -> &mut Layers {
        self.history.top_mut().frame_mut()
    }

    pub fn switch_layer(&mut self, group_id: usize, layer: usize) {
        self.frame_mut().group_idx = group_id;
        self.frame_mut().layer_idx = layer;
    }

    pub fn cel(&self) -> Option<Rc<RefCell<Layer>>> {
        self.history.top().cel()
    }

    pub fn toggle_layer_visibility(&mut self, group: usize, layer: usize) -> Result<(), String> {
        self.commit();
        let frame = self.frame_mut();
        let l = frame
            .groups
            .get_mut(group)
            .ok_or_else(||"no group".to_owned())?
            .1
            .get_mut(layer)
            .ok_or_else(||"no layer".to_owned())?;
        l.borrow_mut().toggle_visible();
        self.set_redraw(true);
        Ok(())
    }

    pub fn remove_layer(&mut self, group: usize, old: usize) -> Result<(), String> {
        self.commit();
        let layers = self.frame_mut();
        layers.layer_idx = 0;
        layers.remove_layer(group, old);

        Ok(())
    }

    pub fn rename_layer(&mut self, name: &str) -> Result<(), String> {
        self.commit();
        let layers = self.frame_mut();
        layers.layer().unwrap().borrow_mut().name = name.to_owned();
        Ok(())
    }

    pub fn get_brush_for_tool(&self, tool_type: ToolType) -> Option<Brush> {
        match tool_type {
            ToolType::Pencil => {
                let tool = self.toolbox.pencil.borrow_mut();
                Some(tool.brush.clone())
            }
            ToolType::Vector => {
                let tool = self.toolbox.vector.borrow_mut();
                Some(tool.brush.clone())
            }
            ToolType::Eraser => {
                let tool = self.toolbox.eraser.borrow_mut();
                Some(tool.brush.clone())
            }
            _ => None,
        }
    }
    pub fn set_brush_for_tool(&self, tool_type: ToolType, brush: Brush) -> Result<(), String> {
        match tool_type {
            ToolType::Pencil => {
                let mut tool = self.toolbox.pencil.borrow_mut();
                tool.brush = brush;
                Ok(())
            }
            ToolType::Vector => {
                let mut tool = self.toolbox.vector.borrow_mut();
                tool.brush = brush;
                Ok(())
            }
            ToolType::Eraser => {
                let mut tool = self.toolbox.eraser.borrow_mut();
                tool.brush = brush;
                Ok(())
            }
            _ => Err("No brush attached to tool".to_owned()),
        }
    }

    pub fn last_tool(&self) -> ToolType {
        *self.toolbox.tool_stack.last().unwrap()
    }
}

impl Xprite {
    pub fn render(&self, rdr: &mut dyn Renderer) {
        self.render_cursor(rdr);
        self.render_bezier(rdr);
        self.render_line(rdr);
        self.render_marquee(rdr);
        self.render_canvas_extras(rdr);
    }

    pub fn render_line(&self, rdr: &mut dyn Renderer) {
        for Rect(p0, p1) in &self.line_buf {
            self.canvas.draw_line(rdr, *p0, *p1, XpriteRgba::red().into());
        }
    }

    pub fn render_cursor(&self, rdr: &mut dyn Renderer) -> Option<()> {
        for p in self.cursor.iter() {
            let c = p.color.to_rgba(Some(self))?.into();
            self.canvas.draw_pixel_rect(rdr, p.point, c, true);
        }
        let outline = self.cursor.outline();
        for (point, outline) in outline.iter() {
            self.canvas.draw_pixel_outline(rdr, *point, *outline);
        }
        Some(())
    }

    pub fn render_canvas_extras(&self, rdr: &mut dyn Renderer) {
        rdr.reset();
        // self.canvas.draw_canvas(rdr);
        self.canvas.draw_grid(rdr);
    }

    pub fn render_bezier(&self, rdr: &mut dyn Renderer) {
        for seg in &self.bz_buf {
            let &CubicBezierSegment { ctrl1, ctrl2, from, to } = seg;
            self.canvas.draw_bezier(rdr, from, ctrl1, ctrl2, to, XpriteRgba::grey().into(), 1.);
            let red = XpriteRgba::red().into();
            let blue = XpriteRgba::blue().into();
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

    pub fn render_marquee(&self, rdr: &mut dyn Renderer) {
        for (ith, (p, outline)) in self.marquee_buf.iter().enumerate() {
            self.canvas.draw_pixel_marqee(rdr, *p, *outline, ith);
        }
    }
}

impl Xprite {
    pub fn as_img(&self) -> Result<img::DynamicImage, String> {
        let mut rdr = ImageRenderer::new(self.canvas.bg, self.canvas.art_w, self.canvas.art_h);
        self.export(&mut rdr)?;
        Ok(rdr.into_img())
    }

    pub fn selected_layer_as_im(&self) -> Option<img::DynamicImage> {
        let l = self.history.top().cel().unwrap();
        let layer = l.borrow();
        let mut rdr = ImageRenderer::new(self.canvas.bg, self.canvas.art_w, self.canvas.art_h);
        layer.draw(&mut rdr, Some(self));
        rdr.render(Some(self))?;
        Some(rdr.image)
    }

    pub fn layer_as_im(&self, group_idx: usize, layer_idx: usize, trim: bool) -> Option<img::DynamicImage> {
        let layer = &self.history.top().frame().groups[group_idx].1[layer_idx].borrow();
        if trim {
            let bb = layer.content.bounding_rect();
            return layer.content.as_image(bb, Some(self));
        }
        let mut rdr = ImageRenderer::new(self.canvas.bg, self.canvas.art_w, self.canvas.art_h);
        layer.draw(&mut rdr, Some(self));
        rdr.render(Some(self))?;
        Some(rdr.image)
    }

    pub fn group_as_im(&self, group_idx: usize, trim: bool) -> Option<img::DynamicImage> {
        let group = &self.history.top().frame().groups[group_idx].1;
        if trim {
            let mut content = Pixels::new();
            for i in group.iter() {
                content.extend(&i.borrow().content);
            }
            let bb = content.bounding_rect();
            return content.as_image(bb, Some(self));
        }
        let mut rdr = ImageRenderer::new(self.canvas.bg, self.canvas.art_w, self.canvas.art_h);
        for layer in group.iter() {
            layer.borrow().draw(&mut rdr, Some(self));
        }
        rdr.render(Some(self))?;
        Some(rdr.image)
    }

    pub fn preview(&self, rdr: &mut dyn Renderer) -> Result<(), String> {
        for (i, group) in self.frame().groups.iter().enumerate().rev() {
            for (j, layer) in group.1.iter().enumerate().rev() {
                let draw_buf = |rdr: &mut dyn Renderer| -> Result<(), String> {
                    if i == self.frame().group_idx && j == self.frame().layer_idx {
                        // draw current layer pixels
                        for &Pixel { point, color } in self.pixels().iter() {
                            let Vec2f { x, y } = point;
                            // println!("{:?}", color);
                            let c = color.to_rgba(Some(self)).ok_or_else(||"color index too big".to_owned())?.into();
                            rdr.pixel(x, y, c, true);
                        }
                    }
                    Ok(())
                };
                if !layer.borrow().visible {
                    draw_buf(rdr)?;
                    continue;
                } else {
                    layer.borrow().draw(rdr, Some(self));
                    draw_buf(rdr)?;
                }
            }
        }

        Ok(())
    }

    /// export pixels to an image via renderer
    pub fn export(&self, rdr: &mut dyn Renderer) -> Result<(), String> {
        for layer in self.frame().iter_layers().rev() {
            // draw layers
            let layer = layer.borrow();
            // skip invisible layers
            if !layer.visible {
                continue;
            }
            layer.draw(rdr, Some(self));
        }
        Ok(())
    }
}

/// aseprite file format converter
impl Xprite {
    pub fn as_ase(&self) -> Option<ase::Aseprite> {
        let header = ase::Header::new(self.canvas.art_w as u16, self.canvas.art_h as u16);
        let mut frame = ase::Frame::new();
        for (i, layer) in self.history.top().frame().iter_layers().rev().enumerate() {
            let layer = layer.borrow();
            frame.add_chunk(ase::Chunk::new(ase::ChunkData::LayerChunk(ase::chunk::LayerChunk::new(
                layer.name.as_str(),
                layer.visible,
            ))));
            if !layer.content.is_empty() {
                frame.add_chunk(ase::Chunk::new(ase::ChunkData::CelChunk({
                    let Rect(Vec2f { x: x0, y: y0 }, Vec2f { x: x1, y: y1 }) = layer.content.bounding_rect();
                    let w = x1 - x0 + 1.;
                    let h = y1 - y0 + 1.;
                    let pixels: ase::Pixels = layer.content.clone().to_ase_pixels(Some(self))?;
                    ase::chunk::CelChunk::new(i as u16, x0 as i16, y0 as i16, w as u16, h as u16, pixels)
                })));
            }
        }
        Some(ase::Aseprite::new(header, vec![frame]))
    }

    pub fn from_ase(name: String, aseprite: &ase::Aseprite) -> Self {
        let ase::Aseprite { header, frames } = aseprite;
        let ase::Header {
            width_in_pixels,
            height_in_pixels,
            ..
        } = &header;
        let canvas = Canvas::new(f64::from(*width_in_pixels), f64::from(*height_in_pixels));
        let mut history = History::new();

        let frame = &frames[0];
        let ase::Frame { chunks, .. } = frame;
        for ase::Chunk { chunk_data, .. } in chunks {
            match chunk_data {
                ase::ChunkData::LayerChunk(ase::chunk::LayerChunk {
                    flags,
                    layer_type,
                    layer_name,
                    ..
                }) => {
                    let visible = flags.contains(ase::chunk::layer_chunk::Flags::Visible);
                    if *layer_type == ase::chunk::LayerType::Normal {
                        // image layer
                        history.top_mut().frame_mut().add_layer(Some(layer_name), visible);
                    } else {
                        // group layer
                        history.top_mut().frame_mut().add_group(Some(layer_name));
                    }
                }
                ase::ChunkData::CelChunk(ase::chunk::CelChunk {
                    layer_index,
                    x_position,
                    y_position,
                    cel,
                    ..
                }) => {
                    let ase_pixs = cel.pixels(header.color_depth).unwrap();
                    let x = f64::from(*x_position);
                    let y = f64::from(*y_position);
                    let x_ = x + f64::from(cel.w().unwrap() - 1); // TODO: FIXME: off by 1 error from Pixels::bounding_box
                    let y_ = y + f64::from(cel.h().unwrap() - 1);
                    let bb = Rect(Vec2f { x, y }, Vec2f { x: x_, y: y_ });
                    let pixs = Pixels::from_ase_pixels(&ase_pixs, bb);
                    let layer = &mut history.top_mut().frame_mut().groups[0].1[usize::from(*layer_index)].borrow_mut();
                    layer.content.extend(&pixs);

                    // dbg!(pixs);
                }
                _ => (),
            };
        }

        history.top_mut().frame_mut().groups[0].1.reverse();

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
            KeyUp { key } => self.key_up(*key),
            KeyDown { key } => self.keys_down(*key),
        }
    }

    pub fn key_up(&mut self, key: InputItem) -> Result<(), String> {
        self.set_option(key.as_str(), "false")
    }

    pub fn keys_down(&mut self, key: InputItem) -> Result<(), String> {
        self.set_option(key.as_str(), "true")
    }

    pub fn mouse_move(&mut self, evt: &InputEvent) -> Result<(), String> {
        if let InputEvent::MouseMove { x, y } = evt {
            let p = Vec2f { x: *x, y: *y };
            let tool = self.toolbox.tool();
            tool.borrow_mut().mouse_move(self, p)?;
        }
        Ok(())
    }

    pub fn mouse_up(&mut self, evt: &InputEvent) -> Result<(), String> {
        if let InputEvent::MouseUp { x, y, .. } = evt {
            let tool = self.toolbox.tool();
            let p = Vec2f { x: *x, y: *y };
            tool.borrow_mut().mouse_up(self, p)?;
        }
        Ok(())
    }

    pub fn mouse_down(&mut self, evt: &InputEvent) -> Result<(), String> {
        if let InputEvent::MouseDown { x, y, button } = evt {
            let tool = self.toolbox.tool();
            let p = Vec2f { x: *x, y: *y };
            tool.borrow_mut().mouse_down(self, p, *button)?;
        }
        Ok(())
    }
}

impl Xprite {
    pub fn commit(&mut self) {
        self.history.duplicate();
        self.history.clear_redo();
    }
}

/// layers
impl Xprite {
    pub fn redraw(&self) -> bool {
        self.redraw
    }

    pub unsafe fn override_redraw(&mut self, redraw: bool) {
        self.redraw = redraw;
    }

    pub fn set_redraw(&mut self, redraw: bool) {
        self.redraw |= redraw;
    }

    pub fn get_layer(&self, group_id: usize, layer_id: usize) -> Rc<RefCell<Layer>> {
        Rc::clone(&self.frame().groups[group_id].1[layer_id])
    }

    pub fn swap_layer(&mut self, prev: usize, next: usize) {
        self.frame_mut().swap_layer(prev, next);
    }

    pub fn swap_group(&mut self, prev: usize, next: usize) {
        self.frame_mut().swap_group(prev, next);
    }
}

/// import/export
impl Xprite {
    pub fn save_layer_img<P: AsRef<Path>>(&self, group_idx: usize, layer_idx: usize, img_path: P, rescale: u32, trim: bool) -> Option<()> {
        let im = self.layer_as_im(group_idx, layer_idx, trim)?;
        let nwidth = im.width() * rescale;
        let nheight = im.height() * rescale;
        let filter = img::FilterType::Nearest;
        let im = img::imageops::resize(&im, nwidth, nheight, filter);

        info!("writing file to {:?}", img_path.as_ref().as_os_str());
        im.save(img_path).unwrap();
        Some(())
    }

    pub fn save_group_img<P: AsRef<Path>>(&self, group_idx: usize, img_path: P, rescale: u32, trim: bool) -> Option<()> {
        let im = self.group_as_im(group_idx, trim)?;
        let nwidth = im.width() * rescale;
        let nheight = im.height() * rescale;
        let filter = img::FilterType::Nearest;
        let im = img::imageops::resize(&im, nwidth, nheight, filter);
        info!("writing file to {:?}", img_path.as_ref().as_os_str());
        im.save(img_path).unwrap();
        Some(())
    }

    pub fn save_img<P: AsRef<Path>>(&self, img_path: P, rescale: u32) -> Option<()> {
        let mut rdr = ImageRenderer::new(self.canvas.bg, self.canvas.art_w, self.canvas.art_h);
        self.export(&mut rdr).unwrap();
        rdr.render(Some(self))?;
        let im = rdr.as_img();
        //rescale image
        let nwidth = im.width() * rescale;
        let nheight = im.height() * rescale;
        let filter = img::FilterType::Nearest;
        let im = img::imageops::resize(im, nwidth, nheight, filter);
        info!("writing file to {:?}", img_path.as_ref().as_os_str());
        im.save(img_path).unwrap();
        Some(())
    }

    pub fn load_img<P: AsRef<Path>>(png_path: P) -> Xprite {
        info!("loading png file {:?}", png_path.as_ref().as_os_str());
        let img = img::open(png_path.as_ref()).unwrap();
        let (w, h) = img.dimensions();
        let name = png_path.as_ref().file_stem().unwrap().to_str().unwrap().to_owned();
        Xprite::from_img(name, w, h, img)
    }

    pub fn from_img(name: String, w: u32, h: u32, img: img::DynamicImage) -> Xprite {
        let xpr = Xprite::new(name, f64::from(w), f64::from(h));
        xpr.cel().unwrap().borrow_mut().content = img.into();
        xpr
    }

    pub fn save_ase<P: AsRef<Path>>(&self, file_path: P) -> Option<()> {
        info!("saving ase file to {:?}", file_path.as_ref().as_os_str());
        let mut f = File::create(file_path).unwrap();
        let aseprite = self.as_ase()?;
        aseprite.write(&mut f).unwrap();
        Some(())
    }

    pub fn load_ase<P: AsRef<Path>>(file_path: P) -> Xprite {
        info!("loading ase file {:?}", file_path.as_ref().as_os_str());
        let mut f = File::open(file_path.as_ref()).unwrap();
        let ase = ase::Aseprite::from_read(&mut f).unwrap();
        let name = file_path.as_ref().file_stem().unwrap().to_str().unwrap().to_owned();
        Xprite::from_ase(name, &ase)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_as_ase() {
        use super::*;
        use std::fs::File;
        let xpr = Xprite::new("test".to_owned(), 100., 100.);
        xpr.cel()
            .unwrap()
            .borrow_mut()
            .content
            .extend(&pixels!(pixel!(0, 0, Color::red()), pixel!(0, 1, Color::red())));
        let aseprite = xpr.as_ase().unwrap();
        let mut f = File::create("test.ase").unwrap();
        aseprite.write(&mut f).unwrap();
        std::fs::remove_file("test.ase").unwrap();
    }

    #[test]
    fn test_as_ase2() {
        use super::*;
        use std::fs::File;
        let xpr = Xprite::new("test".to_owned(), 100., 100.);
        xpr.cel()
            .unwrap()
            .borrow_mut()
            .content
            .extend(&pixels!(pixel!(1, 1, Color::red()), pixel!(1, 2, Color::red())));
        let aseprite = xpr.as_ase().unwrap();
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
        let _ = Xprite::from_ase("test".to_owned(), &mut aseprite);
        // dbg!(&xpr.history.top().groups[0].1[0]);
        // dbg!(xpr);
    }

}
