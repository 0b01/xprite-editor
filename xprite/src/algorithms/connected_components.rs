use crate::prelude::*;
use super::floodfill::floodfill;

pub fn connected_components(pixs: &Pixels, w: usize, h: usize) -> Vec<Pixels> {
    let mut pixs = pixs.clone();
    let mut ret = Vec::new();
    while !pixs.is_empty() {
        let grid = pixs.as_mat(w, h);
        let p = get_first_true(&grid);
        if p.is_none() {break} // possible when some pixels are oob
        else {
            let p = p.unwrap();
            let origin = p.point;
            let bg_col = Some(p.color);
            let connected = floodfill(w as f32, h as f32, &pixs, origin, bg_col, Color::red());
            let cc = pixs.intersection(&connected);
            ret.push(cc);
            pixs.sub(&connected);
        }
    }
    ret
}

fn get_first_true(grid: &[Vec<Option<Pixel>>]) -> Option<Pixel> {
    for row in grid.iter() {
        for cell in row.iter() {
            if cell.is_some() {
                return cell.clone()
            }
        }
    }
    None
}

// /// invariant: pixels must be contiuous
// pub fn connected_components(pixs: &Pixels, w: usize, h: usize) -> Vec<Pixels> {
//     let mut ret = vec![];

//     let mut offset = pixs.iter();
//     let _ = offset.next();

//     let mut temp = Pixels::new();
//     for (p0, p1) in pixs.iter().zip(offset) {
//         temp.push(p0.clone());
//         if connected(p0, p1) {
//             temp.push(p1.clone());
//         } else {
//             ret.push(temp.clone());
//             temp.clear();
//             if p1 == pixs.iter().last().unwrap() {
//                 temp.push(p1.clone());
//             }
//             ret.push(temp.clone());
//             temp.clear();
//         }
//     }
//     if !temp.is_empty() { ret.push(temp.clone()); }

//     ret
// }

// fn connected(p0: &Pixel, p1: &Pixel) -> bool {
//     let ret = (p0.point.x == p1.point.x
//     && (p0.point.y - p1.point.y).abs() == 1.
//     ) ||  (
//     p0.point.y == p1.point.y
//     && (p0.point.x - p1.point.x).abs() == 1.
//     );
//     ret
// }


#[cfg(test)]
mod tests {
    #[test]
    fn test_connected_components() {
        use super::*;
        assert_eq!(
            connected_components(&pixels!{
                pixel!(0.,0., Color::red()),
                pixel!(0.,1., Color::red()),
                pixel!(1.,1., Color::red())
            }, 3, 3),
            vec![pixels![
                pixel!(0.,0., Color::red()),
                pixel!(0.,1., Color::red()),
                pixel!(1.,1., Color::red())
            ]]
        );

        assert_eq!(
            connected_components(&pixels!{
                pixel!(0.,0., Color::red()),
                pixel!(1.,1., Color::red())
            }, 3, 3),
            vec![
                pixels![ pixel!(0.,0., Color::red()) ],
                pixels![ pixel!(1.,1., Color::red()) ]
            ]
        );

    }
}