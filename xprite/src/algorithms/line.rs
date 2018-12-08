use crate::prelude::*;
use std::f32::consts::PI;
use std::f32;

pub fn snapped_line(is_45: bool, start: &Pixel, stop: &Pixel) -> Vec<Pixel> {
    let mut ret = Vec::new();

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
            0 => { for i in 0..dx { ret.push(pixel!(x0+i as f32, y0, Color::red()))} }
            1 => {
                let dx = (dx as f32 * 1.1).ceil() as i32;
                for i in (0..dx).step_by(2) {
                    ret.push(pixel!(x0+i as f32, y0 + i as f32/2., Color::red()));
                    ret.push(pixel!(x0 + 1. + i as f32, y0 + i as f32/2., Color::red()));
                }
            }
            2 => {
                let dy = (dy as f32 * 1.1).ceil() as i32;
                for i in (0..dy).step_by(2) {
                    ret.push(pixel!(x0+i as f32/2., y0 + i as f32, Color::red()));
                    ret.push(pixel!(x0+i as f32/2., y0 + 1. + i as f32, Color::red()));
                }
            }
            3 => { for i in 0..dy { ret.push(pixel!(x0, y0+i as f32, Color::red()))} }
            4 => {
                let dy = (dy as f32 * 1.1).ceil() as i32;
                for i in (0..dy).step_by(2) {
                    ret.push(pixel!(x0-i as f32/2., y0 + i as f32, Color::red()));
                    ret.push(pixel!(x0-i as f32/2., y0 + 1. + i as f32, Color::red()));
                }
            }
            5 => {
                let dx = (dx as f32 * 1.1).ceil() as i32;
                for i in (0..dx).step_by(2) {
                    ret.push(pixel!(x0-i as f32, y0 + i as f32/2., Color::red()));
                    ret.push(pixel!(x0 - 1. - i as f32, y0 + i as f32/2., Color::red()));
                }
            }
            6 => { for i in 0..dx { ret.push(pixel!(x0-i as f32, y0, Color::red()))} }
            7 => {
                let dx = (dx as f32 * 1.1).ceil() as i32;
                for i in (0..dx).step_by(2) {
                    ret.push(pixel!(x0-i as f32, y0 - i as f32/2., Color::red()));
                    ret.push(pixel!(x0 - 1. - i as f32, y0 - i as f32/2., Color::red()));
                }
            }
            8 => {
                let dy = (dy as f32 * 1.1).ceil() as i32;
                for i in (0..dy).step_by(2) {
                    ret.push(pixel!(x0-i as f32/2., y0 - i as f32, Color::red()));
                    ret.push(pixel!(x0-i as f32/2., y0 - 1. - i as f32, Color::red()));
                }
            }
            9 => { for i in 0..dy { ret.push(pixel!(x0, y0 - i as f32, Color::red()))} }
            10 => {
                let dy = (dy as f32 * 1.1).ceil() as i32;
                for i in (0..dy).step_by(2) {
                    ret.push(pixel!(x0+i as f32/2., y0 - i as f32, Color::red()));
                    ret.push(pixel!(x0+i as f32/2., y0 - 1. - i as f32, Color::red()));
                }
            }
            11 => {
                let dx = (dx as f32 * 1.1).ceil() as i32;
                for i in (0..dx).step_by(2) {
                    ret.push(pixel!(x0+i as f32, y0 - i as f32/2., Color::red()));
                    ret.push(pixel!(x0 + 1. + i as f32, y0 - i as f32/2., Color::red()));
                }
            }
            _ => ()
        }
    } else {
        let dir = ((theta / (2. * PI / 8.)).round() + 8.) % 8.;

        let dy = dy.abs() as i32;
        let dx = dx.abs() as i32;
        match dir as i32 {
            0 => { for i in 0..dx { ret.push(pixel!(x0+i as f32, y0, Color::red()))} }
            1 => { for i in 0..dy { ret.push(pixel!(x0+i as f32, y0 + i as f32, Color::red()))} }
            2 => { for i in 0..dy { ret.push(pixel!(x0, y0+i as f32, Color::red()))} }
            3 => { for i in 0..dy { ret.push(pixel!(x0-i as f32, y0 + i as f32, Color::red()))} }
            4 => { for i in 0..dx { ret.push(pixel!(x0-i as f32, y0, Color::red()))} }
            5 => { for i in 0..dx { ret.push(pixel!(x0-i as f32, y0 - i as f32, Color::red()))} }
            6 => { for i in 0..dy { ret.push(pixel!(x0, y0 - i as f32, Color::red()))} }
            7 => { for i in 0..dx { ret.push(pixel!(x0+i as f32, y0 - i as f32, Color::red()))} }
            _ => error!("impossible"),
        }
    }

    ret
}

pub fn bresenham(start: &Point2D<f32>, stop: &Point2D<f32>) -> Vec<Pixel> {
    let mut ret = Vec::new();
    let mut x0 = start.x;
    let mut y0 = start.y;
    let x1 = stop.x;
    let y1 = stop.y;

    let dx = (x1-x0).abs();
    let sx = if x0<x1 {1.} else {-1.};
    let dy = -(y1-y0).abs();
    let sy = if y0<y1 {1.} else {-1.};
    let mut err = dx+dy; /* error value e_xy */
    loop {
        ret.push(pixel!(x0, y0, Color::red()));
        if x0==x1 && y0==y1 { break; }
        let e2 = 2. * err;
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
        let adjusted_end = snapped_line(true, &pixel!(0., 0., Color::red()), &pixel!(10., 9., Color::red()));
        println!("{:?}", adjusted_end);
    }
}