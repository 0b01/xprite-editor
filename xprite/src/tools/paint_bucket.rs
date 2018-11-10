use crate::prelude::*;

#[derive(Clone)]
pub struct PaintBucket {
    cursor_pos: Option<Pixel>,
}


impl PaintBucket {

    pub fn new() -> Self {
        let cursor_pos = None;
        PaintBucket {
            cursor_pos,
        }
    }

    fn draw_cursor(&self, xpr: &mut Xprite) -> Option<()> {
        if let Some(pix) = self.cursor_pos {
            xpr.add_pixel(pix);
        }
        Some(())
    }

    fn floodfill(&self, xpr: &mut Xprite) {
        if let Some(Pixel {point, ..}) = self.cursor_pos {
            let pixels = xpr.pixels();
            unimplemented!()
        }
    }

}

impl Tool for PaintBucket {

    fn tool_type(&self) -> ToolType {
        ToolType::PaintBucket
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Point2D<f32>) -> Option<()> {
        let point = xpr.canvas.shrink_size(&p);
        let color = ColorOption::Set(xpr.color());
        self.cursor_pos = Some(Pixel {point, color});
        self.draw(xpr);
        Some(())
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Point2D<f32>) -> Option<()> {
        let point = xpr.canvas.shrink_size(&p);
        let color = ColorOption::Set(xpr.color());
        self.cursor_pos = Some(Pixel {point, color});
        self.floodfill(xpr);
        self.draw(xpr);

        Some(())
    }


    fn mouse_down(&mut self, xpr: &mut Xprite, p: Point2D<f32>, button: InputItem) -> Option<()> {
        let point = xpr.canvas.shrink_size(&p);
        let color = Some(xpr.color());
        Some(())
    }

    fn draw(&self, xpr: &mut Xprite) -> Option<()> {
        // xpr.canvas.clear_all();
        self.draw_cursor(xpr);
        Some(())
    }

    fn set(&mut self, xpr: &mut Xprite, option: &str, value: &str) -> Option<()> {
        match option {
            _ => panic!("unimpl: {}", option)
        }
        Some(())
    }
}
