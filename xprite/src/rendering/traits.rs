pub trait Renderer {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_fill_style_color(&self, color: &str);
    fn fill_rect(&self, a:f32,b:f32,c:f32,d:f32);
}
