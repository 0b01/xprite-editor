use xprite::tools::*;
use xprite::lib::algorithms::line::*;

#[derive(Clone)]
pub struct Line {
    is_mouse_down: Option<MouseButton>,
    cursor_pos: Option<Pixel>,
    start_pos: Option<Pixel>,
    snap: bool,
    is_snap_45: bool,
}

impl Line {
    pub fn new() -> Self {
        Line {
            is_mouse_down: None,
            cursor_pos: None,
            start_pos: None,
            snap: false,
            is_snap_45: false,
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

    fn get_line(&self) -> Option<Vec<Pixel>> {
        if let Some(start) = self.start_pos {
        if let Some(stop) = self.cursor_pos {
            if self.snap {
                Some(snapped_line(self.is_snap_45, &start, &stop))
            } else {
                Some(bresenham(&start.point.into(), &stop.point.into()))
            }
        } else {None}} else { None }
    }


    fn finalize_line(&mut self, xpr: &mut Xprite) {
        if let Some(ref mut pixs) = self.get_line() {
            xpr.history.enter();
            let mut pixs = Pixels::from_slice(pixs);
            pixs.set_color(&xpr.color());
            xpr.add_pixels(&pixs);
        }
    }

    fn draw_line(&self, xpr: &Xprite) {
        if let Some(pixs) = self.get_line() {
            for &Pixel{point, color} in pixs.iter() {
                let color = match color {
                    ColorOption::Set(c) => c,
                    ColorOption::Unset => xpr.color(),
                }.to_string();
                xpr.canvas.draw(point.x, point.y, &color);
            }
        }
    }

}

impl Tool for Line {

    fn get_name(&self) -> &'static str {
        "line"
    }

    fn mouse_move(&mut self, xpr: &mut Xprite, p: Point2D<i32>) {
        // set current cursor_pos
        let point = xpr.canvas.client_to_grid(p);
        let color = ColorOption::Set(xpr.color());
        self.cursor_pos = Some(Pixel {point, color});
        self.draw(xpr);
    }

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Point2D<i32>) -> Option<()> {
        let point = xpr.canvas.client_to_grid(p);
        let color = ColorOption::Set(xpr.color());
        self.cursor_pos = Some(Pixel {point, color});
        self.finalize_line(xpr);
        self.is_mouse_down = None;
        self.start_pos = None;
        self.draw(xpr);

        Some(())
    }


    fn mouse_down(&mut self, xpr: &mut Xprite, p: Point2D<i32>, button: MouseButton) {
        self.is_mouse_down = Some(button);
        let point = xpr.canvas.client_to_grid(p);
        let color = ColorOption::Set(xpr.color());
        self.start_pos = Some(Pixel{point, color});
    }

    fn draw(&self, xpr: &Xprite) {
        xpr.canvas.clear_all();
        self.draw_line(xpr);
        for &Pixel{point, color} in xpr.pixels().iter() {
            let color = match color {
                ColorOption::Set(c) => c,
                ColorOption::Unset => xpr.color(),
            }.to_string();
            xpr.canvas.draw(point.x, point.y, &color);
        }
        self.draw_cursor(xpr);
    }

    fn set(&mut self, xpr: &mut Xprite, option: &str, value: &str) {
        match option {
            "ctrl" => {
                match value {
                    "true" => { self.snap = true; self.is_snap_45 = true }
                    "false" => { self.snap = false }
                    _ => console!(error, "unimpl for ctrl: ", value)
                }
                self.draw(xpr);
            }
            "shift" => {
                match value {
                    "true" => { self.snap = true; self.is_snap_45 = false }
                    "false" => { self.snap = false }
                    _ => console!(error, "unimpl for ctrl: ", value)
                }
                self.draw(xpr);
            }
            _ => console!(error, "unimpl: ", option)
        }
    }
}
