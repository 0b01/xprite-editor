//! Various distance functions
use image::Rgb;

pub type DistanceFunction = fn(&Rgb<u8>, &Rgb<u8>) -> f64;

/// L1 distance, also known as Manhattan distance
pub fn l1(p1: &Rgb<u8>, p2: &Rgb<u8>) -> f64 {
    let f = |c1, c2| ((c1 as f64) - (c2 as f64)).abs();
    f(p1[0], p2[0]) + f(p1[1], p2[1]) + f(p1[2], p2[2])
}

pub fn l2(p1: &Rgb<u8>, p2: &Rgb<u8>) -> f64 {
    let f = |c1, c2| {
        let n = (c1 as i32) - (c2 as i32);
        n * n
    };
    ((f(p1[0], p2[0]) + f(p1[1], p2[1]) + f(p1[2], p2[2])) as f64).sqrt()
}
