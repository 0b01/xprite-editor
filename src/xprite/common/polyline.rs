use xprite::Path;
use lyon_geom::euclid::Point2D;

#[derive(Clone)]
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

    pub fn push(&mut self, x: f32, y: f32) {
        self.pos.push(Point2D::new(x, y))
    }

    pub fn clear(&mut self) {
        self.pos.clear()
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
        ret.push(self.pos[first].x, self.pos[first].y);

        for _ in 0..(self.pos.len()-2) {
            let dist = point_line_distance(
                self.pos[third],
                self.pos[first],
                self.pos[second]
            );

            if dist <= tol {
                third = third+1;
            } else {
                ret.push(self.pos[third].x, self.pos[third].y);
                first = second;
                second = third;
                third = third+1;
            }
        }

        // last point
        ret.push(self.pos[self.pos.len()-1].x, self.pos[self.pos.len()-1].y);

        Some(ret)
    }

    pub fn interp(&self) -> Path {
        Path::from_polyline(self)
    }

}

/// distance from p0 to p1--p2
fn point_line_distance( p0: Point2D<f32>, p1: Point2D<f32>, p2: Point2D<f32>) -> f32 {
    ((p2.x-p1.x)*(p1.y-p0.y)-(p1.x-p0.x)*(p2.y-p1.y)).abs()
    /
    ((p2.x-p1.x)*(p2.x-p1.x)+(p2.y-p1.y)*(p2.y-p1.y)).sqrt()
}
