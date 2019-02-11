use crate::prelude::*;
use img::GrayImage;
use imageproc::distance_transform::euclidean_squared_distance_transform;
use imageproc::affine::translate;

const DBG_SAVE_IMG: bool = false;

pub fn autoshade(pixs: &Pixels, steps: &[(f64, f64, Color)]) -> Pixels {
    let mut ret = pixs.clone();
    if steps.is_empty() { return ret; }

    // convert pixs to image
    let mut bb = pixs.bounding_rect();
    let orig_bb = bb.clone();
    // expand bounding box to differentiate foreground at edge
    bb.0.x -= 100.;
    bb.0.y -= 100.;
    bb.1.x += 100.;
    bb.1.y += 100.;
    let img = pixs.as_image(bb);
    if DBG_SAVE_IMG { img.save("expanded.png").unwrap(); }
    let mut orig = img.to_luma();
    // binarize
    for p in orig.iter_mut() {
        if *p > 0 { *p = 255; }
    }
    if DBG_SAVE_IMG { orig.save("orig.png").unwrap(); }

    let shift = vec2f!(-100.,-100.) + orig_bb.0;
    let (mut acc, curr) = autoshade_step(&orig, 0, steps[0].0, steps[0].1, steps[0].2);
    ret.extend(&curr.shifted(shift));
    for (i, (step_d, step_dist_mul, step_col)) in steps[1..].iter().enumerate() {
        let (step_acc, curr) = autoshade_step(&acc, i+1, *step_d, *step_dist_mul, *step_col);
        acc = step_acc;
        ret.extend(&curr.shifted(shift));
    }
    ret
}

fn autoshade_step(orig: &GrayImage, ith: usize, step: f64, dist_mul: f64, color: Color) -> (GrayImage, Pixels) {
    let eroded = erode_l2norm(&orig, step);
    if DBG_SAVE_IMG { eroded.save(format!("eroded{}.png", ith)).unwrap(); }

    // shift the eroded image
    let d = (step * dist_mul) as i32;
    let translated = translate(&eroded, (-d, -d));
    if DBG_SAVE_IMG { translated.save(&format!("translated{}.png", ith)).unwrap(); }
    let mut ret = Pixels::new();

    let w = translated.width() as usize;
    let h = translated.height() as usize;
    for (i, (p, orig_p)) in translated.iter().zip(orig.iter()).enumerate() {
        let row = i / w;
        let col = i % w;
        if row == 0 || row == h-1 || col == 0 || col == w-1 {
            continue;
        }
        if *p == 255 && *orig_p == 255 {
            ret.push(pixel!(row, col, color));
        }
    }

    (translated, ret)
}

fn inverse_mut(image: &mut GrayImage) {
    for i in image.iter_mut() {
        *i = if *i != 0 { 0 } else { 255 };
    }
}

fn erode_l2norm(image: &GrayImage, k: f64) -> GrayImage {
    let mut out = image.clone();
    inverse_mut(&mut out);
    let dist_image = euclidean_squared_distance_transform(&out);
    for (i, o) in dist_image.iter().zip(out.iter_mut()) {
        *o = if *i <= k { 0 } else { 255 };
    }
    out
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_autoshade() {
        use super::*;
        let r = 100;
        let mut pixs = crate::algorithms::ellipse::algo_ellipsefill(0,0,r,r);
        pixs.extend(&pixs.shifted(vec2f!(0., 50.)));

        let shaded = autoshade(&pixs, &[
            (200., 0.03, Color::blue()),
            (200., 0.03, Color::green()),
        ]);
        let img = shaded.as_image(shaded.bounding_rect());
        if DBG_SAVE_IMG { img.save("out.png").unwrap(); }
    }

    #[test]
    fn test_autoshade_one_pixel() {
        use super::*;
        let pixs = pixels!(pixel!(0,0, Color::red()));
        let shaded = autoshade(&pixs, &[
            (200., 0.03, Color::blue()),
            (200., 0.03, Color::green()),
        ]);
        assert_eq!(shaded, pixs);
    }
}
