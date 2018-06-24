extern crate dbscan;
use dbscan::{cluster, HasDistance, HasLabel};

// fn neighbours(point: (i32, i32), from_db: Vec<(i32, i32)>, eps: u32) -> Vec<(i32, i32)> {
//     from_db
//         .into_iter()
//         .filter(|q| !is_equal(point, *q) && is_neighbour(point, *q, eps))
//         .collect::<Vec<(i32, i32)>>()
// }

#[derive(Clone, Copy, Debug)]
struct Point {
    x: i32,
    y: i32,
    label: Option<i32>,
}

impl HasLabel for Point {
    fn set_label(&mut self, label: i32) {
        self.label = Some(label);
    }

    fn clear_label(&mut self) {
        self.label = None
    }

    fn label(&self) -> Option<i32> {
        self.label
    }
}

impl HasDistance for Point {
    type Output = u32;

    fn distance(&self, other: Point) -> u32 {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Point {}

fn main() {
    let points: Vec<(i32, i32)> = vec![
        (2, 14),
        (3, 12),
        (4, 10),
        (4, 8),
        (4, 6),
        (5, 5),
        (4, 5),
        (4, 4),
        (5, 4),
        (6, 4),
        (6, 2),
        (8, 1),
        (9, 1),
        (10, 2),
        (11, 4),
        (11, 6),
        (11, 8),
        (10, 10),
        (10, 11),
        (9, 11),
        (9, 10),
        (9, 9),
        (8, 9),
        (7, 7),
        (6, 6),
        (5, 3),
        (3, 2),
        (1, 4),
        (1, 6),
        (1, 8),
        (1, 10),
        (1, 12),
    ];

    let mut labpts = points
        .into_iter()
        .map(|pt| Point {
            x: pt.0,
            y: pt.1,
            label: None,
        })
        .collect::<Vec<_>>();

    cluster(&mut labpts, 1, 1);

    // for lp in stuff.iter() {
    //     println!("{:?}", lp);
    // }

    for lp in labpts.iter() {
        println!("{:?}", lp);
    }

    // let mut looper = vec![];

    // let neighbours = neighbours(points[7], points, 2);

    // for val in 0..looper.len() {
    //   if looper.len() < 15 {
    //     looper.extend(vec![3, 4]);
    //   }
    //   println!("{:?}", looper);
    // }
    // let mut ix = 0usize;
    // while ix <= looper.len() {
    //   println!("{:?}", ix);
    //   println!("{:?}", looper);
    //   ix = ix + 1;
    //   if ix < 25 {
    //     looper.push(ix);
    //   }
    // }
}