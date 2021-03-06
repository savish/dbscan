# DBSCAN

[![Build Status](https://travis-ci.com/savish/dbscan.svg?branch=master)](https://travis-ci.com/savish/dbscan)

> Density-Based Spatial Clustering of Applications with Noise

[Wikipedia link](1)

DBSCAN is a density-based clustering algorithm: given a set of points in some space, it groups together points that are closely packed together (points with many nearby neighbors), marking as outliers points that lie alone in low-density regions (whose nearest neighbors are too far away).

This is an implementation of the algorithm in rust-stable.

## Usage

This project is written entirely in rust. It is recommended that you use the latest stable version with it. The _oldest_ supported version is `1.26.1`

To use, Add the project to your `Cargo.toml` file, under dependencies. At the moment, there are no optional features, so this will suffice:

**Cargo.toml**

```toml
[dependencies]
dbscan = "0.1"
```

Import the library into your project using:

```rust
extern crate dbscan;
use dbscan::{DBSCAN, Proximity};
```

This will add the traits `Proximity` and the struct `DBSCAN` to the current module/scope.

## Examples

Implementation examples are provided in the `examples/` directory. One simple implementation is presented below.

**2D Point clustering**

This implementation uses the `dbscan` crate to add distance-based clustering capabilities to a field of 2D points. The full example can be viewed in the examples directory mentioned above. A few implementation details are presented below.

1.  Define a 'clusterable' type

```rust
/// Represents a 2 Dimensional point
#[derive(Clone, Copy, Debug)]
struct Point {
  id: u32,
  x: f64,
  y: f64,
}
```

2.  Implement required traits. The algorthm requires that the `Proximity`, `Hash`, `Eq` and `Copy` traits be implemented for all potential clusterable types. Two such implementations are listed below.

```rust
impl Proximity for Point {
  type Output = f64;

  fn distance(&self, other: Point) -> f64 {
    ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
  }
}

// Need to implement our own hash function since you cannot derive the `Hash`
// trait for the `f64` primitive type
impl Hash for Point {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.id.hash(state);
  }
}
```

3.  Define your clusterables

```rust
fn main() {
  // Use a tuple vector to define some points
  let point_tuples = vec![
    (0f64, 0f64),
    (1., 0.),
    (0., -1.),
    (1., 2.),
    (3., 5.),
  ];

  // Create a vector of point structs
  let points = point_tuples
    .into_iter()
    .enumerate()
    .map(|(id, pt)| Point {
      id: id as u32,
      x: pt.0,
      y: pt.1,
    })
    .collect::<Vec<_>>();

  ...
```

4.  Create a new instance of the algorithm from your clusterables

```rust
fn main() {
  ...
  let alg = DBSCAN::new(&points, 2f64, 1);
  ...
}
```

5.  Use the `.clusters()` function to get your clustered results

```rust
fn main() {
  ...
  // Print out clusters
  for (cluster, points) in alg.clusters() {
    match cluster {
      Some(cluster_name) => println!("Cluster {:?}: {:?}", cluster_name, points),
      None => println!("Noise: {:?}", points),
    }
  }
  ...
}
```

## Tests

**TODO**

## Versioning

This project uses SemVer for versioning. For the versions available, see the tags on this repository.

## Authors

_Primary:_ Alan K <mailto:afksavish@gmail.com> @savish

## License

This project is licensed under the MIT License - see the LICENSE.md file for details

## Contributing

Please read `CONTRIBUTING.md` for the process of submitting pull requests.

[1]: https://en.wikipedia.org/wiki/DBSCAN
