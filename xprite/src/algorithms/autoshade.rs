//! Autoshade algorithm explanation
//! 1. expand bounding box
//! 2. convert to luma and binarize
//! 3. corrode to mask
//! 4. shift the mask
use crate::prelude::*;
use imageproc::affine::translate;
use imageproc::distance_transform::euclidean_squared_distance_transform;
use img::GrayImage;

const DBG_SAVE_IMG: bool = true;

#[derive(Debug, Clone, Default)]
pub struct AutoshadeStepParam {
    pub erode: f64,
    pub shift: Vec2f,
    pub color: Color
}


pub fn autoshade(pixs: &Pixels, accumulative: bool, step_params: &[AutoshadeStepParam]) -> Pixels {
    let mut ret = pixs.clone();
    if step_params.is_empty() {
        return ret;
    }

    // expand bounding box to differentiate foreground at edge
    let (orig_bb, expanded_bb) = {
        let orig = pixs.bounding_rect();
        let AutoshadeStepParam{shift: Vec2f{x,y}, ..} = &step_params[0];

        let mut bb = orig.clone();
        bb.0.x += x;
        bb.0.y += y;
        bb.1.x -= x;
        bb.1.y -= y;
        (orig, bb)
    };

    // convert pixs to image
    let orig = binarize(pixs.as_image(expanded_bb).to_luma());

    if DBG_SAVE_IMG {
        orig.save("orig.png").unwrap();
    }
    let mut acc = orig.clone();
    let mut prev_mask = orig;
    let mut shift_acc = vec2f!(0., 0.);
    let mut erode_acc = 0.;
    for (i, step_param) in step_params.iter().enumerate() {
        let AutoshadeStepParam {erode, shift, color} = step_param;
        shift_acc += *shift;
        erode_acc += *erode;
        let eroded = erode_l2norm(&acc, erode_acc);
        if DBG_SAVE_IMG {
            eroded.save(format!("eroded{}.png", i)).unwrap();
        }

        let mut step_acc = Pixels::new();

        let w = eroded.width() as usize;
        let h = eroded.height() as usize;
        for (i, (p, orig_p)) in eroded.iter().zip(prev_mask.iter()).enumerate() {
            let row = i / w;
            let col = i % w;
            if row == 0 || row == h - 1 || col == 0 || col == w - 1 {
                continue;
            }
            if *p == 255 && *orig_p == 255 {
                step_acc.push(pixel!(row, col, *color));
            }
        }
        acc = if accumulative { eroded.clone() } else { acc };
        prev_mask = translate(&eroded, (-shift.x as i32, -shift.y as i32));
        erode_acc = if accumulative { 0. } else { erode_acc };
        ret.extend(&step_acc.shifted(shift_acc + orig_bb.0));
    }
    ret
}

fn erode_l2norm(image: &GrayImage, k: f64) -> GrayImage {
    fn inverse_mut(image: &mut GrayImage) {
        for i in image.iter_mut() {
            *i = if *i == 0 { 255 } else { 0 };
        }
    }
    let mut out = image.clone();
    inverse_mut(&mut out);
    let dist_image = euclidean_squared_distance_transform(&out);
    for (i, o) in dist_image.iter().zip(out.iter_mut()) {
        *o = if *i <= k { 0 } else { 255 };
    }
    out
}

fn binarize(mut img: GrayImage) -> GrayImage {
    for p in img.iter_mut() {
        if *p > 0 {
            *p = 255;
        }
    }
    img
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_autoshade() {
        use super::*;
        let r = 100;
        let mut pixs = crate::algorithms::ellipse::algo_ellipsefill(0, 0, r, r);
        pixs.extend(&pixs.shifted(vec2f!(0., 50.)));
        let shaded = autoshade(&pixs,
            true,
            &[
            AutoshadeStepParam {
                erode: 200.,
                shift: vec2f!(-6., -6.),
                color: Color::blue(),
            },
            AutoshadeStepParam {
                erode: 200.,
                shift: vec2f!(-6., -6.),
                color: Color::green(),
            },
            AutoshadeStepParam {
                erode: 200.,
                shift: vec2f!(-6., -6.),
                color: Color::orange(),
            }
        ]);
        let img = shaded.as_image(shaded.bounding_rect());
        img.save("autoshade.png").unwrap();
    }

    // #[test]
    // fn test_autoshade_one_pixel() {
    //     use super::*;
    //     let pixs = pixels!(pixel!(0, 0, Color::red()));
    //     let shaded = autoshade(&pixs, &[(200., 0.03, Color::blue()), (200., 0.03, Color::green())]);
    //     assert_eq!(shaded, pixs);
    // }

}
