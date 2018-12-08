use crate::prelude::*;

pub struct Palette {
    colors: Vec<Color>,
}

impl Palette {
    pub fn new() -> Self {
        let mut colors = Vec::new();
        colors.push( Color{r: 255, g: 255, b: 255, a: 255} );
        colors.push( Color{r: 255, g: 0  , b: 0  , a: 255} );
        colors.push( Color{r: 0  , g: 255, b: 0  , a: 255} );
        colors.push( Color{r: 0  , g: 0  , b: 255, a: 255} );

        Self {
            colors,
        }
    }
}