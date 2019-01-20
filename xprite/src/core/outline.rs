use crate::prelude::*;

bitflags! {
    pub struct Outline: u32 {
        const TOP = 1;
        const BOTTOM = 1 << 1;
        const LEFT = 1 << 2;
        const RIGHT = 1 << 3;
    }
}

pub type MarqueePixel = (Vec2f, Outline);

/// outline
pub fn outline_rect(start: Option<Pixel>, stop: Option<Pixel>) -> Result<Vec<MarqueePixel>, String> {
    let mut ret = Vec::new();

    let start = start.ok_or_else(|| "start is none".to_owned())?;
    let stop = stop.ok_or_else(|| "stop is none".to_owned())?;
    let x1 = start.point.x as i32;
    let y1 = start.point.y as i32;
    let x2 = stop.point.x as i32;
    let y2 = stop.point.y as i32;

    let x1 = i32::min(x1, x2);
    let x2 = i32::max(x1, x2);
    let y1 = i32::min(y1, y2);
    let y2 = i32::max(y1, y2);

    ret.push((vec2f!(y1, x1), Outline::TOP | Outline::LEFT));
    ret.push((vec2f!(y2-1, x1), Outline::BOTTOM | Outline::LEFT));
    ret.push((vec2f!(y1, x2-1), Outline::TOP | Outline::RIGHT));
    ret.push((vec2f!(y2-1, x2-1), Outline::BOTTOM | Outline::RIGHT));

    for i in (x1+1)..x2 {
        ret.push((vec2f!{y1, i}, Outline::TOP));
        ret.push((vec2f!{y2-1, i}, Outline::BOTTOM));
    }
    for j in y1..y2 {
        ret.push((vec2f!{j, x1}, Outline::LEFT));
        ret.push((vec2f!{j, x2-1}, Outline::RIGHT));
    }
    Ok(ret)
}
