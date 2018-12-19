use crate::prelude::*;
use std::u32;

/// draw the outline of a rectangle
pub fn rect(x1: u32, y1: u32, x2:u32, y2:u32, col: Color) -> Result<Pixels, String> {
    if y2 == 0 || x2 == 0 { return Err("u32 must be greater than 0".to_owned()); }
    let mut ret = Pixels::new();
    for i in x1..x2 {
        ret.push(pixel!(i,y1, col));
        ret.push(pixel!(i,y2-1, col));
    }
    for j in y1..y2 {
        ret.push(pixel!(x1, j, col));
        ret.push(pixel!(x2-1, j, col));
    }
    Ok(ret)
}

/// draw a filled rectangle
pub fn filled_rect(x1: u32, y1: u32, x2: u32, y2: u32, col: Color) -> Result<Pixels, String> {
    let mut ret = Pixels::new();
    for i in x1..x2 {
        for j in y1..y2 {
            ret.push(pixel!(i,j, col));
        }
    }
    Ok(ret)
}

pub fn get_rect(start: Option<Pixel>, stop: Option<Pixel>, filled: bool) -> Result<Pixels, String> {
    let start = start.ok_or("start is none".to_owned())?;
    let stop = stop.ok_or("stop is none".to_owned())?;
    let x0 = start.point.x as u32;
    let y0 = start.point.y as u32;
    let x1 = stop.point.x as u32;
    let y1 = stop.point.y as u32;
    let f = if filled {filled_rect} else {rect};
    f(
        u32::min(x0, x1),
        u32::min(y0, y1),
        u32::max(x0, x1),
        u32::max(y0, y1),
        Color::red(),
    )
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_filled() {
        use super::*;
        let rect = filled_rect(0, 0, 3, 3, Color::red());
        let mut exp = Pixels::new();
        exp.push(pixel!(0,0, Color::red()));
        exp.push(pixel!(0,1, Color::red()));
        exp.push(pixel!(0,2, Color::red()));
        exp.push(pixel!(1,0, Color::red()));
        exp.push(pixel!(1,1, Color::red()));
        exp.push(pixel!(1,2, Color::red()));
        exp.push(pixel!(2,0, Color::red()));
        exp.push(pixel!(2,1, Color::red()));
        exp.push(pixel!(2,2, Color::red()));
        assert_eq!(Ok(exp), rect);
    }

    #[test]
    fn test_unfilled() {
        use super::*;
        let rect = rect(0, 0, 3, 3, Color::red());
        let mut exp = Pixels::new();
        exp.push(pixel!(0,0, Color::red()));
        exp.push(pixel!(0,2, Color::red()));
        exp.push(pixel!(1,0, Color::red()));
        exp.push(pixel!(1,2, Color::red()));
        exp.push(pixel!(2,0, Color::red()));
        exp.push(pixel!(2,2, Color::red()));
        exp.push(pixel!(0,1, Color::red()));
        exp.push(pixel!(2,1, Color::red()));
        assert_eq!(Ok(exp), rect);
    }
}