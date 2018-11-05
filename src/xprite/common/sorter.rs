use xprite::*;
use std::cmp::Ordering;

pub fn sort_path(path: &mut [Pixel]) -> Vec<Pixel> {
    let mut dir = if path.iter().last().unwrap().point.y < path[0].point.y {
        -1
    } else {
        1
    };
    if path.iter().last().unwrap().point.x < path[0].point.x {
        console!(log, "blah");
        dir *= -1;
        path.reverse();
    };

    let mut segs = Vec::new();
    let mut p0 = path[0];
    let mut d = (1,1);

    for pi in path.iter() {
        // console!(log, format!("{:?}", pi));
        if pi.point.x == p0.point.x || pi.point.y == p0.point.y {
            d = (
                d.0 + pi.point.x as i32 - p0.point.x as i32,
                d.1 + dir * (pi.point.y as i32 - p0.point.y as i32),
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

    let mut ret = Vec::new();
    let mut p0 = path[0];

    p0.point.y -= 1;
    p0.point.x -= 1;
    for &((dx, dy), _) in segs.iter() {
        let mut dx = dx;
        let mut dy = dy;
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
