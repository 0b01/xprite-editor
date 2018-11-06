//! Smoothing curves by sorting monotonic subcurve line segments by slope.

use xprite::prelude::*;
use std::cmp::Ordering;

pub fn get_concavity(path: &[Pixel]) -> bool {
    let p1 = path[0];
    let p2 = path[path.len() / 2];
    let p3 = path[path.len() - 1];

    let Point2D {x: x1, y: y1} = p1.point.as_i32();
    let Point2D {x: x2, y: y2} = p2.point.as_i32();
    let Point2D {x: x3, y: y3} = p3.point.as_i32();

    // assert!(x1 < x2);
    // assert!(x2 < x3);

    let m1 = (y2 - y1) / (x2 - x1);
    let m2 = (y3 - y2) / (x3 - x2);

    if m1 < m2 {
        false
    } else {
        true
    }
}

pub fn sort_path(path: &mut [Pixel]) -> Vec<Pixel> {

    let going_up = path.iter().last().unwrap().point.y < path[0].point.y;
    let mut dir = if going_up {
        // console!(log, "going up");
        -1
    } else {
        1
    };
    // if the path is drawn from right to left
    let right_to_left = path.iter().last().unwrap().point.x < path[0].point.x;
    if right_to_left {
        // console!(log, "right to left");
        dir *= -1;
        path.reverse();
    };

    let is_concave_up = get_concavity(path);
    // console!(log, format!("concavity: {}", is_concave_up));

    let mut segs = Vec::new();
    let Pixel { point: mut p0, .. } = path[0];
    let mut d = (1,1);

    for Pixel {point: pi, ..} in path.iter() {
        let p0_ = pi.as_i32();
        let pi_ = pi.as_i32();
        // console!(log, format!("{:?}", pi));
        if pi.x == p0.x || pi.y == p0.y {
            d = (
                d.0 +        pi_.x - p0_.x,
                d.1 + dir * (pi_.y - p0_.y),
            );
        } else {
            // console!(log, format!("{:?}", d));
            while d.0 > 1 && d.1 > 1 {
                segs.push( ((1,1), 1. ));
                d.0 -= 1;
                d.1 -= 1;
            }
            segs.push(
                ( d, d.1 as f32 / d.0 as f32 )
            );
            d = (1,1);
        }
        p0 = *pi;
    }
    segs.push( ( d, d.1 as f32 / d.0 as f32));

    // sort by slope
    segs.sort_by(|a, b| {
        let r = a.1 - b.1;
        if r < 0. {Ordering::Less}
        else if r == 0. {Ordering::Equal}
        else {Ordering::Greater}
    });

    if (is_concave_up && !going_up)
    || (!is_concave_up && going_up) {
        segs.reverse();
    }

    if right_to_left {
        segs.reverse();
    }

    let mut ret = Vec::new();
    let mut p0 = path[0];
    p0.point.y -= 1;
    p0.point.x -= 1;
    for &((dx, dy), _) in segs.iter() {
        if dx == 1 {
            p0.point.x += 1;
            for _ in 0..dy {
                if dir == 1 {
                    p0.point.y += 1;
                } else {
                    p0.point.y -= 1;
                }
                ret.push(p0);
            }
        } else if dy == 1 {
            if dir == 1 {
                p0.point.y += 1;
            } else {
                p0.point.y -= 1;
            }
            for _ in 0..dx {
                p0.point.x += 1;
                ret.push(p0);
            }
        }
    }

    // console!(log, format!("{:#?}", segs));
    ret
}
