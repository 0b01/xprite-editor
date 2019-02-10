use crate::core::outline::{MarqueePixel, Outline};
use crate::prelude::*;

pub fn find_perimeter(w: usize, h: usize, pixs: &Pixels) -> Pixels {
    let mut ret = Pixels::new();
    let canvas = pixs.as_mat(w, h);
    for y in 0..h {
        for x in 0..w {
            if canvas[y][x].is_some() {
                if (y >= 1 && canvas[y - 1][x].is_some())
                    && (y < h - 1 && canvas[y + 1][x].is_some())
                    && (x >= 1 && canvas[y][x - 1].is_some())
                    && (x < w - 1 && canvas[y][x + 1].is_some())
                {
                    continue;
                } else {
                    ret.push(canvas[y][x].unwrap())
                }
            }
        }
    }
    ret
}

pub fn find_outline(bb: Rect, pixs: &Pixels) -> Vec<MarqueePixel> {
    let mut ret = vec![];
    let h = bb.h() as usize;
    let w = bb.w() as usize;
    let canvas = pixs.as_mat_bb(bb);
    for y in 0..h {
        for x in 0..w {
            if canvas[y][x].is_some() {
                if (y >= 1 && canvas[y - 1][x].is_some())
                    && (y < h - 1 && canvas[y + 1][x].is_some())
                    && (x >= 1 && canvas[y][x - 1].is_some())
                    && (x < w - 1 && canvas[y][x + 1].is_some())
                {
                    continue;
                } else {
                    let mut outline = Outline::all();
                    if y >= 1 && canvas[y - 1][x].is_some() {
                        outline ^= Outline::TOP;
                    }
                    if y < h - 1 && canvas[y + 1][x].is_some() {
                        outline ^= Outline::BOTTOM;
                    }
                    if x >= 1 && canvas[y][x - 1].is_some() {
                        outline ^= Outline::LEFT;
                    }
                    if x < w - 1 && canvas[y][x + 1].is_some() {
                        outline ^= Outline::RIGHT;
                    }
                    let mut p = canvas[y][x].unwrap().point;
                    p.x += bb.0.x;
                    p.y += bb.0.y;
                    ret.push((p, outline));
                }
            }
        }
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_perimeter() {
        let mut pixs = Pixels::new();
        pixs.push(pixel!(0, 0, Color::red()));
        pixs.push(pixel!(0, 1, Color::red()));
        pixs.push(pixel!(0, 2, Color::red()));
        pixs.push(pixel!(1, 0, Color::red()));
        pixs.push(pixel!(1, 1, Color::red()));
        pixs.push(pixel!(1, 2, Color::red()));
        pixs.push(pixel!(2, 0, Color::red()));
        pixs.push(pixel!(2, 1, Color::red()));
        pixs.push(pixel!(2, 2, Color::red()));

        let mut result = Pixels::new();
        result.push(pixel!(0, 0, Color::red()));
        result.push(pixel!(0, 1, Color::red()));
        result.push(pixel!(0, 2, Color::red()));
        result.push(pixel!(1, 0, Color::red()));
        result.push(pixel!(1, 2, Color::red()));
        result.push(pixel!(2, 0, Color::red()));
        result.push(pixel!(2, 1, Color::red()));
        result.push(pixel!(2, 2, Color::red()));

        let peri = find_perimeter(3, 3, &pixs);
        assert_eq!(result, peri);

        let bb = pixs.bounding_rect();
        let outline = find_outline(bb, &pixs);
        let outline_result = vec![
            (vec2f_xy!(0, 0), Outline::TOP | Outline::LEFT),
            (vec2f_xy!(1, 0), Outline::TOP),
            (vec2f_xy!(2, 0), Outline::TOP | Outline::RIGHT),
            (vec2f_xy!(0, 1), Outline::LEFT),
            (vec2f_xy!(2, 1), Outline::RIGHT),
            (vec2f_xy!(0, 2), Outline::BOTTOM | Outline::LEFT),
            (vec2f_xy!(1, 2), Outline::BOTTOM),
            (vec2f_xy!(2, 2), Outline::BOTTOM | Outline::RIGHT),
        ];
        assert_eq!(outline_result, outline);
    }
}
