use crate::prelude::*;
use crate::algorithms;

const MAX_CURSOR_NUM: usize = 256;

#[derive(Clone)]
pub struct PaintBucket { }

impl PaintBucket {
    pub fn new() -> Self {
        PaintBucket { }
    }

    pub fn floodfill(&self, xpr: &Xprite, p: Pixel) -> Option<Pixels> {
        let current_layer = xpr.current_layer();
        let pixs = &current_layer.borrow().content;
        let w = xpr.canvas.art_w;
        let h = xpr.canvas.art_h;
        // info!("{:#?}, {:#?},{:#?},{:#?},{:#?},", w, h, pixs, self.cursor?, xpr.color());
        info!("Filling: {:?}", xpr.color());
        let buffer = algorithms::floodfill::floodfill(w, h, pixs, p, xpr.color());
        info!{"{:#?}", buffer};
        Some(buffer)
    }

}

impl Tool for PaintBucket {

    fn tool_type(&self) -> ToolType {
        ToolType::PaintBucket
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Point2D<f32>) -> Option<()> {
        let point = xpr.canvas.shrink_size(&p);
        let color = xpr.color();
        xpr.set_cursor(&(Pixel {point, color}).into());
        self.draw(xpr);
        Some(())
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Point2D<f32>) -> Option<()> {
        let point = xpr.canvas.shrink_size(&p);
        let color = xpr.color();
        let p = Pixel {point, color};
        let buffer = self.floodfill(xpr, p)?;

        xpr.history.enter()?;
        xpr.history.top()
            .selected_layer
            .borrow_mut()
            .content
            .extend(&buffer);
        Some(())
    }

    fn mouse_down(&mut self, xpr: &mut Xprite, p: Point2D<f32>, _button: InputItem) -> Option<()> {
        let point = xpr.canvas.shrink_size(&p);
        let color = xpr.color();
        let p = Pixel {point, color};
        let buffer = self.floodfill(xpr, p)?;
        if buffer.len() > MAX_CURSOR_NUM {
            let w = xpr.canvas.art_w;
            let h = xpr.canvas.art_h;
            xpr.set_cursor(&algorithms::perimeter::find_perimeter(w as usize, h as usize, &buffer));
        } else {
            xpr.set_cursor(&buffer);
        }

        Some(())
    }

    fn draw(&mut self, xpr: &mut Xprite) -> Option<()> {
        xpr.new_frame();
        // noop
        Some(())
    }

    fn set(&mut self, _xpr: &mut Xprite, option: &str, _value: &str) -> Option<()> {
        match option {
            _ => (), // noop
        }
        Some(())
    }
}
