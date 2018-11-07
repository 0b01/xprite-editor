pub trait Renderer {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_fill_style_color(&self, color: &str);
    fn fill_rect(&self, a:f64,b:f64,c:f64,d:f64);
}
