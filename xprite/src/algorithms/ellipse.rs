use crate::prelude::*;
use std::i32;

pub fn get_ellipse(
    start: Option<Pixel>,
    stop: Option<Pixel>,
    filled: bool,
) -> Result<Pixels, String> {
    let start = start.ok_or_else(|| "start is none".to_owned())?;
    let stop = stop.ok_or_else(|| "stop is none".to_owned())?;
    let x0 = start.point.x as i32;
    let y0 = start.point.y as i32;
    let x1 = stop.point.x as i32;
    let y1 = stop.point.y as i32;

    if (x1 - x0 < 1) || (y1 - y0 < 1) {
        return Err("".to_owned());
    }

    unimplemented! {}
}
