use crate::prelude::*;
use crate::algorithms::floodfill::floodfill;
use std::i32;

pub fn ellipse(x1: i32, y1: i32, x2:i32, y2:i32, col: Color) -> Result<Pixels, String> {
    let mut ret = Pixels::new();

    let xc = (x2 + x1) / 2;
    let yc = (y2 + y1) / 2;

    let width = x2 - x1;
    let height = y2 - y1;
    let a2 = width * width;
    let b2 = height * height;
    let fa2 = 4 * a2;
    let fb2 = 4 * b2;

    // first half
    let mut x = 0;
    let mut y = height;
    let mut sigma = 2*b2+a2*(1-2*height);
    while b2*x <= a2*y {
        ret.push(pixel!(xc + x, yc + y, col));
        ret.push(pixel!(xc - x, yc + y, col));
        ret.push(pixel!(xc + x, yc - y, col));
        ret.push(pixel!(xc - x, yc - y, col));
        if sigma >= 0 {
            sigma += fa2 * (1 - y);
            y -= 1;
        }
        sigma += b2 * ((4 * x) + 6);
        x += 1;
    }

    /* second half */
    let mut x = width;
    let mut y = 0;
    let mut sigma = 2*a2+b2*(1-2*width);
    while a2*y <= b2*x {
        ret.push(pixel!(xc + x, yc + y, col));
        ret.push(pixel!(xc - x, yc + y, col));
        ret.push(pixel!(xc + x, yc - y, col));
        ret.push(pixel!(xc - x, yc - y, col));
        if sigma >= 0 {
            sigma += fb2 * (1 - x);
            x -= 1;
        }
        sigma += a2 * ((4 * y) + 6);
        y += 1;
    }

    Ok(ret)
}

pub fn get_ellipse(start: Option<Pixel>, stop: Option<Pixel>, filled: bool) -> Result<Pixels, String> {
    let start = start.ok_or_else(||"start is none".to_owned())?;
    let stop = stop.ok_or_else(||"stop is none".to_owned())?;
    let x0 = start.point.x as i32;
    let y0 = start.point.y as i32;
    let x1 = stop.point.x as i32;
    let y1 = stop.point.y as i32;

    if (x1-x0 < 1) || (y1-y0<1) { return Err("".to_owned()); }

    let ret = ellipse(
        i32::min(x0, x1),
        i32::min(y0, y1),
        i32::max(x0, x1),
        i32::max(y0, y1),
        Color::red(),
    )?;

    if filled {
        // let x_m = (x0+x1)/2;
        // let y_m = (y0+y1)/2;
        // let origin = Vec2D::new(x_m as f32, y_m as f32);
        // let ok = floodfill((x0-x1).abs() as f32, (y0-y1).abs() as f32, &ret, origin, None, Color::blue());
        // return Ok(ok)
        unimplemented!()
    }
    Ok(ret)
}
