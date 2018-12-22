extern crate image as img;
extern crate num_traits;
extern crate rand;
extern crate rayon;

mod common;
pub mod distance;
pub mod generators;

pub mod image {
    pub use img::*;
}
