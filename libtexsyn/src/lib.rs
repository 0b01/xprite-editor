//#[macro_use]
//extern crate approx;
//extern crate conv;
#[macro_use]
extern crate error_chain;
extern crate imageproc;
extern crate image as img;
//extern crate noise;
extern crate num_traits;
extern crate rand;
extern crate rayon;

mod common;
pub mod distance;
pub mod errors;
pub mod generators;

pub mod image {
    pub use img::*;
}
