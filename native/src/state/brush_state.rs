// TODO: remove this state. directly modify tool brush state w/ msg

pub struct BrushState {
    pub sz: [i32; 2],
}

impl Default for BrushState {
    fn default() -> Self {
        Self { sz: [1, 0] }
    }
}
