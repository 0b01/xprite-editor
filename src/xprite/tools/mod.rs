use xprite::prelude::*;
use stdweb::web::event::MouseButton;

pub mod pencil;
pub mod line;
pub mod paint_bucket;

pub trait Tool {
    fn get_name(&self) -> &'static str;
    fn mouse_move(&mut self, &mut Xprite, p: Point2D<i32>);
    fn mouse_up(&mut self, &mut Xprite, p: Point2D<i32>);
    fn mouse_down(&mut self, &mut Xprite, p: Point2D<i32>, button: MouseButton);
    fn draw(&self, &Xprite);
    fn set(&mut self, &mut Xprite, option: &str, value: &str);
}
