use crate::prelude::*;
use crate::algorithms;

#[derive(Clone)]
pub struct PaintBucket {
    cursor: Option<Pixel>,
}


impl PaintBucket {
    pub fn new() -> Self {
        let cursor = None;
        PaintBucket {
            cursor,
        }
    }

    fn set_cursor(&self, xpr: &mut Xprite) -> Option<()> {
        if self.cursor.is_none() { return None; }
        let cursor = self.cursor.clone().unwrap();
        xpr.set_cursor(cursor.into());
        Some(())
    }

    fn floodfill(&mut self, xpr: &mut Xprite) -> Option<Pixels> {
        let current_layer = xpr.current_layer();
        let pixs = &current_layer.borrow().content;
        let w = xpr.canvas.art_w;
        let h = xpr.canvas.art_h;
        // info!("{:#?}, {:#?},{:#?},{:#?},{:#?},", w, h, pixs, self.cursor?, xpr.color());
        let filled = algorithms::floodfill::floodfill(w, h, pixs, self.cursor?, xpr.color());
        Some(filled)
    }

}

impl Tool for PaintBucket {

    fn tool_type(&self) -> ToolType {
        ToolType::PaintBucket
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Point2D<f32>) -> Option<()> {
        let point = xpr.canvas.shrink_size(&p);
        let color = ColorOption::Unset;
        self.cursor = Some(Pixel {point, color});
        self.draw(xpr);
        Some(())
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Point2D<f32>) -> Option<()> {
        let point = xpr.canvas.shrink_size(&p);
        let color = ColorOption::Unset;
        self.cursor = Some(Pixel {point, color});
        let buffer = self.floodfill(xpr)?;
        info!("mouse: {:#?}", self.cursor);
        info!("{:#?}", buffer);

        xpr.history.enter()?;
        xpr.history.top()
            .selected_layer
            .borrow_mut()
            .content
            .extend(&buffer);
        Some(())
    }


    fn mouse_down(&mut self, xpr: &mut Xprite, p: Point2D<f32>, button: InputItem) -> Option<()> {
        // noop
        Some(())
    }

    fn draw(&self, xpr: &mut Xprite) -> Option<()> {
        xpr.new_frame();
        self.set_cursor(xpr);
        Some(())
    }

    fn set(&mut self, xpr: &mut Xprite, option: &str, value: &str) -> Option<()> {
        match option {
            _ => panic!("unimpl: {}", option)
        }
        Some(())
    }
}
