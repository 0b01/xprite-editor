use crate::prelude::*;
use crate::core::outline::{Outline, MarqueePixel};

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

pub fn find_outline(w: usize, h: usize, pixs: &Pixels) -> Vec<MarqueePixel> {
    let mut ret = vec![];
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
                    ret.push(( canvas[y][x].unwrap().point, outline));
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

        let outline = find_outline(3, 3, &pixs);
        let outline_result = vec![
            (vec2f!(0, 0), Outline::TOP | Outline::LEFT),
            (vec2f!(1, 0), Outline::TOP),
            (vec2f!(2, 0), Outline::TOP | Outline::RIGHT),
            (vec2f!(0, 1), Outline::LEFT),
            (vec2f!(2, 1), Outline::RIGHT),
            (vec2f!(0, 2), Outline::BOTTOM | Outline::LEFT),
            (vec2f!(1, 2), Outline::BOTTOM),
            (vec2f!(2, 2), Outline::BOTTOM | Outline::RIGHT),
        ];
        assert_eq!(outline_result, outline);
    }
}
