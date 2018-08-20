use xprite::Pixels;

#[derive(Serialize, Clone)]
pub struct Stroke {
    pub pos: Vec<(u32, u32)>,
}

js_serializable!(Stroke);

impl Stroke {
    pub fn new() -> Self {
        Self {
            pos: Vec::new(),
        }
    }

    pub fn push(&mut self, x: u32, y: u32) {
        self.pos.push((x, y))
    }

    pub fn clear(&mut self) {
        self.pos.clear()
    }

    pub fn reumann_witkam(&self, tol: f32) -> Option<Stroke> {
        if self.pos.len() < 10 {
            return None;
        }

        let mut ret = Stroke::new();
        let mut first = 0;
        let mut second = 1;
        let mut third = 2;

        // first point
        ret.push(self.pos[first].0, self.pos[first].1);
        // last point
        ret.push(self.pos[self.pos.len()-1].0, self.pos[self.pos.len()-1].1);

        for _ in 0..(self.pos.len()-2) {
            let dist = point_line_distance(
                self.pos[third],
                self.pos[first],
                self.pos[second]
            );

            if dist <= tol {
                third = third+1;
            } else {
                ret.push(self.pos[third].0, self.pos[third].1);
                first = second;
                second = third;
                third = third+1;
            }
        }
        Some(ret)
    }

    pub fn rasterize(&self) -> Pixels {
        // ... todo
        Pixels::new()
    }
}

fn point_line_distance(
    (x0, y0): (u32, u32),
    (x1, y1): (u32, u32),
    (x2, y2): (u32, u32)
) -> f32 {
    let x0 = x0 as f32;
    let x1 = x1 as f32;
    let x2 = x2 as f32;
    let y0 = y0 as f32;
    let y1 = y1 as f32;
    let y2 = y2 as f32;

    ((x2-x1)*(y1-y0)-(x1-x0)*(y2-y1)).abs()
    /
    ((x2-x1)*(x2-x1)+(y2-y1)*(y2-y1)).sqrt()
}
