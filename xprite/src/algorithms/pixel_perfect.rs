use crate::prelude::*;

pub fn pixel_perfect(path: &Pixels) -> Pixels {
    if path.len() == 1 || path.len() == 0 {
        return path.clone();
    }
    let mut ret = Pixels::new();
    let mut c = 0;

    while c < path.len() {
        if c > 0
            && c + 1 < path.len()
            && (path[c - 1].point.x == path[c].point.x || path[c - 1].point.y == path[c].point.y)
            && (path[c + 1].point.x == path[c].point.x || path[c + 1].point.y == path[c].point.y)
            && path[c - 1].point.x != path[c + 1].point.x
            && path[c - 1].point.y != path[c + 1].point.y
        {
            c += 1;
        }

        ret.push(path[c]);

        c += 1;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pp() {
        let path = pixels!(
            pixel!(0., 0., Color::red()),
            pixel!(0., 1., Color::red()),
            pixel!(1., 1., Color::red())
        );

        let ret = pixel_perfect(&path);
        println!("{:#?}", ret);
    }
}
