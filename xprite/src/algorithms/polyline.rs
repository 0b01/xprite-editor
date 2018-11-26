use crate::prelude::*;
use crate::algorithms::line::bresenham;

#[derive(Debug, Clone, PartialEq)]
pub struct Polyline {
    pub pos: Vec<Point2D<f32>>,
}

/// a bunch of pixel positions
impl Polyline {
    pub fn new() -> Self {
        Self {
            pos: Vec::new(),
        }
    }

    pub fn push(&mut self, p: Point2D<f32>) {
        self.pos.push(p)
    }

    pub fn clear(&mut self) {
        self.pos.clear()
    }

    pub fn anchors(&self, xpr: &Xprite) -> Circles {
        let mut circ_buf = Pixels::new();
        for &p in self.pos.iter() {
            // let Point2D{x, y} = xpr.canvas.snap(&p);
            circ_buf.push(pixel!(p.x, p.y, Color::blue()));
        }
        circ_buf
    }

    /// line simplification algorithm
    pub fn reumann_witkam(&self, tol: f32) -> Option<Polyline> {
        if self.pos.len() < 10 {
            return None;
        }

        let mut ret = Polyline::new();
        let mut first = 0;
        let mut second = 1;
        let mut third = 2;

        // first point
        ret.push(Point2D::new(self.pos[first].x, self.pos[first].y));

        for _ in 0..(self.pos.len()-2) {
            let dist = point_line_distance(
                self.pos[third],
                self.pos[first],
                self.pos[second]
            );

            if dist <= tol {
                third = third+1;
            } else {
                ret.push(Point2D::new(self.pos[third].x, self.pos[third].y));
                first = second;
                second = third;
                third = third+1;
            }
        }

        // last point
        ret.push(Point2D::new(self.pos[self.pos.len()-1].x, self.pos[self.pos.len()-1].y));

        Some(ret)
    }

    pub fn interp(&self) -> Path {
        Path::from_polyline(self)
    }

    pub fn connect_with_line(&self, xpr: &Xprite) -> Option<Vec<Pixel>> {
        let mut ret = Vec::new();
        for (p0, p1) in self.pos.iter().zip(self.pos[1..].iter()) {
            let p0 = xpr.canvas.shrink_size(p0);
            let p1 = xpr.canvas.shrink_size(p1);
            let seg = bresenham(&p0, &p1);
            ret.extend(&seg);
        }

        // console!(log, ret.0.len() as i32);
        ret.dedup();
        Some(ret)
    }

}

/// distance from p0 to p1--p2
pub fn point_line_distance( p0: Point2D<f32>, p1: Point2D<f32>, p2: Point2D<f32>) -> f32 {
    ((p2.x-p1.x)*(p1.y-p0.y)-(p1.x-p0.x)*(p2.y-p1.y)).abs()
    /
    ((p2.x-p1.x)*(p2.x-p1.x)+(p2.y-p1.y)*(p2.y as f32-p1.y as f32)).sqrt()
}
