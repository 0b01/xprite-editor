use std::f32;
use std::cmp::{min, max};
use crate::prelude::*;
use super::pixel_perfect::pixel_perfect;

fn convert(p1: Point2D, p2: Point2D, p3: Point2D, p4: Point2D) -> CubicBezierSegment {
    let t = 0.5;
    CubicBezierSegment {
        from: Point2D::new(p2.x, p2.y),
        ctrl1: Point2D::new(
            p2.x + (p3.x-p1.x)/(6.*t),
            p2.y + (p3.y-p1.y)/(6.*t)
        ),
        ctrl2: Point2D::new(
            p3.x - (p4.x-p2.x)/(6.*t),
            p3.y - (p4.y-p2.y)/(6.*t)
        ),
        to: Point2D::new(p3.x, p3.y),
    }
}


fn line_slope(p0: Point2D, p1: Point2D) -> f32 {
    if p1.x == p0.x {
        0.
    } else {
        (p1.y - p0.y) / (p1.x - p0.x)
    }
}

fn line_finite_diff(points: &[Point2D]) -> Vec<f32> {
    let mut m = Vec::new();
    let mut p0 = points[0];
    let mut p1 = points[1];
    let mut d = line_slope(p0, p1);
    m.push(d);

    for i in 1..(points.len()-1) {
        p0 = p1;
        p1 = points[i + 1];
        let d1 = line_slope(p0, p1);
        m.push((d + d1) / 2.);
        d = d1
    }
    m.push(d);
    m
}


#[derive(Debug, PartialEq, Clone)]
pub struct Path {
    pub segments: Vec<CubicBezierSegment>,
}

impl Path {
    pub fn from_polyline(polyline: &Polyline) -> Self {
        let points = &polyline.pos;
        let mut segments = Vec::new();

        if points.len() < 3 {
            return Path {segments: Vec::new()}
        }
        let tangents = Path::d3_svg_line_monotone(&points);
        // let tangents = Path::d3_svg_line_cardinal_tangents(&points, 0.7);

        // let mut i = 0;
        // for _ in 0..(points.len()-1) {
        //     let line = CubicBezierSegment {
        //         from: points[i],
        //         ctrl1: points[i] + tangents[i],
        //         ctrl2: points[i+1] + tangents[i+1],
        //         to: points[i+1]
        //     };
        //     segments.push(line);
        //     i += 1;
        // }

        let mut pi = 1;
        let p0 = points[0];
        let t0 = tangents[0];
        if tangents.len() > 1 {
            let t = tangents[1];
            let p = points[pi];
            pi += 1;

            let from = p0;
            let ctrl1 = Point2D::new(p0.x + t0.x, p0.y + t0.y);
            let ctrl2 = Point2D::new(p.x - t.x, p.y - t.y);
            let to =  Point2D::new(p.x, p.y);
            let curve = CubicBezierSegment {from, ctrl1, ctrl2, to};
            segments.push(curve);

            let mut i = 2;
            while i < tangents.len() {
                let p = points[pi];
                let t = tangents[i];
                let from = points[pi - 1];
                let ctrl1 = Point2D::new(from.x + tangents[i-1].x, from.y + tangents[i-1].y);
                let ctrl2 = Point2D::new(p.x - t.x, p.y - t.y);
                let to = p;
                let curve = CubicBezierSegment {from, ctrl1, ctrl2, to};
                segments.push(curve);
                i += 1;
                pi += 1;
            }
        }

        Path { segments }
    }

    /// from d3:
    ///     https://github.com/d3/d3/blob/a40a611d6b9fc4ff3815ca830d86b6c00d130995/src/svg/line.js#L377
    pub fn d3_svg_line_monotone(points: &[Point2D]) -> Vec<Point2D> {
        let mut tangents = Vec::new();

        let mut m = line_finite_diff(&points);

        for i in 0..(points.len() - 1) {
            let d = line_slope(points[i], points[i + 1]);
            if d.abs() < 0.01 {
                m[i + 1] = 0.;
                m[i] = 0.;
            } else {
                let a = m[i] / d;
                let b = m[i + 1] / d;
                let s = a * a + b * b;
                if s > 9. {
                    let s = d * 3. / s.sqrt();
                    m[i] = s * a;
                    m[i + 1] = s * b;
                }
            }
        }

        for i in 0..points.len() {
            let p0 = points[min(points.len()-1, i + 1)];
            let p1 = points[max(0, i as isize - 1) as usize];
            let s = (p0.x - p1.x) / (6. * (1. + m[i] * m[i]));
            tangents.push(Point2D::new(s, m[i] * s));
        }
        tangents
    }

    // Generates tangents for a cardinal spline.
    pub fn d3_svg_line_cardinal_tangents(points: &[Point2D], tension: f32) -> Vec<Point2D> {
        let mut tangents = Vec::new();

        let a = (1. - tension) / 2.;
        let mut p0;
        let mut p1 = points[0];
        let mut p2 = points[1];
        let mut i = 1;
        let n = points.len();
        i += 1;
        while i < n {
            p0 = p1;
            p1 = p2;
            p2 = points[i];
            tangents.push(Point2D::new(
                a * (p2.x - p0.x),
                a * (p2.y - p0.y)
            ));
            i += 1;
        }
        return tangents;
    }

    /// get control points
    pub fn control_points(&self) -> Circles {
        let mut circ_buf = Pixels::new();

        // circles: control points
        for seg in &self.segments {
            let CubicBezierSegment { ctrl1, ctrl2, .. } = seg;
            for p in vec![ctrl1, ctrl2] {
                // let Point2D{x, y} = xpr.canvas.snap(p);
                circ_buf.push(pixel!(p.x, p.y, Color::red()));
            }
        }

        circ_buf
    }

    #[allow(unused)]
    /// found on pomax's website (catmull-rom)
    pub fn cubic(polyline: &Polyline) -> Self {
        let mut segments = Vec::new();

        let mut first = 0;
        let mut second = 1;
        let mut third = 2;

        let distance = 0.;

        for _ in 0..(polyline.pos.len()-3) {
            let mut p1 = polyline.pos[first];
            let mut p2 = polyline.pos[second];
            let mut p3 = polyline.pos[third];

            let mut dx = p3.x - p1.x;
            let mut dy = p3.y - p1.y;
            let m = (dx*dx + dy*dy).sqrt();
            dx /= m;
            dy /= m;


            let mut p0 = Point2D::new(
                p1.x + (p3.x - p2.x) - distance * dx,
                p1.y + (p3.y - p2.y) - distance * dy
            );
            let p4 = Point2D::new(
                p1.x + (p3.x - p2.x) + distance * dx,
                p1.y + (p3.y - p2.y) + distance * dy
            );

            let seg0 = convert(p0, p1, p2, p3);
            segments.push(seg0);
            let seg1 = convert(p1, p2, p3, p4);
            segments.push(seg1);

            first  += 1;
            second += 1;
            third  += 1;
        }


        Path {
            segments
        }
    }

    pub fn rasterize(&self, xpr: &Xprite) -> Option<Pixels> {
        let mut ret = Vec::new();
        // convert each segment
        for seg in &self.segments {
            let pixs = Path::convert_path_to_pixel(xpr, seg)?;
            ret.extend(&pixs);
        }

        let points = pixel_perfect(&ret);
        Some(Pixels::from_slice(&points))
    }

    /// rasterize a single bezier curve by sampling
    fn convert_path_to_pixel(xpr: &Xprite, seg: &CubicBezierSegment) -> Option<Vec<Pixel>> {
        let mut path = Vec::new();

        let mut set = Pixels::new();

        // sample n points
        for i in 0..100 {
            let t = i as f32 / 100.;
            let point = seg.sample(t);

            let Point2D {x, y} = xpr.canvas.snap(&point);
            let pixel = pixel!(x, y, Color::red());

            // don't allow duplicate pixels
            if !set.contains(&pixel) {
                set.push(pixel);
                path.push(pixel);
            }
        }

        let points = pixel_perfect(&path);

        Some(points)
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line_finite_diff() {
        let points = vec![
            Point2D::new(10., 10.),
            Point2D::new(50., 10.),
        ];
        assert_eq!(vec![0., 0.], line_finite_diff(&points));
    }

    #[test]
    fn test_line_slope() {
        let p0 = Point2D::new(10., 10.);
        let p1 = Point2D::new(10., 10.);
        let p2 = Point2D::new(0., 0.);

        assert_eq!(0., line_slope(p0, p1));
        assert_eq!(1., line_slope(p0, p2));
    }

    #[test]
    fn test_monotonic_tangent() {
        let points = vec![
            Point2D::new(10., 10.),
            Point2D::new(50., 10.),
        ];
        assert_eq!(
            vec![
                Point2D::new(6.666666666666667, 0.0),
                Point2D::new(6.666666666666667, 0.0)
            ],
            Path::d3_svg_line_monotone(&points),
        )
    }
}
