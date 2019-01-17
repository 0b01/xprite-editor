use crate::prelude::*;

/// flood fill algorithm
/// converts pixels into a grid of size (w, h)
/// The operation starts at origin
pub fn floodfill(
    w: f32,
    h: f32,
    pix: &Pixels,
    origin: Vec2f,
    bg_col: Option<Color>,
    color: Color,
) -> Pixels {
    let mut ret = Pixels::new();
    let canvas = pix.as_mat(w as usize, h as usize);
    let mut stack = vec![origin];

    let mut visited = vec![vec![false; w as usize]; h as usize];
    let mut neighbors = Vec::with_capacity(4);
    while let Some(point) = stack.pop() {
        let Vec2f { x, y } = point;
        match (bg_col, canvas[y as usize][x as usize]) {
            (Some(bg), Some(Pixel { color, .. })) => {
                if bg != color {
                    continue;
                }
            }
            (None, Some(_)) => continue,
            (Some(_), None) => continue,
            (None, None) => (),
        };
        neighbors.clear(); // Checking only 4 neighbors
        if x < w - 1. {
            neighbors.push((x + 1., y))
        };
        if x > 0. {
            neighbors.push((x - 1., y))
        };
        if y < h - 1. {
            neighbors.push((x, y + 1.))
        };
        if y > 0. {
            neighbors.push((x, y - 1.))
        };
        for &(nx, ny) in neighbors.iter() {
            if visited[ny as usize][nx as usize] {
                continue;
            };
            stack.push(Vec2f { x: nx, y: ny });
            visited[ny as usize][nx as usize] = true;
        }
        ret.push(Pixel { point, color });
        visited[y as usize][x as usize] = true;
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;

    // #[test]
    // fn test_as_mat() {
    //     let mut pixs = Pixels::new();
    //     pixs.push(pixel!(0., 0., Color::blue()));
    //     let arr = pixs.as_mat(2, 2);
    //     assert_eq!(
    //         vec![
    //             vec![pixel!(0,0,Color::blue()), pixel!(0,1,Color::red())],
    //             vec![pixel!(1,0,Color::red()), pixel!(1,1,Color::red())]
    //         ],
    //         arr
    //     );
    // }

    #[test]
    fn test_floodfill() {
        // xx
        // oo
        let mut pixs = Pixels::new();
        pixs.push(pixel!(0., 0., Color::black()));
        pixs.push(pixel!(0., 1., Color::black()));
        let to_fill = floodfill(2., 2., &pixs, Vec2f { x: 1., y: 1. }, None, Color::red());
        assert_eq!(
            Pixels::from_slice(&vec![
                pixel!(1, 1, Color::red()),
                pixel!(1, 0, Color::red()),
            ]),
            to_fill
        );
    }

    #[test]
    fn test_floodfill2() {
        //  x
        // xox
        //  x
        let mut pixs = Pixels::new();
        pixs.push(pixel!(1., 0., Color::black()));
        pixs.push(pixel!(0., 1., Color::black()));
        pixs.push(pixel!(1., 2., Color::black()));
        pixs.push(pixel!(2., 1., Color::black()));

        let to_fill = floodfill(
            100.,
            100.,
            &pixs,
            Vec2f { x: 1., y: 1. },
            None,
            Color::black(),
        );
        assert_eq!(
            Pixels::from_slice(&vec![pixel!(1, 1, Color::black()),]),
            to_fill
        );

        let to_fill = floodfill(
            100.,
            100.,
            &pixs,
            Vec2f { x: 1., y: 1. },
            None,
            Color::black(),
        );
        assert_eq!(
            Pixels::from_slice(&vec![pixel!(1, 1, Color::black()),]),
            to_fill
        );
    }

    #[test]
    fn test_floodfill3() {
        //  xx
        // xoox
        //  xx
        let mut pixs = Pixels::new();
        pixs.push(pixel!(0., 1., Color::black()));
        pixs.push(pixel!(0., 2., Color::black()));
        pixs.push(pixel!(1., 0., Color::black()));
        pixs.push(pixel!(1., 3., Color::black()));
        pixs.push(pixel!(2., 1., Color::black()));
        pixs.push(pixel!(2., 2., Color::black()));

        let to_fill = floodfill(4., 3., &pixs, Vec2f { x: 1., y: 1. }, None, Color::blue());
        assert_eq!(
            Pixels::from_slice(&vec![
                pixel!(1, 1, Color::blue()),
                pixel!(1, 2, Color::blue()),
            ]),
            to_fill
        );
    }
}
