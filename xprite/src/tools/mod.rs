use crate::prelude::*;

pub mod pencil;
pub mod line;
pub mod paint_bucket;

pub trait Tool {
    fn get_name(&self) -> &'static str;
    fn mouse_move(&mut self, xpr:&mut Xprite, p: Point2D<f32>) -> Option<()>;
    fn mouse_up(&mut self, xpr:&mut Xprite, p: Point2D<f32>) -> Option<()>;
    fn mouse_down(&mut self, xpr:&mut Xprite, p: Point2D<f32>, button: InputItem) -> Option<()>;
    fn draw(&self, xpr:&mut Xprite) -> Option<()>;
    fn set(&mut self, xpr:&mut Xprite, option: &str, value: &str) -> Option<()>;
}
