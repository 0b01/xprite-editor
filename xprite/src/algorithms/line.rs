use crate::prelude::*;
use std::f64;
use std::f64::consts::PI;

pub fn snapped_line(is_45: bool, start: Vec2f, stop: Vec2f, color: Color) -> Pixels {
    let mut ret = Pixels::new();

    let x0 = start.x;
    let y0 = start.y;
    let x1 = stop.x;
    let y1 = stop.y;

    let dx = x1 - x0;
    let dy = y1 - y0;

    let theta = f64::atan2(dy, dx);

    if !is_45 {
        let dir = ((theta / (2. * PI / 12.)).round() + 12.) % 12.;

        let dy = dy.abs() as i32;
        let dx = dx.abs() as i32;
        match dir as i32 {
            0 => {
                for i in 0..dx {
                    ret.push(pixel!(y0, x0 + f64::from(i), color))
                }
            }
            1 => {
                let dx = (dx as f64 * 1.1).ceil() as i32;
                for i in (0..dx).step_by(2) {
                    ret.push(pixel!(y0 + f64::from(i) / 2., x0 + f64::from(i), color));
                    ret.push(pixel!(y0 + f64::from(i) / 2., x0 + 1. + f64::from(i), color));
                }
            }
            2 => {
                let dy = (dy as f64 * 1.1).ceil() as i32;
                for i in (0..dy).step_by(2) {
                    ret.push(pixel!(y0 + f64::from(i), x0 + f64::from(i) / 2., color));
                    ret.push(pixel!(y0 + 1. + f64::from(i), x0 + f64::from(i) / 2., color));
                }
            }
            3 => {
                for i in 0..dy {
                    ret.push(pixel!(y0 + f64::from(i), x0, color))
                }
            }
            4 => {
                let dy = (dy as f64 * 1.1).ceil() as i32;
                for i in (0..dy).step_by(2) {
                    ret.push(pixel!(y0 + f64::from(i), x0 - f64::from(i) / 2., color));
                    ret.push(pixel!(y0 + 1. + f64::from(i), x0 - f64::from(i) / 2., color));
                }
            }
            5 => {
                let dx = (dx as f64 * 1.1).ceil() as i32;
                for i in (0..dx).step_by(2) {
                    ret.push(pixel!(y0 + f64::from(i) / 2., x0 - f64::from(i), color));
                    ret.push(pixel!(y0 + f64::from(i) / 2., x0 - 1. - f64::from(i), color));
                }
            }
            6 => {
                for i in 0..dx {
                    ret.push(pixel!(y0, x0 - f64::from(i), color))
                }
            }
            7 => {
                let dx = (dx as f64 * 1.1).ceil() as i32;
                for i in (0..dx).step_by(2) {
                    ret.push(pixel!(y0 - f64::from(i) / 2., x0 - f64::from(i), color));
                    ret.push(pixel!(y0 - f64::from(i) / 2., x0 - 1. - f64::from(i), color));
                }
            }
            8 => {
                let dy = (dy as f64 * 1.1).ceil() as i32;
                for i in (0..dy).step_by(2) {
                    ret.push(pixel!(y0 - f64::from(i), x0 - f64::from(i) / 2., color));
                    ret.push(pixel!(y0 - 1. - f64::from(i), x0 - f64::from(i) / 2., color));
                }
            }
            9 => {
                for i in 0..dy {
                    ret.push(pixel!(y0 - f64::from(i), x0, color))
                }
            }
            10 => {
                let dy = (dy as f64 * 1.1).ceil() as i32;
                for i in (0..dy).step_by(2) {
                    ret.push(pixel!(y0 - f64::from(i), x0 + f64::from(i) / 2., color));
                    ret.push(pixel!(y0 - 1. - f64::from(i), x0 + f64::from(i) / 2., color));
                }
            }
            11 => {
                let dx = (dx as f64 * 1.1).ceil() as i32;
                for i in (0..dx).step_by(2) {
                    ret.push(pixel!(y0 - f64::from(i) / 2., x0 + f64::from(i), color));
                    ret.push(pixel!(y0 - f64::from(i) / 2., x0 + 1. + f64::from(i), color));
                }
            }
            _ => (),
        }
    } else {
        let dir = ((theta / (2. * PI / 8.)).round() + 8.) % 8.;

        let dy = dy.abs() as i32;
        let dx = dx.abs() as i32;
        match dir as i32 {
            0 => {
                for i in 0..dx {
                    ret.push(pixel!(y0, x0 + f64::from(i), color))
                }
            }
            1 => {
                for i in 0..dy {
                    ret.push(pixel!(y0 + f64::from(i), x0 + f64::from(i), color))
                }
            }
            2 => {
                for i in 0..dy {
                    ret.push(pixel!(y0 + f64::from(i), x0, color))
                }
            }
            3 => {
                for i in 0..dy {
                    ret.push(pixel!(y0 + f64::from(i), x0 - f64::from(i), color))
                }
            }
            4 => {
                for i in 0..dx {
                    ret.push(pixel!(y0, x0 - f64::from(i), color))
                }
            }
            5 => {
                for i in 0..dx {
                    ret.push(pixel!(y0 - f64::from(i), x0 - f64::from(i), color))
                }
            }
            6 => {
                for i in 0..dy {
                    ret.push(pixel!(y0 - f64::from(i), x0, color))
                }
            }
            7 => {
                for i in 0..dx {
                    ret.push(pixel!(y0 - f64::from(i), x0 + f64::from(i), color))
                }
            }
            _ => error!("impossible"),
        }
    }

    ret
}

pub fn pixel_perfect_line(start: Vec2f, stop: Vec2f, color: Color) -> Pixels {
    let Vec2f { x: mut x1, y: mut y1 } = start;
    let Vec2f { x: mut x2, y: mut y2 } = stop;
    let yaxis: bool = (y2 - y1).abs() > (x2 - x1).abs();
    if yaxis {
        std::mem::swap(&mut x1, &mut y1);
        std::mem::swap(&mut x2, &mut y2);
    }

    let w = (x2 - x1).abs() + 1.;
    let h = (y2 - y1).abs() + 1.;
    let dx = (x2 - x1).signum();
    let dy = (y2 - y1).signum();

    // Move x2 one extra pixel to the dx direction so we can use
    // operator!=() instead of operator<(). Here I prefer operator!=()
    // instead of swapping x1 with x2 so the error always start from 0
    // in the origin (x1,y1).
    x2 += dx;

    let mut ret = Pixels::new();
    let mut x = x1;
    let mut e = 0.;
    let mut y = y1;
    while x != x2 {
        if yaxis {
            ret.push(pixel!(x, y, color))
        } else {
            ret.push(pixel!(y, x, color))
        }

        // The error advances "h/w" per each "x" step. As we're using a
        // integer value for "e", we use "w" as the unit.
        e += h;
        if e >= w {
            y += dy;
            e -= w;
        }

        x += dx
    }

    ret
}

/// input must be snapped to grid
pub fn continuous_line(start: Vec2f, stop: Vec2f, color: Color) -> Pixels {
    let Vec2f { x: mut x0, y: mut y0 } = start.floor();
    let Vec2f { x: x1, y: y1 } = stop.floor();

    let dx = (x1 - x0).abs();
    let sx = (x1 - x0).signum();
    let dy = -(y1 - y0).abs();
    let sy = (y1 - y0).signum();
    let mut err = dx + dy;
    let mut e2;

    let mut ret = Pixels::new();
    loop {
        ret.push(pixel!(y0, x0, color));
        e2 = 2. * err;
        if e2 >= dy {
            if x0 == x1 {
                break;
            }
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            if y0 == y1 {
                break;
            }
            err += dx;
            y0 += sy;
        }
    }

    ret
}

#[deprecated]
pub fn bresenham(start: Vec2f, stop: Vec2f, color: Color) -> Pixels {
    let mut ret = Pixels::new();
    let mut x0 = start.x;
    let mut y0 = start.y;
    let x1 = stop.x;
    let y1 = stop.y;

    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1. } else { -1. };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1. } else { -1. };
    let mut err = dx + dy; /* error value e_xy */
    loop {
        ret.push(pixel!(y0, x0, color));
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2. * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        } /* e_xy+e_x > 0 */
        if e2 <= dx {
            err += dx;
            y0 += sy;
        } /* e_xy+e_y < 0 */
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_snap_line() {
        let adjusted_end = snapped_line(true, vec2f!(0., 0.), vec2f!(9., 10.), Color::red());

        let expected = pixels![
            pixel!(0, 0, Color::red()),
            pixel!(1, 1, Color::red()),
            pixel!(2, 2, Color::red()),
            pixel!(3, 3, Color::red()),
            pixel!(4, 4, Color::red()),
            pixel!(5, 5, Color::red()),
            pixel!(6, 6, Color::red()),
            pixel!(7, 7, Color::red()),
            pixel!(8, 8, Color::red())
        ];
        assert_eq!(expected, adjusted_end);
    }
}
