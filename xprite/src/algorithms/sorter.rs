//! Smoothing curves by sorting monotonic subcurve line segments by slope.

use crate::prelude::*;
use std::cmp::Ordering;

pub fn get_concavity(path: &[Pixel]) -> bool {
    let p1 = path[0];
    let p2 = path[path.len() / 2];
    let p3 = path[path.len() - 1];

    let Vec2f { x: x1, y: y1 } = p1.point;
    let Vec2f { x: x2, y: y2 } = p2.point;
    let Vec2f { x: x3, y: y3 } = p3.point;

    // console!(log, x1, x2, x3);
    // assert!(x1 < x2);
    // assert!(x2 < x3);
    if (x2 == x1) || (x2 == x3) {
        false
    } else {
        let m1 = (y2 - y1) / (x2 - x1);
        let m2 = (y3 - y2) / (x3 - x2);

        if m1 < m2 {
            false
        } else {
            true
        }
    }
}

/// [((dx, dy), slope)]
/// ((run x, run y), slope) tuple
pub fn to_chunks(path: &[Pixel], dir: i32) -> Vec<((i32, i32), f64)> {
    let mut segs = Vec::new();
    let Pixel { point: p0, .. } = path[0];
    let mut p0 = p0;
    let mut d = (1, 1);

    for Pixel { point: pi, .. } in path.iter() {
        let p0_ = p0;
        let pi_ = pi;
        // console!(log, format!("{:?}", pi));
        if pi.x == p0.x || pi.y == p0.y {
            d = (
                d.0 + (pi_.x - p0_.x) as i32,
                d.1 + (dir as f64 * (pi_.y - p0_.y)) as i32, // BUG:
            );
        } else {
            // console!(log, format!("{:?}", d));
            while d.0 > 1 && d.1 > 1 {
                segs.push(((1, 1), 1.));
                d.0 -= 1;
                d.1 -= 1;
            }
            segs.push((d, d.1 as f64 / d.0 as f64));
            d = (1, 1);
        }
        p0 = *pi;
    }
    segs.push((d, d.1 as f64 / d.0 as f64));
    segs
}

pub fn sort_path(input: &Pixels) -> Result<Pixels, String> {
    let mut path: Vec<Pixel> = input.0.iter().cloned().collect();

    // if the path is drawn from bottom to top
    let up = path.iter().last().ok_or_else(|| "does not contain last".to_owned())?.point.y < path[0].point.y;
    let mut dir = if up { -1 } else { 1 };

    // if the path is drawn from right to left
    let right_to_left = path.iter().last().ok_or_else(|| "does not contain last".to_owned())?.point.x < path[0].point.x;
    if right_to_left {
        dir *= -1;
        path.reverse();
    };

    let is_concave_up = get_concavity(&path);
    // console!(log, format!("concavity: {}\nup: {}", is_concave_up, up));

    let mut chunks = to_chunks(&path, dir);

    // sort by slope
    chunks.sort_by(|(_a, a_slope), (_b, b_slope)| {
        let r = a_slope - b_slope;
        if r < 0. {
            Ordering::Less
        } else if r == 0. {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    });

    if (is_concave_up && !up) || (!is_concave_up && up) {
        chunks.reverse();
    }

    // rrf in dihedral group
    if right_to_left {
        chunks.reverse();
    }

    let mut ret = Pixels::new();
    let mut p0 = path[0];

    // offset
    if (right_to_left && up) || (!right_to_left && !up) {
        p0.point.x -= 1.;
        p0.point.y -= 1.;
    } else if (!right_to_left && up) || (right_to_left && !up) {
        p0.point.x -= 1.;
        p0.point.y += 1.;
    }

    for &((dx, dy), _) in chunks.iter() {
        if dx == 1 {
            p0.point.x += 1.;
            for _ in 0..dy {
                if dir == 1 {
                    p0.point.y += 1.;
                } else {
                    p0.point.y -= 1.;
                }
                ret.push(p0);
            }
        } else if dy == 1 {
            if dir == 1 {
                p0.point.y += 1.;
            } else {
                p0.point.y -= 1.;
            }
            for _ in 0..dx {
                p0.point.x += 1.;
                ret.push(p0);
            }
        }
    }

    // console!(log, format!("{:#?}", segs));
    Ok(ret)
}
