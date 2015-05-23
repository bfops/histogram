#![feature(core)]

use std::iter::{range_inclusive, repeat};
use std::num;

#[derive(Copy, Debug)]
struct Point { x: i32, y: i32 }

/// `points` should be sorted from left to right.
fn compute_best_area(k: u32, points: &[Point]) -> u32 {
  // Subproblems, indexed by k and then by n.
  let mut subproblems: Vec<Vec<u32>> = Vec::with_capacity(k as usize + 1);

  let n: u32 = num::cast(points.len()).unwrap();

  // Just to make indexing nice.
  subproblems.push(Vec::new());

  // Base case: k = 1.
  {
    let mut subsubproblems = Vec::with_capacity(n as usize + 1);
    subsubproblems.push(0);
    for n in range_inclusive(1, n) {
      subsubproblems.push(area_of(points[0 .. n as usize].iter().map(|&p| p)));
    }
    subproblems.push(subsubproblems);
  }

  for k in range_inclusive(2, k) {
    println!("k = {}", k);
    let mut subsubproblems = Vec::with_capacity(n as usize + 1);
    // Area is vacuously 0 for n < k.
    subsubproblems.extend(repeat(0).take(k as usize + 1));

    // For n > k.
    for n in range_inclusive(k + 1, n) {
      println!("n = {}", n);
      // For every i<nth point, try building k-1 rectangles to that point,
      // and one more from i+1 to n. Whichever of those combinations has the
      // smallest total area, we keep.
      let best_area =
        range(0, n)
        // TODO: Don't keep recomputing area_of.
        .map(|i| {
          let prev_area = subproblems[k as usize - 1][i as usize];
          println!("[{}][{}] = {}", k-1, n-1, prev_area);
          let remaining_area = area_of(points[i as usize..n as usize].iter().map(|&p| p));
          println!("{} remains", remaining_area);
          prev_area + remaining_area
        })
        .min()
        .unwrap();

      subsubproblems.push(best_area);
    }

    subproblems.push(subsubproblems);
  }

  subproblems[k as usize][n as usize]
}

// Compute the area of a single rectangle over the provided points.
fn area_of<I: Iterator<Item=Point>>(mut points: I) -> u32 {
  // Top-right corner of the rectangle so far.
  let mut top_right;
  // Dimensions of the rectangle so far.
  let mut width;

  {
    let p = points.next().unwrap();
    top_right = p;
    width = 0;
  }

  let mut area = 0;
  for p in points {
    // n == i + 1

    let dx = p.x - top_right.x;
    let dy = p.y - top_right.y;

    assert!(dx > 0);

    area += dx*p.y;
    if dy > 0 {
      area += width*dy;
      top_right = p;
    } else {
      top_right.x = p.x;
    }

    width += dx;
  }

  num::cast(area).unwrap()
}

#[test]
fn k_equals_n() {
  let k = 4;
  let points = [
    Point { x: 1, y: 1 },
    Point { x: 2, y: 1 },
    Point { x: 5, y: 8 },
    Point { x: 10, y: 16 },
  ];

  assert_eq!(compute_best_area(k, &points), 0);
}

#[test]
fn simple() {
  let k = 1;
  let points = [
    Point { x: 1, y: 1 },
    Point { x: 2, y: 1 },
  ];

  assert_eq!(compute_best_area(k, &points), 1);
}

#[test]
fn obvious() {
  let k = 3;
  let points = [
    Point { x: 1, y: 1 },
    Point { x: 2, y: 1 },
    Point { x: 5, y: 8 },
    Point { x: 10, y: 16 },
  ];

  assert_eq!(compute_best_area(k, &points), 1);
}

#[test]
fn less_obvious() {
  let k = 4;
  let points = [
    Point { x: 1, y: 1 },
    Point { x: 2, y: 1 },
    Point { x: 5, y: 8 },
    Point { x: 10, y: 16 },
    Point { x: 15, y: 4 },
    Point { x: 17, y: 4 },
  ];

  assert_eq!(compute_best_area(k, &points), 9);
}

#[cfg(not(test))]
fn main() {
  let k = 3;
  let points = [
    Point { x: 1, y: 1 },
    Point { x: 2, y: 1 },
    Point { x: 5, y: 8 },
    Point { x: 10, y: 16 },
  ];

  let area = compute_best_area(k, &points);

  println!("Best area for {} rectangles over points {:?} is {}.", k, points, area);
}
