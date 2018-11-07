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

    fn draw_cursor(&self, xpr: &Xprite) -> Option<()> {
        if let Some(pix) = self.cursor_pos {
            xpr.canvas.draw(
                pix.point.x,
                pix.point.y,
                &Color::red().to_string()
            );
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

    fn get_name(&self) -> &'static str {
        "paint_bucket"
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Point2D<i32>) -> Option<()> {
        let point = xpr.canvas.client_to_grid(p);
        let color = ColorOption::Set(xpr.color());
        self.cursor_pos = Some(Pixel {point, color});
        self.draw(xpr);
        Some(())
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Point2D<i32>) -> Option<()> {
        let point = xpr.canvas.client_to_grid(p);
        let color = ColorOption::Set(xpr.color());
        self.cursor_pos = Some(Pixel {point, color});
        self.floodfill(xpr);
        self.draw(xpr);

        Some(())
    }


    fn mouse_down(&mut self, xpr: &mut Xprite, p: Point2D<i32>, button: MouseButton) -> Option<()> {
        let point = xpr.canvas.client_to_grid(p);
        let color = Some(xpr.color());
        Some(())
    }

    fn draw(&self, xpr: &Xprite) -> Option<()> {
        xpr.canvas.clear_all();
        for &Pixel{point, color} in xpr.pixels().iter() {
            let color = match color {
                ColorOption::Unset =>
                    xpr.color(),
                ColorOption::Set(c) =>
                    c
            }.to_string();
            xpr.canvas.draw(point.x, point.y, &color);
        }
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
