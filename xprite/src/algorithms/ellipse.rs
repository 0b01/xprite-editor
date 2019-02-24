use crate::algorithms::line::pixel_perfect_line;
use crate::prelude::*;
use std::i32;

pub fn get_ellipse(start: Option<Pixel>, stop: Option<Pixel>, filled: bool) -> Result<Pixels, String> {
    let start = start.ok_or_else(|| "start is none".to_owned())?;
    let stop = stop.ok_or_else(|| "stop is none".to_owned())?;
    let x0 = start.point.x as i32;
    let y0 = start.point.y as i32;
    let x1 = stop.point.x as i32;
    let y1 = stop.point.y as i32;

    if (x1 - x0 < 1) || (y1 - y0 < 1) {
        return Err("".to_owned());
    }
    let ret = if filled {
        algo_ellipsefill(x0, y0, x1, y1)
    } else {
        algo_ellipse(x0, y0, x1, y1)
    };

    Ok(ret)
}

fn bresenham_ellipse_error(rx: i32, ry: i32, x: i32, y: i32) -> i32 {
    x * x * ry * ry + y * y * rx * rx - rx * rx * ry * ry
}

// Move to next pixel to draw, according to Bresenham's algorithm
fn bresenham_ellipse_step(rx: i32, ry: i32, x: &mut i32, y: &mut i32) {
    // Move towards the skinnier pole. Having 2 cases isn't needed, but it ensures
    // swapping rx and ry is the same as rotating 90 degrees.
    if rx > ry {
        let ex = bresenham_ellipse_error(rx, ry, *x, *y - 1);
        let ey = bresenham_ellipse_error(rx, ry, *x + 1, *y);
        let exy = bresenham_ellipse_error(rx, ry, *x + 1, *y - 1);
        if ex + exy < 0 {
            *x += 1;
        }
        if ey + exy > 0 {
            *y -= 1;
        }
    } else {
        let ex = bresenham_ellipse_error(rx, ry, *x, *y + 1);
        let ey = bresenham_ellipse_error(rx, ry, *x - 1, *y);
        let exy = bresenham_ellipse_error(rx, ry, *x - 1, *y + 1);
        if ex + exy > 0 {
            *x -= 1;
        }
        if ey + exy < 0 {
            *y += 1;
        }
    }
}

/// Helper function for the ellipse drawing routines. Calculates the
/// points of an ellipse which fits onto the rectangle specified by x1,
/// y1, x2 and y2, and calls the specified routine for each one. The
/// output proc has the same format as for do_line, and if the width or
/// height of the ellipse is only 1 or 2 pixels, do_line will be
/// called.
///
/// Copyright (C) 2002 by Elias Pschernig (eliaspschernig@aon.at)
/// for Allegro 4.x.
///
/// Adapted for ASEPRITE by David A. Capello.
pub fn algo_ellipse(x1: i32, y1: i32, x2: i32, y2: i32) -> Pixels {
    let mut ret = Pixels::new();

    let mx;
    let my;
    let mut rx;
    let mut ry;
    let mut x;
    let mut y;

    /* Cheap hack to get elllipses with integer diameter, by just offsetting
     * some quadrants by one pixel. */
    let mx2;
    let my2;

    mx = (x1 + x2) / 2;
    mx2 = (x1 + x2 + 1) / 2;
    my = (y1 + y2) / 2;
    my2 = (y1 + y2 + 1) / 2;
    rx = (x1 - x2).abs();
    ry = (y1 - y2).abs();

    if rx == 1 {
        pixel_perfect_line(Vec2f { x: x2 as f64, y: y1 as f64 }, Vec2f { x: x2 as f64, y: y2 as f64 });
        rx -= 1;
    }
    if rx == 0 {
        pixel_perfect_line(Vec2f { x: x1 as f64, y: y1 as f64 }, Vec2f { x: x1 as f64, y: y2 as f64 });
        return ret;
    }

    if ry == 1 {
        pixel_perfect_line(Vec2f { x: x1 as f64, y: y2 as f64 }, Vec2f { x: x2 as f64, y: y2 as f64 });
        ry -= 1;
    }
    if ry == 0 {
        pixel_perfect_line(Vec2f { x: x1 as f64, y: y1 as f64 }, Vec2f { x: x2 as f64, y: y1 as f64 });
        return ret;
    }

    rx /= 2;
    ry /= 2;

    /* Draw the 4 poles. */
    ret.push(pixel_xy!(mx, my2 + ry, Color::red()));
    ret.push(pixel_xy!(mx, my - ry, Color::red()));
    ret.push(pixel_xy!(mx2 + rx, my, Color::red()));
    ret.push(pixel_xy!(mx - rx, my, Color::red()));

    /* For even diameter axis, double the poles. */
    if mx != mx2 {
        ret.push(pixel_xy!(mx2, my2 + ry, Color::red()));
        ret.push(pixel_xy!(mx2, my - ry, Color::red()));
    }

    if my != my2 {
        ret.push(pixel_xy!(mx2 + rx, my2, Color::red()));
        ret.push(pixel_xy!(mx - rx, my2, Color::red()));
    }

    // Start at the fatter pole
    if rx > ry {
        x = 0;
        y = ry;
    } else {
        x = rx;
        y = 0;
    }

    loop {
        /* Step to the next pixel to draw. */
        bresenham_ellipse_step(rx, ry, &mut x, &mut y);

        /* Edge conditions */
        if y == 0 && x < rx {
            y += 1;
        } // don't move to horizontal radius except at pole
        if x == 0 && y < ry {
            x += 1;
        } // don't move to vertical radius except at pole
        if y <= 0 || x <= 0 {
            break;
        } // stop before pole, since it's already drawn

        /* Process pixel */
        ret.push(pixel_xy!(mx2 + x, my - y, Color::red()));
        ret.push(pixel_xy!(mx - x, my - y, Color::red()));
        ret.push(pixel_xy!(mx2 + x, my2 + y, Color::red()));
        ret.push(pixel_xy!(mx - x, my2 + y, Color::red()));
    }

    ret
}

pub fn algo_ellipsefill(x1: i32, y1: i32, x2: i32, y2: i32) -> Pixels {
    let mut ret = Pixels::new();
    let mx;
    let my;
    let mut rx;
    let mut ry;
    let mut x;
    let mut y;

    /* Cheap hack to get elllipses with integer diameter, by just offsetting
     * some quadrants by one pixel. */
    let mx2;
    let my2;

    let mut hline = |x1, y, x2| {
        for xi in x1..x2 {
            ret.push(pixel_xy!(xi, y, Color::red()));
        }
    };

    mx = (x1 + x2) / 2;
    mx2 = (x1 + x2 + 1) / 2;
    my = (y1 + y2) / 2;
    my2 = (y1 + y2 + 1) / 2;
    rx = (x1 - x2).abs();
    ry = (y1 - y2).abs();

    if rx == 1 {
        let mut c = y1;
        while c <= y2 {
            hline(x2, c, x2);
            rx -= 1;
            c += 1;
        }
    }
    if rx == 0 {
        let mut c = y1;
        while c <= y2 {
            hline(x1, c, x1);
            c += 1;
        }
        return ret;
    }

    if ry == 1 {
        hline(x1, y2, x2);
        ry -= 1;
    }
    if ry == 0 {
        hline(x1, y1, x2);
        return ret;
    }

    rx /= 2;
    ry /= 2;

    /* Draw the north and south poles (possibly 2 pixels) */
    hline(mx, my2 + ry, mx2);
    hline(mx, my - ry, mx2);

    /* Draw the equator (possibly width 2) */
    hline(mx - rx, my, mx2 + rx);
    if my != my2 {
        hline(mx - rx, my2, mx2 + rx);
    }

    /* Initialize drawing position at a pole. */
    // Start at the fatter pole
    if rx > ry {
        x = 0;
        y = ry;
    } else {
        x = rx;
        y = 0;
    }

    loop {
        /* Step to the next pixel to draw. */
        bresenham_ellipse_step(rx, ry, &mut x, &mut y);

        /* Edge conditions */
        if y == 0 && x < rx {
            y += 1;
        } // don't move to horizontal radius except at pole
        if x == 0 && y < ry {
            x += 1;
        } // don't move to vertical radius except at pole
        if y <= 0 || x <= 0 {
            break;
        } // stop before pole, since it's already drawn

        /* Draw the north and south 'lines of latitude' */
        hline(mx - x, my - y, mx2 + x);
        hline(mx - x, my2 + y, mx2 + x);
    }

    ret
}
