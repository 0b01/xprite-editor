use xprite::tools::Tool;
use xprite::*;

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

    fn draw_cursor(&self, xpr: &Xprite) {
        if let Some(pix) = self.cursor_pos {
            xpr.canvas.draw(
                pix.point.x,
                pix.point.y,
                &Color::red().to_string()
            );
        }
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

    fn mouse_move(&mut self, xpr: &mut Xprite, x: i32, y: i32) {
        let point = xpr.canvas.client_to_grid(x, y);
        let color = ColorOption::Set(xpr.color());
        self.cursor_pos = Some(Pixel {point, color});
        self.draw(xpr);
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, x: i32, y: i32) {
        let point = xpr.canvas.client_to_grid(x, y);
        let color = ColorOption::Set(xpr.color());
        self.cursor_pos = Some(Pixel {point, color});
        self.floodfill(xpr);
        self.draw(xpr);
    }


    fn mouse_down(&mut self, xpr: &mut Xprite, x: i32, y: i32, button: MouseButton) {
        let point = xpr.canvas.client_to_grid(x, y);
        let color = Some(xpr.color());
    }

    fn draw(&self, xpr: &Xprite) {
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
    }

    fn set(&mut self, xpr: &mut Xprite, option: &str, value: &str) {
        match option {
            _ => console!(error, "unimpl: ", option)
        }
    }
}
