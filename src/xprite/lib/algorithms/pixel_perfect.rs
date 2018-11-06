use std::f32;
use xprite::prelude::*;
use super::polyline::point_line_distance;

fn get_min_dist(p: &Pixel, samples: &[Point2D<f32>]) -> f32 {
    let mut min_dist = f32::MAX;
    for i in 0..(samples.len()-1) {
        let dist = point_line_distance(p.point.into(), samples[i], samples[i+1]);
        if dist < min_dist { min_dist = dist; }
    }
    min_dist
}

fn is_extra_pixel(points: &[Pixel], i: usize) -> bool {
        if i<=0 || i>=points.len()-1 { false }
        else {
            let q1 = points[i-1];
            let q2 = points[i];
            let q3 = points[i+1];
            (q2.point.x-q1.point.x==0 && q3.point.y-q2.point.y==0)
            || (q2.point.y-q1.point.y==0 && q3.point.x-q2.point.x==0)
        }
}


/// remove extra pixels from path
/// Pixel perfect algo
/// samples are optional
pub fn pixel_perfect(path: &[Pixel], samples: Option<&[Point2D<f32>]>) -> Vec<Pixel> {
    let mut points = Vec::new();
    for i in 0..path.len() {
        if is_extra_pixel(&path, i) {
            let q1 = path[i-1];
            let q2 = path[i];
            let q3 = path[i+1];
            let mut remove = true;

            match samples {
                Some(s) => {
                    let d1 = get_min_dist(&q1, &s);
                    let d2 = get_min_dist(&q2, &s);
                    let d3 = get_min_dist(&q3, &s);
                    if (is_extra_pixel(&path, i-1) && d1 < d2)
                    || (is_extra_pixel(&path, i+1) && d3 < d2) {
                        remove = false;
                    }
                },
                None => {
                    let s = path.iter().map(|x| x.point.as_f32()).collect::<Vec<_>>();
                    let d1 = get_min_dist(&q1, &s);
                    let d2 = get_min_dist(&q2, &s);
                    let d3 = get_min_dist(&q3, &s);
                    if (is_extra_pixel(&path, i-1) && d1 < d2)
                    || (is_extra_pixel(&path, i+1) && d3 < d2) {
                        remove = false;
                    }
                },
            };

            if remove { continue; }
        }
        points.push(path[i]);
    }
    points
}