//! Implementation of the Efros and Freeman image quilting algorithm.
use image::*;
use rand::{Rng, Rand, Closed01, thread_rng};
use rand::distributions::{Range, IndependentSample};
use rayon::prelude::*;

use std::collections::HashMap;
use std::sync::Mutex;

use common::{OrderedFloat, blit_rect, Rect, Patch};
use distance::DistanceFunction;
use errors::*;

type ErrorSurface = ImageBuffer<Luma<f64>, Vec<f64>>;
type CostMap = HashMap<(u32, u32), OrderedFloat<f64>>;

#[allow(dead_code)]
/// Generate an integer image of a normalized error surface.
fn debug_error_surface(mut err_surf: ErrorSurface) -> GrayImage {
    // Find the maximum
    let mut max = 0.;
    for pixel in err_surf.pixels() {
        if pixel.data[0] > max {
            max = pixel.data[0];
        }
    }

    // Then normalize the results
    for pixel in err_surf.pixels_mut() {
        pixel.data[0] /= max/255.;
    }

    let mut img = GrayImage::new(err_surf.width(), err_surf.height());
    for (x, y, pixel) in err_surf.enumerate_pixels() {
        img.put_pixel(x, y, Luma { data: [pixel.data[0].round() as u8] });
    }

    img
}

#[derive(Debug, Clone, Copy)]
/// Enumerates the possible overlapping areas of two patches
enum OverlapArea {
    /// Patches overlap vertically
    Top,
    /// Patches overlap horizontall
    Left,
    /// Patches overlap both vertically and horizontally
    TopLeft
}

fn patch_overlap_area(patch_no: (u32, u32)) -> OverlapArea {
    match patch_no {
        (0, _) => OverlapArea::Top,
        (_, 0) => OverlapArea::Left,
        (_, _) => OverlapArea::TopLeft,
    }
}

/// Compute the error between two images in a rectangle of specified size at
/// the specified coordinates.
fn patch_rect_error(distance_func: DistanceFunction, img1: &RgbImage, img2: &RgbImage,
                    coords_i1: (u32, u32), coords_i2: (u32, u32),
                    rect_size: (u32, u32)) -> f64 {
    let (x1, y1) = coords_i1;
    let (x2, y2) = coords_i2;
    let mut acc = 0.;
    for y in 0..rect_size.1 {
        for x in 0..rect_size.0 {
            acc += distance_func(img1.get_pixel(x + x1, y + y1),
                                 img2.get_pixel(x + x2, y + y2));
        }
    };
    acc
}

/// Describes the parameters of the `Quilter` type.
pub struct QuilterParams {
    size: (u32, u32),
    patch_size: u32,
    overlap: u32,
    seed_coords: Option<(u32, u32)>,
    selection_chance: Option<f64>,
    distance_func: DistanceFunction
}

impl QuilterParams {
    /// Create a new `QuilterParams`
    ///
    /// * `size`: Size of the synthesized image
    /// * `patch_size`: Size of the sample patches
    /// * `overlap`: Size of the overlapping area between consecutive patches
    /// * `seed_coords`: Coordinates of the first patch used in the algorithm
    /// * `selection_chance`: Selection chance of a patch in the selection phase.
    /// If `None`, the algorithm will perform an exhaustive search. Otherwise,
    /// represents the probability that a patch will be considered.
    /// * `distance_func`: Distance function used by the algorithm
    pub fn new(size: (u32, u32), patch_size: u32, overlap: u32,
               seed_coords: Option<(u32, u32)>, selection_chance: Option<f64>,
               distance_func: DistanceFunction) -> Result<QuilterParams> {
        // Check that input size and overlap size are non zero
        match size {
            (0, _) | (_, 0) => bail!(ErrorKind::InvalidArguments("Output size can't be zero".to_owned())),
            _ => ()
        }
        if overlap == 0 {
            bail!(ErrorKind::InvalidArguments("Overlap size can't be zero".to_owned()))
        }
        // Check that the patch size is in a valid range
        if patch_size < (2 * overlap) {
            bail!(ErrorKind::InvalidArguments("Patch size must be at least twice the overlap area size".to_owned()))
        }
        if let Some(s) = selection_chance {
            if s <= 0. {
                bail!(ErrorKind::InvalidArguments("Selection chance must be strictly positive".to_owned()))
            }
        }

        Ok(QuilterParams { size: size, patch_size: patch_size, overlap: overlap,
                           seed_coords: seed_coords,
                           selection_chance: selection_chance,
                           distance_func: distance_func })
    }
}

/// Implements the Efros and Freeman image quilting algorithm.
pub struct Quilter {
    source: RgbImage,
    buffer_opt: Option<RgbImage>,
    params: QuilterParams
}

impl Quilter {
    /// Create a new `Quilter`.
    pub fn new(source: RgbImage, params: QuilterParams) -> Quilter {
        Quilter { source: source, buffer_opt: None, params: params }
    }

    fn validate_params(&self, source_size: (u32, u32)) -> Result<()> {
        // Safety checks
        // Check that the image dimensions are at least as large as the patch size
        let (src_width, src_height) = source_size;
        if self.params.patch_size > src_width || self.params.patch_size > src_height {
            bail!(ErrorKind::InvalidArguments("Patch size must be smaller than the image smallest dimension".to_owned()))
        }
        // Check that the seed patch is within bounds
        if let Some((x_seed, y_seed)) = self.params.seed_coords {
            if (x_seed + self.params.patch_size) > src_width || (y_seed + self.params.patch_size) > src_height {
                bail!(ErrorKind::InvalidArguments("Seed patch coordinates are out of bounds".to_owned()))
            }
        }
        Ok(())
    }

    /// Synthesize an image by the image quilting algorithm.
    pub fn quilt_image(&mut self) -> Result<RgbImage> {
        let d = self.source.dimensions();
        try!(self.validate_params(d));
        let (img_width, img_height) = d;
        let step = self.params.patch_size - self.params.overlap;

        let x_patches =
            if (self.params.size.0 % step) == 0 { self.params.size.0 / step }
            else { self.params.size.0 / step + 1 };
        let y_patches =
            if (self.params.size.1 % step) == 0 { self.params.size.1 / step }
            else { self.params.size.1 / step + 1 };
        let (buffer_width, buffer_height) = (self.params.size.0 + self.params.patch_size, self.params.size.1 + self.params.patch_size);
        self.buffer_opt = Some(RgbImage::new(buffer_width, buffer_height));

        // Blit the first patch
        let mut rng = thread_rng();
        let patch_x_dist = Range::new(0u32, img_width - self.params.patch_size);
        let patch_y_dist = Range::new(0u32, img_height - self.params.patch_size);
        blit_rect(self.buffer_opt.as_mut().unwrap(), &self.source,
                   &Rect { coords: if let Some(seed_coordinates) = self.params.seed_coords { seed_coordinates }
                                   else { (patch_x_dist.ind_sample(&mut rng), patch_y_dist.ind_sample(&mut rng)) },
                            size: (self.params.patch_size, self.params.patch_size) },
                   (0u32, 0u32));

        for patch_y in 0..y_patches {
            for patch_x in 0..x_patches {
                if patch_x == 0 && patch_y == 0 { continue };
                let area = patch_overlap_area((patch_x, patch_y));
                let corner = (patch_x * step, patch_y * step);
                let candidate = self.select_candidate(area, corner);
                let err_surf = self.patch_error_surface(area, &candidate, corner);
                self.cut_and_blit_patch(&candidate, corner, &err_surf, area);

                println!("Done patch ({}, {})", patch_x, patch_y);
            }
        }

        let mut quilt = self.buffer_opt.take().unwrap();
        Ok(quilt.sub_image(0, 0, self.params.size.0, self.params.size.1).to_image())
    }

    /// Compute the error between the specified overlap area of the specified
    /// patch and the buffer.
    fn patch_error(&self, area: OverlapArea, patch: &Patch, buf_coords: (u32, u32)) -> f64 {
        let buffer = self.buffer_opt.as_ref().unwrap();
        match area {
            OverlapArea::Top => {
                patch_rect_error(self.params.distance_func, &self.source,
                                 buffer, patch.coords, buf_coords,
                                 (self.params.overlap, patch.size))
            }
            OverlapArea::Left => {
                patch_rect_error(self.params.distance_func, &self.source,
                                 buffer, patch.coords, buf_coords,
                                 (patch.size, self.params.overlap))
            },
            OverlapArea::TopLeft => {
                patch_rect_error(self.params.distance_func, &self.source,
                                 buffer, patch.coords, buf_coords,
                                 (patch.size, self.params.overlap)) +
                patch_rect_error(self.params.distance_func, &self.source,
                                 buffer,
                                 (patch.coords.0, patch.coords.1 + self.params.overlap),
                                 (buf_coords.0, buf_coords.1 + self.params.overlap),
                                 (self.params.overlap, patch.size - self.params.overlap))
            },
        }
    }

    /// Find a candidate patch to be quilted at the specified coordinates on
    /// the buffer.
    fn select_candidate(&self, area: OverlapArea, buf_coords: (u32, u32)) -> Patch
    {
        const TOLERANCE: f64 = 0.1;
        let (w, h) = self.source.dimensions();
        let (max_x, max_y) = (w - self.params.patch_size, h - self.params.patch_size);
        let candidates_scores = Mutex::new(vec!());
        let current_best = Mutex::new(::std::f64::INFINITY);
        let mut rng = thread_rng();
        if let Some(chance) = self.params.selection_chance {
            let mut scores = candidates_scores.lock().unwrap();
            let mut best = current_best.lock().unwrap();
            while scores.is_empty() {
                for y in 0..max_y + 1 {
                    for x in 0..max_x + 1 {
                        let Closed01(d) = Closed01::<f64>::rand(&mut rng);
                        if d > chance {
                            let p = Patch { coords: (x, y), size: self.params.patch_size };
                            let error = self.patch_error(area, &p, buf_coords);
                            if error < *best * (1. + TOLERANCE) {
                                *best = if error < *best { error } else { *best };
                                scores.push((p, error));
                            }
                        }
                    }
                }
            }
        }
        else {
            (0..max_y + 1).into_par_iter().for_each(|y| {
                for x in 0..max_x + 1 {
                    let p = Patch { coords: (x, y), size: self.params.patch_size };
                    let error = self.patch_error(area, &p, buf_coords);
                    let mut best = current_best.lock().unwrap();
                    let mut scores = candidates_scores.lock().unwrap();
                    if error < *best * (1. + TOLERANCE) {
                        *best = if error < *best { error } else { *best };
                        scores.push((p, error));
                    }
                }
            });
        }
        let scores = candidates_scores.into_inner().unwrap();
        let best = current_best.lock().unwrap();
        let mut candidates: Vec<Patch> = scores.into_iter().filter_map(|(p, err)| if err > *best * (1. + TOLERANCE) { None } else { Some(p.clone()) }).collect();
        println!("Found {} candidates", candidates.len());
        rng.shuffle(&mut candidates);
        candidates.first().unwrap().clone()
    }

    /// Compute the error surface of the specified patch.
    fn patch_error_surface(&self, area: OverlapArea, patch: &Patch, buf_coords: (u32, u32)) -> ErrorSurface {
        let mut err_surf = ErrorSurface::new(self.params.patch_size, self.params.patch_size);
        let (xs, ys) = buf_coords;
        let (px, py) = patch.coords;
        let dist = self.params.distance_func;
        match area {
            OverlapArea::Top => {
                for x in 0..self.params.patch_size {
                    for y in 0..self.params.overlap {
                        let err = dist(self.source.get_pixel(px + x, py + y),
                                       self.buffer_opt.as_ref().unwrap().get_pixel(xs + x, ys + x));
                        err_surf.put_pixel(x, y, Luma { data: [err] });
                    }
                }
            },
            OverlapArea::Left => {
                for x in 0..self.params.overlap {
                    for y in 0..self.params.patch_size {
                        let err = dist(self.source.get_pixel(px + x, py + y),
                                       self.buffer_opt.as_ref().unwrap().get_pixel(xs + x, ys + x));
                        err_surf.put_pixel(x, y, Luma { data: [err] });
                    }
                }
            },
            OverlapArea::TopLeft => {
                for x in 0..self.params.patch_size {
                    for y in 0..self.params.overlap {
                        let err = dist(self.source.get_pixel(px + x, py + y),
                                       self.buffer_opt.as_ref().unwrap().get_pixel(xs + x, ys + x));
                        err_surf.put_pixel(x, y, Luma { data: [err] });
                    }
                }
                for x in 0..self.params.overlap {
                    for y in self.params.overlap..self.params.patch_size {
                        let err = dist(self.source.get_pixel(px + x, py + y),
                                       self.buffer_opt.as_ref().unwrap().get_pixel(xs + x, ys + x));
                        err_surf.put_pixel(x, y, Luma { data: [err] });
                    }
                }
            }
        }

        err_surf
    }

    fn vertical_cost_map(&self, err_surf: &ErrorSurface) -> CostMap {
        let mut cost_map = CostMap::new();

        fn pixel_error(cost_map: &mut CostMap, e: &ErrorSurface, overlap: u32,
                       x: u32, y: u32) -> OrderedFloat<f64> {
            if cost_map.contains_key(&(x, y)) {
                *cost_map.get(&(x, y)).unwrap()
            }
            else if y == 0 {
                let val = *e.get_pixel(x, y);
                let v = OrderedFloat::<f64>::try_from(val.data[0]).unwrap();
                cost_map.insert((x, y), v);
                v
            }
            else {
                let mut val = pixel_error(cost_map, e, overlap, x, y - 1);
                if x != 0 {
                    let v = pixel_error(cost_map, e, overlap, x - 1, y - 1);
                    if v < val { val = v };
                }
                if x != overlap - 1 {
                    let v = pixel_error(cost_map, e, overlap, x + 1, y - 1);
                    if v < val { val = v };
                }
                val += OrderedFloat::<f64>::try_from(e.get_pixel(x, y).data[0]).unwrap();
                cost_map.insert((x, y), val);
                val
            }
        };

        for x in 0..self.params.overlap {
            pixel_error(&mut cost_map, err_surf, self.params.overlap, x, self.params.patch_size - 1);
        }

        cost_map
    }

    fn minimum_cost_vertical_path(&self, err_surf: &ErrorSurface) -> Vec<(u32, u32)> {
        let mut v = vec!();
        let cost_map = self.vertical_cost_map(err_surf);

        // Find path starting point
        let row = (0..self.params.overlap).into_iter().map(|x| cost_map[&(x, self.params.patch_size - 1)]).collect::<Vec<_>>();
        let (mut x, mut y) = (row.into_iter().enumerate().min_by(|&(_, v1), &(_, v2)| v1.cmp(&v2)).unwrap().0 as u32,
                              self.params.patch_size - 1);
        v.push((x, y));
        while y != 0 {
            let top = cost_map[&(x, y - 1)];
            if x == 0 {
                let right = cost_map[&(x + 1, y - 1)];
                if right < top { x += 1; }
            }
            else if x == self.params.overlap - 1 {
                let left = cost_map[&(x - 1, y - 1)];
                if left < top { x -= 1; }
            }
            else {
                let left = cost_map[&(x - 1, y - 1)];
                let right = cost_map[&(x + 1, y - 1)];
                if left < top {
                    if left < right { x -= 1; }
                }
                else if right < top { x += 1; }
            }
            y -= 1;
            v.push((x, y));
        }

        v
    }

    fn horizontal_cost_map(&self, err_surf: &ErrorSurface) -> CostMap {
        let mut cost_map = CostMap::new();

        fn pixel_error(cost_map: &mut CostMap, e: &ErrorSurface, overlap: u32,
                       x: u32, y: u32) -> OrderedFloat<f64> {
            if cost_map.contains_key(&(x, y)) {
                *cost_map.get(&(x, y)).unwrap()
            }
            else if x == 0 {
                let val = *e.get_pixel(x, y);
                let v = OrderedFloat::<f64>::try_from(val.data[0]).unwrap();
                cost_map.insert((x, y), v);
                v
            }
            else {
                let mut val = pixel_error(cost_map, e, overlap, x - 1, y);
                if y != 0 {
                    let v = pixel_error(cost_map, e, overlap, x - 1, y - 1);
                    if v < val { val = v };
                }
                if y != overlap - 1 {
                    let v = pixel_error(cost_map, e, overlap, x - 1, y + 1);
                    if v < val { val = v };
                }
                val += OrderedFloat::<f64>::try_from(e.get_pixel(x, y).data[0]).unwrap();
                cost_map.insert((x, y), val);
                val
            }
        };

        for y in 0..self.params.overlap {
            pixel_error(&mut cost_map, err_surf, self.params.overlap, self.params.patch_size - 1, y);
        }

        cost_map
    }

    fn minimum_cost_horizontal_path(&self, err_surf: &ErrorSurface) -> Vec<(u32, u32)> {
        let mut v = vec!();
        let cost_map = self.horizontal_cost_map(err_surf);

        // Find path starting point
        let column = (0..self.params.overlap).into_iter().map(|y| cost_map[&(self.params.patch_size - 1, y)]).collect::<Vec<_>>();
        let (mut x, mut y) = (self.params.patch_size - 1,
                              column.into_iter().enumerate().min_by(|&(_, v1), &(_, v2)| v1.cmp(&v2)).unwrap().0 as u32);
        v.push((x, y));
        while x != 0 {
            let left = cost_map[&(x - 1, y)];
            if y == 0 {
                let down = cost_map[&(x - 1, y + 1)];
                if down < left { y += 1; }
            }
            else if y == self.params.overlap - 1 {
                let up = cost_map[&(x - 1, y - 1)];
                if up < left { y -= 1; }
            }
            else {
                let up = cost_map[&(x - 1, y - 1)];
                let down = cost_map[&(x - 1, y + 1)];
                if up < left {
                    if up < down { y -= 1; }
                }
                else if down < left { y += 1; }
            }
            x -= 1;
            v.push((x, y));
        }

        v
    }

    fn cut_and_blit_vertical(&mut self, patch: &Patch, buf_coords: (u32, u32),
                             path: Vec<(u32, u32)>) {
        let buffer = self.buffer_opt.as_mut().unwrap();
        for (xp, yp) in path {
            if yp + patch.coords.1 < buffer.height() {
                for x in 0..self.params.overlap {
                    if x >= xp && x < buffer.width()  {
                        buffer.put_pixel(buf_coords.0 + x, buf_coords.1 + yp, *self.source.get_pixel(patch.coords.0 + x, patch.coords.1 + yp));
                    }
                }
            }
        }
    }

    fn cut_and_blit_horizontal(&mut self, patch: &Patch, buf_coords: (u32, u32),
                               path: Vec<(u32, u32)>) {
        let buffer = self.buffer_opt.as_mut().unwrap();
        for (xp, yp) in path {
            if xp + patch.coords.0 < buffer.width() {
                for y in 0..self.params.overlap {
                    if y >= yp && y < buffer.height()  {
                        buffer.put_pixel(buf_coords.0 + xp, buf_coords.1 + y, *self.source.get_pixel(patch.coords.0 + xp, patch.coords.1 + y));
                    }
                }
            }
        }
    }

    fn cut_and_blit_corner(&mut self, patch: &Patch, buf_coords: (u32, u32),
                           hpath: Vec<(u32, u32)>, vpath: Vec<(u32, u32)>) {
        let overlap = self.params.overlap;
        let mut do_pixel = |x, y| {
            let buffer = self.buffer_opt.as_mut().unwrap();
            let hpos = hpath.iter().find(|&&(xx, _)| xx == x).unwrap();
            let vpos = vpath.iter().find(|&&(_, yy)| yy == y).unwrap();
            if y >= hpos.1 && x >= vpos.0 {
                buffer.put_pixel(buf_coords.0 + x, buf_coords.1 + y,
                                 *self.source.get_pixel(patch.coords.0 + x, patch.coords.1 + y));
            }
        };
        for x in 0..overlap {
            for y in 0..overlap {
                do_pixel(x, y);
            }
        }
    }

    fn cut_and_blit_patch(&mut self, patch: &Patch, buf_coords: (u32, u32),
                          err_surf: &ErrorSurface, area: OverlapArea) {
        let overlap = self.params.overlap;
        match area {
            OverlapArea::Left => {
                let path = self.minimum_cost_vertical_path(err_surf);
                self.cut_and_blit_vertical(patch, buf_coords, path);
                let mut buffer = self.buffer_opt.as_mut().unwrap();
                blit_rect(buffer, &self.source,
                          &Rect { coords: (patch.coords.0 + overlap, patch.coords.1),
                                  size: (self.params.patch_size - overlap, self.params.patch_size) },
                          (buf_coords.0 + overlap, buf_coords.1));
            },
            OverlapArea::Top => {
                let path = self.minimum_cost_horizontal_path(err_surf);
                self.cut_and_blit_horizontal(patch, buf_coords, path);
                let mut buffer = self.buffer_opt.as_mut().unwrap();
                blit_rect(buffer, &self.source,
                          &Rect { coords: (patch.coords.0, patch.coords.1 + overlap),
                                  size: (self.params.patch_size, self.params.patch_size - overlap) },
                          (buf_coords.0, buf_coords.1 + overlap));
            },
            OverlapArea::TopLeft => {
                let (vpath, vpath_corner): (Vec<_>, Vec<_>) = self.minimum_cost_vertical_path(err_surf)
                                                                  .into_iter()
                                                                  .partition(|&(_, y)| y >= overlap);
                let (hpath, hpath_corner): (Vec<_>, Vec<_>) = self.minimum_cost_horizontal_path(err_surf)
                                                                  .into_iter()
                                                                  .partition(|&(x, _)| x >= overlap);
                self.cut_and_blit_vertical(patch, buf_coords, vpath);
                self.cut_and_blit_horizontal(patch, buf_coords, hpath);
                self.cut_and_blit_corner(patch, buf_coords, hpath_corner, vpath_corner);
                let mut buffer = self.buffer_opt.as_mut().unwrap();
                blit_rect(buffer, &self.source,
                          &Rect { coords: (patch.coords.0 + overlap, patch.coords.1 + overlap),
                                  size: (self.params.patch_size - overlap, self.params.patch_size - overlap) },
                          (buf_coords.0 + overlap, buf_coords.1 + overlap));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use distance::l1;

    #[test]
    fn test_patch_rect_error() {
        let mut i1 = RgbImage::new(11, 11);
        let i2 = RgbImage::new(11, 11);
        i1.put_pixel(3, 3, Rgb { data: [7, 7, 7] });
        i1.put_pixel(4, 4, Rgb { data: [20, 20, 20] });
        i1.put_pixel(6, 6, Rgb { data: [20, 20, 20] });
        i1.put_pixel(5, 7, Rgb { data: [7, 7, 7] });
        i1.put_pixel(7, 5, Rgb { data: [7, 7, 7] });

        let f = patch_rect_error(l1, &i1, &i2, (4, 4), (0, 0), (3u32, 3u32));
        assert_relative_eq!(f, 120.);
    }

    #[test]
    fn test_patch_error_surface_left() {
        // Give values to the first column of the source image
        let mut source = RgbImage::new(11, 11);
        for y in 0..5 {
            source.put_pixel(0, y, Rgb { data: [255, 0, 0] });
        }

        let params = QuilterParams::new((100, 100), 5, 1, None, None, l1).unwrap();
        let mut quilter = Quilter::new(source, params);
        let patch = Patch { coords: (0, 0), size: 5 };
        quilter.buffer_opt = Some(RgbImage::new(11, 11));

        let err_surf = quilter.patch_error_surface(OverlapArea::Left, &patch, (0, 0));
        for y in 0..5 {
            let val = err_surf.get_pixel(0, y).data[0];
            assert!(val == 255.);
        }
    }
}
