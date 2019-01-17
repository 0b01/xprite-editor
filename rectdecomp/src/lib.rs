extern crate intervaltree;
use intervaltree::{IntervalTree, Element};
use std::ops::Sub;
use std::ops::Range;

// "use strict"

// let bipartiteIndependentSet = require("bipartite-independent-set")
// let create_interval_tree = require("interval-tree-1d")
// let dup = require("dup")

// module.exports = decomposeRegion

// function Vertex(point, path, index, concave) {
//   this.point = point
//   this.path = path
//   this.index = index
//   this.concave = concave
//   this.next = null
//   this.prev = null
//   this.visited = false
// }

struct Vertex<T: Copy> {
    point: [T;2],
    path: usize,
    index: usize,
    concave: bool,
    next: Option<Vertex<T>>,
    prev: Option<Vertex<T>>,
    visited: bool,
}

impl<T:Copy> Vertex<T> {
    fn new(point: [T;2], path: usize, index: usize, concave: bool) -> Self {
        Self {
            point, path, index, concave, next:None, prev:None, visited: false
        }
    }
}

fn get_diagonals<T: Copy+Sub + PartialEq + Ord>(
  vertices: &[Vertex<T>],
  paths: &[Vec<Vertex<T>>],
  direction:usize,
  tree: IntervalTree<T, Segment<T>>
) -> Vec<Segment<T>> {
  let mut concave = vec![];
  for i in 0..vertices.len() {
    if vertices[i].borrow().concave {
      concave.push(vertices[i])
    }
  }
  concave.sort_by(|a,b| {
    if a.borrow().point[direction] != b.borrow().point[direction] {
      return ::std::cmp::Ordering::Greater;
    }
    return a.borrow().point[direction^1].cmp(&b.borrow().point[direction^1])
  });
  let mut diagonals = vec![];
  for i in 0..concave.len() {
    let a = concave[i-1];
    let b = concave[i];
    if a.borrow().point[direction] == b.borrow().point[direction] {
      if a.borrow().path == b.borrow().path {
        let n = paths[a.borrow().path].len();
        let d = (a.borrow().index-b.borrow().index+n) % n;
        if d == 1 || d == n-1 {
          continue
        }
      }
      if !test_segment(a, b, tree, direction) {
        //Check orientation of diagonal
        diagonals.push(Segment::new(a, b, direction));
      }
    }
  }
  return diagonals
}

fn test_segment<T: Copy+Ord>(a: Vertex<T>, b: Vertex<T>, tree: IntervalTree<T, Segment<T>>, direction: usize) -> bool {
    let ax = a.borrow().point[direction^1];
    let bx = b.borrow().point[direction^1];
    let s = tree.query_point(a.borrow().point[direction]).next().unwrap();
    let x = s.value.start.borrow().point[direction^1];
    if ax < x && x < bx {
        return true
    }
    return false
}


struct Segment<T: PartialOrd+Copy> {
    range: Range<T>,
    start: Vertex<T>,
    end: Vertex<T>,
    direction: usize,
    number: i32,
}

impl<T:PartialOrd+Copy> Into<Element<T,Segment<T>>> for Segment<T> {
    fn into(self) -> Element<T, Segment<T>> {
        Element {
            range: self.range,
            value: self,
        }
    }
}

impl<T:Copy+ PartialOrd> Segment<T> {
    fn new(start: Vertex<T>, end: Vertex<T>, direction: usize) -> Self{
        let a = start.point[(direction^1) as usize];
        let b = end.point[(direction^1) as usize];
        let range = if a < b {
            a..b
        } else {
            b..a
        };
        Self {
            range,
            start,
            end,
            direction,
            number: -1,
        }
    }
}

// //Find all crossings between diagonals
fn find_crossings<T:Copy+PartialOrd+Ord+Clone>(hdiagonals:&[Segment<T>], vdiagonals:&[Segment<T>]) -> Vec<[Segment<T>;2]> {
  let htree: IntervalTree<T,Segment<T>> = hdiagonals.into_iter().map(|i|i.into()).collect();
  let mut crossings = vec![];
  for i in 0..vdiagonals.len() {
    let v = vdiagonals[i];
    let x = v.start.point[0];
    let h = htree.query_point(v.start.point[1]).next().unwrap();
    let x = h.value.start.point[0];
    if v.range.start <= x && x <= v.range.end {
      crossings.push([h.value, v])
    }
  }
  crossings
}

fn find_splitters<T: Ord+Copy>(hdiagonals: &[Segment<T>], vdiagonals:&[Segment<T>]) {
  //First find crossings
  let crossings = find_crossings(hdiagonals, vdiagonals);

  //Then tag and convert edge format
  for i in 0..hdiagonals.len() {
    hdiagonals[i].number = i as i32;
  }
  for i in 0..vdiagonals.len() {
    vdiagonals[i].number = i as i32;
  }
  let edges = crossings.iter().map(|c|
    [ c[0].number, c[1].number ]
  );

  //Find independent set
  let selected = bipartiteIndependentSet(hdiagonals.length, vdiagonals.length, edges);

  //Convert into result format
  let result = new Array(selected[0].length + selected[1].length)
  let ptr = 0
  for(let i=0; i<selected[0].length; ++i) {
    result[ptr++] = hdiagonals[selected[0][i]]
  }
  for(let i=0; i<selected[1].length; ++i) {
    result[ptr++] = vdiagonals[selected[1][i]]
  }

  unimplemented!()

  //Done
  return result
}

fn splitSegment<T: Ord>(segment: &mut Segment<T>) {
  //Store references
  let a = segment.start;
  let b = segment.end;
  let pa = a.borrow().prev.unwrap();
  let na = a.borrow().next.unwrap();
  let pb = b.borrow().prev.unwrap();
  let nb = b.borrow().next.unwrap();

  //Fix concavity
  a.borrow().concave = false;
  b.borrow().concave = false;

  //Compute orientation
  let ao = pa.borrow().point[segment.direction] == a.borrow().point[segment.direction];
  let bo = pa.borrow().point[segment.direction] == b.borrow().point[segment.direction];

  if ao && bo {
    //Case 1:
    //            ^
    //            |
    //  --->A+++++B<---
    //      |
    //      V
    a.borrow().prev = Some(pb);
    pb.borrow().next = Some(a);
    b.borrow().prev = Some(pa);
    pa.borrow().next = Some(b);
  } else if ao && !bo {
    //Case 2:
    //      ^     |
    //      |     V
    //  --->A+++++B--->
    //
    //
    a.borrow().prev = Some(b);
    b.borrow().next = Some(a);
    pa.borrow().next = Some(nb);
    nb.borrow().prev = Some(pa);
  } else if(!ao && bo) {
    //Case 3:
    //
    //
    //  <---A+++++B<---
    //      ^     |
    //      |     V
    a.borrow().next = Some(b);
    b.borrow().prev = Some(a);
    na.borrow().prev = Some(pb);
    pb.borrow().next = Some(na);

  } else if !ao && !bo {
    //Case 3:
    //            |
    //            V
    //  <---A+++++B--->
    //      ^
    //      |
    a.borrow().next = Some(nb);
    nb.borrow().prev = Some(a);
    b.borrow().next = Some(na);
    na.borrow().prev = Some(b);
  }
}

// function findLoops(vertices) {
//   //Initialize visit flag
//   for(let i=0; i<vertices.length; ++i) {
//     vertices[i].visited = false
//   }
//   //Walk over vertex list
//   let loops = []
//   for(let i=0; i<vertices.length; ++i) {
//     let v = vertices[i]
//     if(v.visited) {
//       continue
//     }
//     //Walk along loop
//     let loop = []
//     while(!v.visited) {
//       loop.push(v)
//       v.visited = true
//       v = v.next
//     }
//     loops.push(loop)
//   }
//   return loops
// }


// function splitConcave(vertices) {
//   //First step: build segment tree from vertical segments
//   let leftsegments = []
//   let rightsegments = []
//   for(let i=0; i<vertices.length; ++i) {
//     let v = vertices[i]
//     if(v.next.point[1] === v.point[1]) {
//       if(v.next.point[0] < v.point[0]) {
//         leftsegments.push(new Segment(v, v.next, 1))
//       } else {
//         rightsegments.push(new Segment(v, v.next, 1))
//       }
//     }
//   }
//   let lefttree = create_interval_tree(leftsegments)
//   let righttree = create_interval_tree(rightsegments)
//   for(let i=0; i<vertices.length; ++i) {
//     let v = vertices[i]
//     if(!v.concave) {
//       continue
//     }

//     //Compute orientation
//     let y = v.point[1]
//     let direction
//     if(v.prev.point[0] === v.point[0]) {
//       direction = v.prev.point[1] < y
//     } else {
//       direction = v.next.point[1] < y
//     }
//     direction = direction ? 1 : -1

//     //Scan a horizontal ray
//     let closestSegment = null
//     let closestDistance = Infinity * direction
//     if(direction < 0) {
//       righttree.queryPoint(v.point[0], function(h) {
//         let x = h.start.point[1]
//         if(x < y && x > closestDistance) {
//           closestDistance = x
//           closestSegment = h
//         }
//       })
//     } else {
//       lefttree.queryPoint(v.point[0], function(h) {
//         let x = h.start.point[1]
//         if(x > y && x < closestDistance) {
//           closestDistance = x
//           closestSegment = h
//         }
//       })
//     }

//     //Create two splitting vertices
//     let splitA = new Vertex([v.point[0], closestDistance], 0, 0, false)
//     let splitB = new Vertex([v.point[0], closestDistance], 0, 0, false)

//     //Clear concavity flag
//     v.concave = false

//     //Split vertices
//     splitA.prev = closestSegment.start
//     closestSegment.start.next = splitA
//     splitB.next = closestSegment.end
//     closestSegment.end.prev = splitB

//     //Update segment tree
//     let tree
//     if(direction < 0) {
//       tree = righttree
//     } else {
//       tree = lefttree
//     }
//     tree.remove(closestSegment)
//     tree.insert(new Segment(closestSegment.start, splitA, 1))
//     tree.insert(new Segment(splitB, closestSegment.end, 1))

//     //Append vertices
//     vertices.push(splitA, splitB)

//     //Cut v, 2 different cases
//     if(v.prev.point[0] === v.point[0]) {
//       // Case 1
//       //             ^
//       //             |
//       // --->*+++++++X
//       //     |       |
//       //     V       |
//       splitA.next = v
//       splitB.prev = v.prev
//     } else {
//       // Case 2
//       //     |       ^
//       //     V       |
//       // <---*+++++++X
//       //             |
//       //             |
//       splitA.next = v.next
//       splitB.prev = v
//     }

//     //Fix up links
//     splitA.next.prev = splitA
//     splitB.prev.next = splitB
//   }
// }

// function findRegions(vertices) {
//   let n = vertices.length
//   for(let i=0; i<n; ++i) {
//     vertices[i].visited = false
//   }
//   //Walk over vertex list
//   let rectangles = []
//   for(let i=0; i<n; ++i) {
//     let v = vertices[i]
//     if(v.visited) {
//       continue
//     }
//     //Walk along loop
//     let lo = [ Infinity, Infinity ]
//     let hi = [-Infinity,-Infinity ]
//     while(!v.visited) {
//       for(let j=0; j<2; ++j) {
//         lo[j] = Math.min(v.point[j], lo[j])
//         hi[j] = Math.max(v.point[j], hi[j])
//       }
//       v.visited = true
//       v = v.next
//     }
//     rectangles.push([lo, hi])
//   }
//   return rectangles
// }


fn decompose_region<T: Sub+PartialEq+Ord+Copy>(paths: &[Vec<[T;2]>], clockwise: bool) {
  //First step: unpack all vertices into internal format
  let mut vertices = vec![];
  let mut  ptr = 0;
  let npaths = Vec::with_capacity(paths.len());
  for i in 0..paths.len() {
    let path = paths[i];
    let n = path.len();
    let mut prev = &path[n-3];
    let mut cur = &path[n-2];
    let mut next = &path[n-1];
    npaths[i] = vec![];
    for j in 0..n {
      prev = cur;
      cur = next;
      next = &path[j];
      let mut concave = false;
      if prev[0] == cur[0] {
        if next[0] == cur[0] {
          continue
        }
        let dir0 = prev[1] < cur[1];
        let dir1 = cur[0] < next[0];
        concave = dir0 == dir1;
      } else {
        if next[1] == cur[1] {
          continue
        }
        let dir0 = prev[0] < cur[0];
        let dir1 = cur[1] < next[1];
        concave = dir0 != dir1;
      }
      if clockwise {
        concave = !concave;
      }
      let vtx = Rc::new(RefCell::new(Vertex::new(
        *cur,
        i,
        (j + n - 1)%n,
        concave
      )));
      npaths[i].push(vtx);
      vertices.push(vtx);
    }
  }

  //Next build interval trees for segments, link vertices into a list
  let mut hsegments = vec![];
  let mut vsegments = vec![];
  for i in 0..npaths.len() {
    let p = npaths[i];
    for j in 0..p.len() {
      let mut a = p[j];
      let mut b = p[(j+1)%p.len()];
      if a.borrow().point[0] == b.borrow().point[0] {
        hsegments.push(Segment::new(a,b,0))
      } else {
        vsegments.push(Segment::new(a,b,1))
      }
      if clockwise {
        a.borrow().prev = Some(b);
        b.borrow().next = Some(a);
      } else {
        a.borrow().next = Some(b);
        b.borrow().prev = Some(a);
      }
    }
  }
  let htree: IntervalTree<_,_> = hsegments.into_iter().map(|i|i.into()).collect();
  let vtree: IntervalTree<_,_> = vsegments.into_iter().map(|i|i.into()).collect();

  //Find horizontal and vertical diagonals
  let hdiagonals = get_diagonals(&vertices, &npaths, 0, vtree);
  let vdiagonals = get_diagonals(&vertices, &npaths, 1, htree);

  //Find all splitting edges
  let splitters = find_splitters(&hdiagonals, &vdiagonals);

  //Cut all the splitting diagonals
  for i in 0..splitters.len() {
    split_segment(&mut splitters[i]);
  }

  //Split all concave vertices
  splitConcave(vertices)

  //Return regions
  return findRegions(vertices)
}