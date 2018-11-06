use xprite::tools::*;

use std::f32::consts::PI;
use std::f32;

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

    fn get_line(&self) -> Option<Pixels> {
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
            xpr.history.new_history_frame();
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

    fn mouse_up(&mut self, xpr: &mut Xprite, p: Point2D<i32>) {
        let point = xpr.canvas.client_to_grid(p);
        let color = ColorOption::Set(xpr.color());
        self.cursor_pos = Some(Pixel {point, color});
        self.finalize_line(xpr);
        self.is_mouse_down = None;
        self.start_pos = None;
        self.draw(xpr);
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

fn snapped_line(is_45: bool, start: &Pixel, stop: &Pixel) -> Pixels {
    let mut ret = Pixels::new();

    let x0 = start.point.x as f32;
    let y0 = start.point.y as f32;
    let x1 = stop.point.x as f32;
    let y1 = stop.point.y as f32;

    let dx = x1 - x0;
    let dy = y1 - y0;

    let theta = f32::atan2(dy, dx);

    if !is_45 {
        let dir = ((theta / (2. * PI / 12.)).round() + 12.) % 12.;

        let dy = dy.abs() as i32;
        let dx = dx.abs() as i32;
        match dir as i32 {
            0 => { for i in 0..dx { ret.insert(pixel!(x0+i as f32, y0))} }
            1 => {
                let dx = (dx as f32 * 1.1).ceil() as i32;
                for i in (0..dx).step_by(2) {
                    ret.insert(pixel!(x0+i as f32, y0 + i as f32/2.));
                    ret.insert(pixel!(x0 + 1. + i as f32, y0 + i as f32/2.));
                }
            }
            2 => {
                let dy = (dy as f32 * 1.1).ceil() as i32;
                for i in (0..dy).step_by(2) {
                    ret.insert(pixel!(x0+i as f32/2., y0 + i as f32));
                    ret.insert(pixel!(x0+i as f32/2., y0 + 1. + i as f32));
                }
            }
            3 => { for i in 0..dy { ret.insert(pixel!(x0, y0+i as f32))} }
            4 => {
                let dy = (dy as f32 * 1.1).ceil() as i32;
                for i in (0..dy).step_by(2) {
                    ret.insert(pixel!(x0-i as f32/2., y0 + i as f32));
                    ret.insert(pixel!(x0-i as f32/2., y0 + 1. + i as f32));
                }
            }
            5 => {
                let dx = (dx as f32 * 1.1).ceil() as i32;
                for i in (0..dx).step_by(2) {
                    ret.insert(pixel!(x0-i as f32, y0 + i as f32/2.));
                    ret.insert(pixel!(x0 - 1. - i as f32, y0 + i as f32/2.));
                }
            }
            6 => { for i in 0..dx { ret.insert(pixel!(x0-i as f32, y0))} }
            7 => {
                let dx = (dx as f32 * 1.1).ceil() as i32;
                for i in (0..dx).step_by(2) {
                    ret.insert(pixel!(x0-i as f32, y0 - i as f32/2.));
                    ret.insert(pixel!(x0 - 1. - i as f32, y0 - i as f32/2.));
                }
            }
            8 => {
                let dy = (dy as f32 * 1.1).ceil() as i32;
                for i in (0..dy).step_by(2) {
                    ret.insert(pixel!(x0-i as f32/2., y0 - i as f32));
                    ret.insert(pixel!(x0-i as f32/2., y0 - 1. - i as f32));
                }
            }
            9 => { for i in 0..dy { ret.insert(pixel!(x0, y0 - i as f32))} }
            10 => {
                let dy = (dy as f32 * 1.1).ceil() as i32;
                for i in (0..dy).step_by(2) {
                    ret.insert(pixel!(x0+i as f32/2., y0 - i as f32));
                    ret.insert(pixel!(x0+i as f32/2., y0 - 1. - i as f32));
                }
            }
            11 => {
                let dx = (dx as f32 * 1.1).ceil() as i32;
                for i in (0..dx).step_by(2) {
                    ret.insert(pixel!(x0+i as f32, y0 - i as f32/2.));
                    ret.insert(pixel!(x0 + 1. + i as f32, y0 - i as f32/2.));
                }
            }
            _ => ()
        }
    } else {
        let dir = ((theta / (2. * PI / 8.)).round() + 8.) % 8.;

        let dy = dy.abs() as i32;
        let dx = dx.abs() as i32;
        match dir as i32 {
            0 => { for i in 0..dx { ret.insert(pixel!(x0+i as f32, y0))} }
            1 => { for i in 0..dy { ret.insert(pixel!(x0+i as f32, y0 + i as f32))} }
            2 => { for i in 0..dy { ret.insert(pixel!(x0, y0+i as f32))} }
            3 => { for i in 0..dy { ret.insert(pixel!(x0-i as f32, y0 + i as f32))} }
            4 => { for i in 0..dx { ret.insert(pixel!(x0-i as f32, y0))} }
            5 => { for i in 0..dx { ret.insert(pixel!(x0-i as f32, y0 - i as f32))} }
            6 => { for i in 0..dy { ret.insert(pixel!(x0, y0 - i as f32))} }
            7 => { for i in 0..dx { ret.insert(pixel!(x0+i as f32, y0 - i as f32))} }
            _ => console!(error, "impossible")
        }
    }

    ret
}

pub fn bresenham(start: &Point2D<u32>, stop: &Point2D<u32>) -> Pixels {
    let mut ret = Pixels::new();
    let mut x0 = start.x as i32;
    let mut y0 = start.y as i32;
    let x1 = stop.x as i32;
    let y1 = stop.y as i32;

    let dx = (x1-x0).abs();
    let sx = if x0<x1 {1} else {-1};
    let dy = -(y1-y0).abs();
    let sy = if y0<y1 {1} else {-1};
    let mut err = dx+dy; /* error value e_xy */
    loop {
        ret.insert(pixel!(x0, y0));
        if x0==x1 && y0==y1 { break; }
        let e2 = 2 * err;
        if e2 >= dy { err += dy; x0 += sx; } /* e_xy+e_x > 0 */
        if e2 <= dx { err += dx; y0 += sy; } /* e_xy+e_y < 0 */
    }
    ret
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_adjust() {
        let adjusted_end = snapped_line(true, &pixel!(0, 0), &pixel!(10, 9));
        println!("{:?}", adjusted_end);
    }
}
