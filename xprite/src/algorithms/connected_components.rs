use crate::prelude::*;

pub fn connected_components(pixs: &Pixels, w: usize, h: usize) -> Vec<Pixels> {
    let mut ret = Vec::new();
    let mut canvas = pixs.as_mat(w, h);

    let ff = |canvas: &[Vec<Option<Pixel>>],
              origin: Vec2f,
              bg_col: Option<Color>| {
        let mut cc = Pixels::new();
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
            if x < w as f64 - 1. {
                neighbors.push((x + 1., y))
            };
            if x > 0. {
                neighbors.push((x - 1., y))
            };
            if y < h as f64 - 1. {
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
            cc.push(Pixel {
                point,
                color: Color::red(),
            });
            visited[y as usize][x as usize] = true;
        }
        cc
    };

    while let Some(p) = get_first_true(&canvas) {
        let connected = ff(&canvas, p.point, Some(p.color));
        for &Pixel {
            point: Vec2f { x, y },
            ..
        } in connected.iter()
        {
            canvas[y as usize][x as usize] = None;
        }
        ret.push(connected);
    }
    ret
}

fn get_first_true(grid: &[Vec<Option<Pixel>>]) -> Option<Pixel> {
    for row in grid.iter() {
        for cell in row.iter() {
            if cell.is_some() {
                return *cell
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_connected_components() {
        use super::*;
        assert_eq!(
            /*
             *  ##.
             *  .#.
             *  ...
             */
            connected_components(
                &pixels! {
                    pixel!(0.,0., Color::red()),
                    pixel!(0.,1., Color::red()),
                    pixel!(1.,1., Color::red())
                },
                3,
                3
            ),
            vec![pixels![
                pixel!(0., 0., Color::red()),
                pixel!(0., 1., Color::red()),
                pixel!(1., 1., Color::red())
            ]]
        );

        assert_eq!(
            /*
             *  #.
             *  .#
             */
            connected_components(
                &pixels! {
                    pixel!(0.,0., Color::red()),
                    pixel!(1.,1., Color::red())
                },
                3,
                3
            ),
            vec![
                pixels![pixel!(0., 0., Color::red())],
                pixels![pixel!(1., 1., Color::red())]
            ]
        );

        assert_eq!(
            /*
             *  #.
             *  .#
             *  .#
             */
            connected_components(
                &pixels! {
                    pixel!(0.,0., Color::red()),
                    pixel!(1.,1., Color::red()),
                    pixel!(2.,1., Color::red())
                },
                3,
                3
            ),
            vec![
                pixels![pixel!(0., 0., Color::red())],
                pixels![
                    pixel!(1., 1., Color::red()),
                    pixel!(2., 1., Color::red())
                ]
            ]
        );
    }

    #[test]
    fn test_connected_components_oob() {
        use super::*;
        assert_eq!(
            /*
             *  ##.
             *  .#.
             *  ...
             */
            connected_components(
                &pixels! {
                    pixel!(0.,0., Color::red()),
                    pixel!(0.,1., Color::red()),
                    pixel!(1.,1., Color::red())
                },
                1,
                1
            ),
            vec![pixels![pixel!(0., 0., Color::red())]]
        );
    }

}
