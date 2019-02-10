use crate::prelude::*;

pub fn polygon(points: &[Vec2f]) -> Pixels {
    let mut ret = Pixels::new();

    let n = points.len();
    let mut poly_ints = vec![0; n];

    let mut miny = points[0].y;
    let mut maxy = points[0].y;
    for point in &points[1..] {
        if point.y < miny {
            miny = point.y;
        }
        if point.y > maxy {
            maxy = point.y;
        }
    }

    /* 2.0.16: Optimization by Ilia Chipitsine -- don't waste time offscreen */
    /* 2.0.26: clipping rectangle is even better */
    // if (miny < im->cy1) {
    //   miny = im->cy1;
    // }
    // if (maxy > im->cy2) {
    //   maxy = im->cy2;
    // }

    /* Fix in 1.3: count a vertex only once */
    let mut y = miny;
    while y <= maxy {
        /*1.4           int interLast = 0; */
        /*              int dirLast = 0; */
        /*              int interFirst = 1; */
        /* 2.0.26+      int yshift = 0; */

        let mut ints = 0;

        let mut x1: f64;
        let mut x2: f64;
        let mut y1: f64;
        let mut y2: f64;
        for i in 0..n {
            let (ind1, ind2) = if i == 0 { (n - 1, 0) } else { (i - 1, i) };
            y1 = points[ind1].y;
            y2 = points[ind2].y;
            if y1 < y2 {
                x1 = points[ind1].x;
                x2 = points[ind2].x;
            } else if y1 > y2 {
                y2 = points[ind1].y;
                y1 = points[ind2].y;
                x2 = points[ind1].x;
                x1 = points[ind2].x;
            } else {
                continue;
            }

            /* Do the following math as float intermediately, and round to ensure
             * that Polygon and FilledPolygon for the same set of points have the
             * same footprint. */

            if (y >= y1) && (y < y2) {
                poly_ints[ints] = (((y - y1) * (x2 - x1)) as f64
                    / (y2 - y1) as f64
                    + 0.5
                    + x1) as i32;
                ints += 1;
            } else if (y == maxy) && (y > y1) && (y <= y2) {
                poly_ints[ints] = (((y - y1) * (x2 - x1)) as f64
                    / (y2 - y1) as f64
                    + 0.5
                    + x1) as i32;
                ints += 1;
            }
        }
        /*
        2.0.26: polygons pretty much always have less than 100 points,
        and most of the time they have considerably less. For such trivial
        cases, insertion sort is a good choice. Also a good choice for
        future implementations that may wish to indirect through a table.
        */
        for i in 1..ints {
            let index = poly_ints[i];
            let mut j = i;
            while j > 0 && poly_ints[j - 1] > index {
                poly_ints[j] = poly_ints[j - 1];
                j -= 1;
            }
            poly_ints[j] = index;
        }
        let mut i = 0;
        while i < ints {
            /* 2.0.29: back to gdImageLine to prevent segfaults when
            performing a pattern fill */
            let start = poly_ints[i];
            let stop = poly_ints[i + 1];
            for j in start..=stop {
                ret.push(pixel!(y, j, Color::red()));
            }
            i += 2;
        }
        y += 1.;
    }

    ret
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_polygon_square() {
        use super::*;
        assert_eq!(
            polygon(&vec![
                vec2f!(0, 0),
                vec2f!(0, 10),
                vec2f!(10, 10),
                vec2f!(10, 0),
                vec2f!(0, 0),
            ])
            .len(),
            11 * 11
        );

        assert_eq!(
            polygon(&vec![
                vec2f!(0, 0),
                vec2f!(0, 2),
                vec2f!(2, 2),
                vec2f!(2, 0),
            ])
            .len(),
            3 * 3
        );
    }
}
