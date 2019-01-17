use crate::prelude::*;
use std::i32;

/// draw the outline of a rectangle
pub fn rect(x1: i32, y1: i32, x2:i32, y2:i32, col: Color) -> Result<Pixels, String> {
    if y2 == 0 || x2 == 0 { return Err("i32 must be greater than 0".to_owned()); }
    let mut ret = Pixels::new();
    for i in x1..x2 {
        ret.push(pixel!(y1, i, col));
        ret.push(pixel!(y2-1, i, col));
    }
    for j in y1..y2 {
        ret.push(pixel!(j, x1, col));
        ret.push(pixel!(j, x2-1, col));
    }
    Ok(ret)
}

/// draw a filled rectangle
pub fn filled_rect(x1: i32, y1: i32, x2: i32, y2: i32, col: Color) -> Result<Pixels, String> {
    let mut ret = Pixels::new();
    for i in x1..x2 {
        for j in y1..y2 {
            ret.push(pixel!(j, i, col));
        }
    }
    Ok(ret)
}

pub fn get_rect(start: Option<Pixel>, stop: Option<Pixel>, filled: bool) -> Result<Pixels, String> {
    let start = start.ok_or_else(||"start is none".to_owned())?;
    let stop = stop.ok_or_else(||"stop is none".to_owned())?;
    let x0 = start.point.x as i32;
    let y0 = start.point.y as i32;
    let x1 = stop.point.x as i32;
    let y1 = stop.point.y as i32;
    let f = if filled {filled_rect} else {rect};
    f(
        i32::min(x0, x1),
        i32::min(y0, y1),
        i32::max(x0, x1),
        i32::max(y0, y1),
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