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
pub fn outline_rect(
    start: Option<Vec2f>,
    stop: Option<Vec2f>,
) -> Result<Vec<MarqueePixel>, String> {
    let mut ret = Vec::new();

    let start = start.ok_or_else(|| "start is none".to_owned())?;
    let stop = stop.ok_or_else(|| "stop is none".to_owned())?;
    let x1_ = start.x as i32;
    let y1_ = start.y as i32;
    let x2_ = stop.x as i32;
    let y2_ = stop.y as i32;

    let x1 = i32::min(x1_, x2_);
    let x2 = i32::max(x1_, x2_);
    let y1 = i32::min(y1_, y2_);
    let y2 = i32::max(y1_, y2_);

    ret.push((vec2f!(y1, x1), Outline::TOP | Outline::LEFT));
    ret.push((vec2f!(y2 - 1, x1), Outline::BOTTOM | Outline::LEFT));
    ret.push((vec2f!(y1, x2 - 1), Outline::TOP | Outline::RIGHT));
    ret.push((vec2f!(y2 - 1, x2 - 1), Outline::BOTTOM | Outline::RIGHT));

    for i in (x1 + 1)..x2 {
        ret.push((vec2f! {y1, i}, Outline::TOP));
    }
    for j in y1..y2 {
        ret.push((vec2f! {j, x1}, Outline::LEFT));
    }
    for i in (x1 + 1)..x2 {
        ret.push((vec2f! {y2-1, i}, Outline::BOTTOM));
    }
    for j in y1..y2 {
        ret.push((vec2f! {j, x2-1}, Outline::RIGHT));
    }
    Ok(ret)
}
