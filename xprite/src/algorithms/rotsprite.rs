use crate::prelude::*;

/// pixel rotator based on rotsprite a algorithm
/// angle is in radians
pub fn rotsprite(pixs: &Pixels, angle: f64, pivot: Vec2f) -> Pixels {
    if angle == 0. {
        pixs.clone()
    } else {
        let mut bb = pixs.bounding_rect();
        if bb.0.x > pivot.x || bb.0.y > pivot.y {
            bb.0 = pivot;
        } else if bb.1.x < pivot.x || bb.1.y < pivot.y {
            bb.1 = pivot;
        }
        // dbg!(bb);
        let pivot = pivot - bb.0;
        // dbg!(pivot);
        let pixs = pixs.shifted(-bb.0);
        let pixs = scale2x(&pixs, bb);
        let pixs = scale2x(&pixs, Rect(bb.0, bb.1 * 2.));
        let pixs = scale2x(&pixs, Rect(bb.0, bb.1 * 4.));
        // pixs.save("scaled.png");
        let rotated = rotate_and_reduce(&pixs, Rect(bb.0, bb.1 * 8.), angle, pivot);

        rotated.shifted(bb.0)
    }
}

/// Doubles the size of the given image using the scale2x Algorithm
fn scale2x(original: &Pixels, bb: Rect) -> Pixels {
    let w = bb.w() as usize;
    let h = bb.h() as usize;
    let mut scaled = Pixels::new();
    for x in 0..h {
        for y in 0..w {
            let x = x as isize;
            let y = y as isize;
            let p = original.get_pixel(x, y).map(|i| i.color);
            let a = if y > 0 { original.get_pixel(x, y - 1).map(|i| i.color) } else { p };
            let b = if x < w as isize - 1 {
                original.get_pixel(x + 1, y).map(|i| i.color)
            } else {
                p
            };
            let c = if x > 0 { original.get_pixel(x - 1, y).map(|i| i.color) } else { p };
            let d = if y < h as isize - 1 {
                original.get_pixel(x, y + 1).map(|i| i.color)
            } else {
                p
            };
            macro_rules! set_pix {
                ($x:expr, $y:expr, $col:expr) => {
                    if let Some(col) = $col {
                        scaled.push(pixel!(vec2f!($x, $y), col));
                    }
                };
            }
            if c == a && c != d && a != b {
                set_pix!((x << 1), (y << 1), a);
            } else {
                set_pix!((x << 1), (y << 1), p);
            }
            if a == b && a != c && b != d {
                set_pix!((x << 1) + 1, (y << 1), b);
            } else {
                set_pix!((x << 1) + 1, (y << 1), p);
            }
            if d == c && d != b && c != a {
                set_pix!((x << 1), (y << 1) + 1, c);
            } else {
                set_pix!((x << 1), (y << 1) + 1, p);
            }
            if b == d && b != a && d != c {
                set_pix!((x << 1) + 1, (y << 1) + 1, d);
            } else {
                set_pix!((x << 1) + 1, (y << 1) + 1, p);
            }
        }
    }
    scaled
}

///Rotates the image while also scaling it down by a factor of 8
/// bb is the bounding rect of pixs
fn rotate_and_reduce(scaled: &Pixels, bb: Rect, angle: f64, pivot: Vec2f) -> Pixels {
    let mut rotated = Pixels::new();
    let w = bb.w();
    let h = bb.h();

    let (center_x, center_y) = (w / 2., h / 2.);

    let shift = if pivot == vec2f!(center_x / 8., center_y / 8.) {
        vec2f!(center_x / 8., center_y / 8.)
    } else {
        let dy = pivot.y - center_y / 8.;
        let dx = pivot.x - center_x / 8.;
        let dir = (dy).atan2(dx) + angle;
        let mag = (dx.powf(2.) + dy.powf(2.)).sqrt();
        let yy = -dir.sin() * mag;
        let xx = -dir.cos() * mag;
        vec2f!(yy.round(), xx.round())
    };

    for x in 0..(w as usize) {
        for y in 0..(h as usize) {
            let x = x as f64;
            let y = y as f64;
            let dir = (y - center_y).atan2(x - center_x) + angle;
            let mag = ((x - center_x).powf(2.) + (y - center_y).powf(2.)).sqrt() * 8.;

            let orig_x = (center_x + mag * dir.cos()).round();
            let orig_y = (center_y + mag * dir.sin()).round();

            // if oob(orig_x, orig_y, w, h) { continue; }
            let col = scaled.get_pixel(orig_x as isize, orig_y as isize);
            if let Some(c) = col {
                rotated.push(pixel!(x - center_x, y - center_y, c.color));
            }
        }
    }

    rotated.shifted(shift)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_scale2x() {
        use super::*;
        let pixs = pixels!(pixel!(0, 0, Color::orange()), pixel!(1, 1, Color::blue()));
        let ret = scale2x(&pixs, pixs.bounding_rect());
        assert_eq!(
            pixels! {
                pixel!(0,0,Color::void()),
                pixel!(1,0,Color::void()),
                pixel!(0,1,Color::void()),
                pixel!(3,2,Color::void()),
                pixel!(2,3,Color::void()),
                pixel!(3,3,Color::void())
            },
            ret
        );
    }

    #[test]
    fn test_scale2x_1() {
        use super::*;

        let pixs = pixels!(pixel!(0, 0, Color::orange()), pixel!(0, 2, Color::white()), pixel!(1, 1, Color::blue()));

        let ret = scale2x(&pixs, pixs.bounding_rect());

        assert_eq!(
            pixels! {
                // ...
            },
            ret
        );

        let img = ret.as_image(ret.bounding_rect(), None).unwrap();
        // img.save("scale2x.png").unwrap();
    }

    #[test]
    fn test_scale2x_2() {
        use super::*;
        let pixs = pixels!(pixel!(0, 0, Color::orange()), pixel!(0, 1, Color::white()), pixel!(1, 1, Color::blue()));
        let ret = scale2x(&pixs, pixs.bounding_rect());
        assert_eq!(12, ret.len());
    }

    #[test]
    fn test_rotsprite() {
        use super::*;
        let mut pixs = crate::algorithms::rect::filled_rect(0, 0, 30, 30, Color::white()).unwrap();
        pixs.extend(&crate::algorithms::rect::rect(0, 0, 30, 30, Color::orange()).unwrap());

        let mut pixs1 = pixs.clone();
        let rotated = rotsprite(&pixs, -PI / 6., vec2f!(0, 0));
        pixs1.extend(&rotated);
        pixs1.save("rotsprite1.png", None);

        let mut pixs1 = pixs.clone();
        let rotated = rotsprite(&pixs, -PI / 6., vec2f!(15, 15));
        pixs1.extend(&rotated);
        pixs1.save("rotsprite2.png", None);

        let mut pixs1 = pixs.shifted(vec2f!(10, 10));
        let rotated = rotsprite(&pixs1, -PI / 6., vec2f!(0, 0));
        pixs1.extend(&rotated);
        pixs1.save("rotsprite3.png", None);
    }
}
