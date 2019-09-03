use crate::prelude::*;
pub enum MouseCursorType {
    Hand,
    None,
}

pub trait Renderer {
    fn time(&self) -> f32 {
        0.
    }
    fn width(&self) -> f64;
    fn height(&self) -> f64;
    fn rect(&mut self, p0: [f64; 2], p1: [f64; 2], color: [f32; 4], filled: bool);
    fn pixel(&mut self, x: f64, y: f64, color: [f32; 4], filled: bool);
    fn circ(&mut self, p0: [f64; 2], r: f64, color: [f32; 4], filled: bool);
    fn line(&mut self, p0: [f64; 2], p1: [f64; 2], color: [f32; 4]);
    fn bezier(&mut self, p0: [f64; 2], cp1: [f64; 2], cp2: [f64; 2], p1: [f64; 2], color: [f32; 4], thickness: f64);
    #[allow(unused)]
    fn set_mouse_cursor(&mut self, cursor_type: MouseCursorType) {}
    fn render(&mut self, _xpr: Option<&Xprite>) -> Option<()> {
        Option::None
    }
    fn reset(&mut self) {}
    fn add_img(&mut self, _img: img::DynamicImage, _format: img::ColorType) -> usize {
        0
    }
}
