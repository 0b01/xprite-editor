pub enum MouseCursorType {
    Move,
}

pub trait Renderer {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn rect(&self, p0:[f32;2], p1:[f32;2], color:[f32;4], filled: bool);
    fn circ(&self, p0:[f32;2], r:f32, color:[f32;4], filled: bool);
    fn line(&self, p0:[f32;2], p1:[f32;2], color:[f32;4]);
    fn bezier(&self, p0:[f32;2], cp1:[f32;2], cp2: [f32;2], p1:[f32;2], color:[f32;4], thickness: f32);
    fn set_mouse_cursor(&self, cursor_type: MouseCursorType);
    fn render(&mut self) {}
}
