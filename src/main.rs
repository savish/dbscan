extern crate dbscan;
use dbscan::{HasDistance, DBSCAN};

#[derive(Clone, Copy, Debug, Hash)]
struct Point {
    x: i32,
    y: i32,
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
        .map(|pt| Point { x: pt.0, y: pt.1 })
        .collect::<Vec<_>>();

    // for line in cluster(&mut labpts, 2, 2).iter() {
    //     println!("{:?}", line);
    // }

    let dbscan = DBSCAN::new(&mut labpts, 2, 2);
    // println!("{:?}", dbscan.inner());
    for cluster in dbscan.clusters() {
        println!("{:?}", cluster);
    }

    println!("{:?}", dbscan.noise());
}
