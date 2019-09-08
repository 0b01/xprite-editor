use crate::prelude::*;

/// selectively color a pixel perfect line
/// each segment of length l contains l*k number of pixels with alt color
pub fn selective_antialias(path: &mut Pixels, k: f64, alt_color: Color) {
    let min_segment_length = 2;
    let mut chunks = vec![];
    let mut last = 0;
    for (i, (pi, pj)) in path.iter().zip(path.iter().skip(1)).enumerate() {
        // if consecutive pixels are not connected
        if (pi.point.x != pj.point.x) && (pi.point.y != pj.point.y) {
            let last_pix = path.0.get_index(last).unwrap();
            let dir = last_pix.dir(&pi);

            //            println!("{:?} {:?}", last_pix, pi);
            chunks.push((i - last, dir));
            last = i + 1;
        }
    }
    let last_pix = path.0.get_index(last).unwrap();
    let dir = last_pix.dir(path.iter().last().unwrap());
    chunks.push(((path.len() - last - 1), dir));

    //    println!("{:?}", chunks);
    assert_eq!(chunks.iter().map(|i| i.0 + 1).sum::<usize>(), path.len());

    let mut acc = 0;
    for (l, dir) in chunks {
        for idx in 0..=l {
            if l >= min_segment_length && (idx <= (l as f64 * k) as usize) ^ dir {
                path.0.get_index(acc).unwrap().set_color(alt_color);
            } else {
                // noop
            }
            acc += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_selective_antialias() {
        let mut path = pixels![
            pixel!(0, 0, Color::red()),
            pixel!(0, 1, Color::red()),
            pixel!(0, 2, Color::red()),
            pixel!(1, 3, Color::red()),
            pixel!(2, 3, Color::red()),
            pixel!(3, 3, Color::red()),
            pixel!(4, 4, Color::red())
        ];

        selective_antialias(&mut path, 0.5, Color::orange());
        for i in path.iter() {
            print!("{:?}", i.point);
            println!("{:?}", i.color);
        }
    }

}
