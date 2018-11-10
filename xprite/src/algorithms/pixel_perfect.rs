use crate::prelude::*;

pub fn pixel_perfect(path: &[Pixel]) -> Vec<Pixel> {
    if path.len() == 1 || path.len() == 0 {
        return path.iter().cloned().collect();
    }
    let mut ret = Vec::new();
    let mut c = 0;

    while c < path.len() {

      // We ignore a pixel that is between other two pixels in the
      // corner of a L-like shape.
      if c > 0 && c+1 < path.len()
        && (path[c-1].point.x == path[c].point.x || path[c-1].point.y == path[c].point.y)
        && (path[c+1].point.x == path[c].point.x || path[c+1].point.y == path[c].point.y)
        && path[c-1].point.x != path[c+1].point.x
        && path[c-1].point.y != path[c+1].point.y {

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
    let path = vec![
      pixel!(0.,0.),
      pixel!(0.,1.),
      pixel!(1.,1.),
    ];

    let ret = pixel_perfect(&path);
    println!("{:#?}", ret);
  }
}
