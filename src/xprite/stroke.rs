#[derive(Serialize)]
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

    pub fn reumann_witkam(&self, tol: f32) -> Stroke {
        let mut ret = Stroke::new();
        let mut first = 0;
        let mut second = 1;
        let mut third = 2;

        for _ in 0..(self.pos.len()-2) {
            let dist = point_line_distance(
                self.pos[third].0 as f32,
                self.pos[third].1 as f32,
                self.pos[first].0 as f32,
                self.pos[first].1 as f32,
                self.pos[second].0 as f32,
                self.pos[second].1 as f32
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
        ret
    }
}

fn point_line_distance(x0: f32, y0: f32, x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
    ((x2-x1)*(y1-y0)-(x1-x0)*(y2-y1)).abs()
    /
    ((x2-x1)*(x2-x1)+(y2-y1)*(y2-y1)).sqrt()
}
