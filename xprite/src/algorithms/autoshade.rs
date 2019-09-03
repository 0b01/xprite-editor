//! Autoshade algorithm explanation
use crate::prelude::*;
use imageproc::affine::translate;
use imageproc::distance_transform::euclidean_squared_distance_transform;
use img::GrayImage;

const DBG_SAVE_IMG: bool = false;

#[derive(Debug, Clone, Default)]
pub struct AutoshadeStepParam {
    pub erode: f64,
    pub shift: Vec2f,
    pub mode: AutoshadeBlendingMode,
}

#[derive(Debug, Clone)]
pub enum AutoshadeBlendingMode {
    Lighten(u8),
    Replace(XpriteRgba),
}

impl Default for AutoshadeBlendingMode {
    fn default() -> Self {
        AutoshadeBlendingMode::Lighten(10)
    }
}

pub fn autoshade(pixs: &Pixels, accumulative: bool, step_params: &[AutoshadeStepParam]) -> Option<Pixels> {
    let mut ret = pixs.clone();
    if step_params.is_empty() {
        return Some(ret);
    }

    // expand bounding box to differentiate foreground at edge
    let (orig_bb, expanded_bb) = {
        let orig = pixs.bounding_rect();
        let AutoshadeStepParam { shift: Vec2f { x:_x, y:_y }, .. } = &step_params[0];

        let mut bb = orig.clone();
        bb.0.x -= 1.;
        bb.0.y -= 1.;
        bb.1.x += 1.;
        bb.1.y += 1.;
        (orig, bb)
    };

    // convert pixs to image
    let orig = binarize(pixs.as_image(expanded_bb, None)?.to_luma());

    if DBG_SAVE_IMG {
        orig.save("orig.png").unwrap();
    }
    let mut acc = orig.clone();
    let mut prev_mask = orig;
    let mut shift_acc = vec2f!(0., 0.);
    let mut erode_acc = 0.;
    for (i, step_param) in step_params.iter().enumerate() {
        let AutoshadeStepParam { erode, shift, mode } = step_param;
        erode_acc += *erode;
        shift_acc += *shift;
        let eroded = erode_l2norm(&acc, erode_acc);
        if DBG_SAVE_IMG {
            eroded.save(format!("eroded{}.png", i)).unwrap();
        }

        let translated = translate(&eroded, (shift_acc.x as i32, shift_acc.y as i32));

        let mut step_acc = Pixels::new();

        let w = eroded.width() as usize;
        let h = eroded.height() as usize;
        for (i, (p, orig_p)) in translated.iter().zip(prev_mask.iter()).enumerate() {
            let y = i / w;
            let x = i % w;
            if y == 0 || y == h - 1 || x == 0 || x == w - 1 {
                continue;
            }
            let intersect = *p == 255 && *orig_p == 255;
            if intersect {
                let orig_pixel = ret
                    .get_pixel(orig_bb.0.y as isize + y as isize - 1, orig_bb.0.x as isize + x as isize - 1)
                    .unwrap();
                let new_col = blend(mode, unsafe { orig_pixel.color.as_rgba() });
                step_acc.push(pixel!(y - 1, x - 1, Color::Rgba(new_col)));
            }
        }
        acc = if accumulative { eroded.clone() } else { acc };
        prev_mask = translated;
        erode_acc = if accumulative { 0. } else { erode_acc };
        ret.extend(&step_acc.shifted(orig_bb.0));
    }
    Some(ret)
}

fn blend(mode: &AutoshadeBlendingMode, input_color: XpriteRgba) -> XpriteRgba {
    use AutoshadeBlendingMode::*;
    match mode {
        Replace(col) => *col,
        Lighten(diff) => {
            let mut ret = input_color;
            ret.r += *diff;
            ret.g += *diff;
            ret.b += *diff;
            ret
        }
    }
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
        let shaded = autoshade(
            &pixs,
            true,
            &[
                AutoshadeStepParam {
                    erode: 200.,
                    shift: vec2f!(-6., -6.),
                    mode: AutoshadeBlendingMode::Lighten(10),
                },
                AutoshadeStepParam {
                    erode: 200.,
                    shift: vec2f!(-6., -6.),
                    mode: AutoshadeBlendingMode::Lighten(10),
                },
                AutoshadeStepParam {
                    erode: 200.,
                    shift: vec2f!(-6., -6.),
                    mode: AutoshadeBlendingMode::Lighten(10),
                },
            ],
        ).unwrap();
        let img = shaded.as_image(shaded.bounding_rect(), None).unwrap();
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
