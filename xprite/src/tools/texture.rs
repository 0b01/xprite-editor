use crate::tools::*;
use crate::algorithms::rect::*;
use libtexsyn::{
    distance::l1,
    generators::patch::{Quilter, QuilterParams},
};

#[derive(Clone, Default)]
pub struct Texture {
    is_mouse_down: Option<InputItem>,
    cursor_pos: Option<Pixel>,
    start_pos: Option<Pixel>,
    pub blocksize: i32,
    pub overlap: i32,
}

impl Texture {
    pub fn new() -> Self {
        Texture {
            is_mouse_down: None,
            start_pos: None,
            cursor_pos: None,
            blocksize: 12,
            overlap: 6,
        }
    }

    fn set_cursor(&self, xpr: &mut Xprite) -> Option<()> {
        if let Some(pix) = self.cursor_pos {
            let c = pixel!(pix.point.x, pix.point.y, Color::red());
            let mut pixels = Pixels::new();
            pixels.push(c);
            xpr.set_cursor(&pixels);
        }
        Some(())
    }

    pub fn finalize(&mut self, xpr: &mut Xprite) -> Option<()> {
        self.finalize_line(xpr)?;
        Some(())
    }

    fn finalize_line(&mut self, xpr: &mut Xprite) -> Option<()> {
        if let Some(mut pixs) = get_rect(self.start_pos, self.cursor_pos, true) {
            xpr.history.enter()?;
            pixs.set_color(&xpr.color());
            let content = &mut xpr.history.top().selected_layer.borrow_mut().content;
            let intersection = content.intersection(&pixs);
            let (x,y, origin) = {
                let x0 = self.start_pos.unwrap().point.x;
                let y0 = self.start_pos.unwrap().point.y;
                let x1 = self.cursor_pos.unwrap().point.x;
                let y1 = self.cursor_pos.unwrap().point.y;
                (
                    (x1-x0).abs(),
                    (y1-y0).abs(),
                    (f32::min(x0, x1), f32::min(y0,y1))
                )
            };
            let img = intersection.as_image(x, y, origin);

            let width = xpr.canvas.art_w as u32;
            let height = xpr.canvas.art_h as u32;
            let params = QuilterParams::new((width, height), self.blocksize as u32, self.overlap as u32, None, None, l1).unwrap();
            let mut quilter = Quilter::new(img.to_rgb(), params);
            let res = quilter.quilt_image().unwrap();
            res.save("1.png").unwrap();
            // xpr.history.top().selected_layer.borrow_mut().content.extend(&pixs);
        }
        Some(())
    }

    fn draw_line(&self, xpr: &mut Xprite) -> Option<()> {
        if let Some(mut pixs) = get_rect(self.start_pos, self.cursor_pos, false) {
            pixs.set_color(&xpr.color());
            xpr.add_pixels(&pixs)
        }
        Some(())
    }

}

impl Tool for Texture {

    fn tool_type(&self) -> ToolType {
        ToolType::Texture
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Vec2D) -> Option<()> {
        // set current cursor_pos
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        if self.is_mouse_down.is_some() {
            self.cursor_pos = Some(Pixel {point, color});
        }
        self.draw(xpr);
        Some(())
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Vec2D) -> Option<()> {
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.cursor_pos = Some(Pixel {point, color});
        // self.finalize_line(xpr)?;

        self.is_mouse_down = None;
        // self.start_pos = None;

        self.draw(xpr);
        Some(())
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, p: Vec2D, button: InputItem) -> Option<()> {
        if InputItem::Left != button { return Some(()); }
        self.is_mouse_down = Some(button);
        let point = xpr.canvas.shrink_size(p);
        let color = xpr.color();
        self.start_pos = Some(Pixel{point, color});
        Some(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Option<()> {
        xpr.new_frame();
        self.draw_line(xpr);
        self.set_cursor(xpr);
        Some(())
    }

    fn set(&mut self, xpr: &mut Xprite, option: &str, value: &str) -> Option<()> {
        match option {
            "ctrl" => {
                match value {
                    _ => error!("unimpl for ctrl: {}", value)
                }
                self.draw(xpr);
            }
            "shift" => {
                match value {
                    _ => error!("unimpl for ctrl: {}", value)
                }
                self.draw(xpr);
            }
            "alt" => {
                info!("alt pressed (unimplemented)");
            }
            _ => info!("unimplemented option: {}", option)
        }
        Some(())
    }


}
