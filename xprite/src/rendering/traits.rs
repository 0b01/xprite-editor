pub enum MouseCursorType {
    Hand,
}

pub trait Renderer {
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn rect(&mut self, p0:[f32;2], p1:[f32;2], color:[f32;4], filled: bool);
    fn circ(&mut self, p0:[f32;2], r:f32, color:[f32;4], filled: bool);
    fn line(&mut self, p0:[f32;2], p1:[f32;2], color:[f32;4]);
    fn bezier(&mut self, p0:[f32;2], cp1:[f32;2], cp2: [f32;2], p1:[f32;2], color:[f32;4], thickness: f32);
    #[allow(unused)]
    fn set_mouse_cursor(&mut self, cursor_type: MouseCursorType) {}
    fn render(&mut self) {}
    fn reset(&mut self) {}
    fn add_img(&mut self, _img: img::DynamicImage, _format: img::ColorType) -> usize { 0 }
}
