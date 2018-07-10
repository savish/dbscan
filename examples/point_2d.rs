//! This is an example of implementing DBSCAN for a 2D cartesian system
//!
//! The struct `Point` is our main 'clusterable' type. The algorithm requires
//! that a number of traits be implemented by this type before it can be used.
//!
//! In the main function we define a number of 2D points and use the algorithm
//! to cluster them accordingly and print out the clusters

extern crate dbscan;
use dbscan::{Algorithm, Proximity, DBSCAN};
use std::fmt;
use std::hash::{Hash, Hasher};

/// Represents a 2 Dimensional point
#[derive(Clone, Copy, Debug)]
struct Point {
  id: u32,
  x: f64,
  y: f64,
}

// Must be implemented for the struct to be used in the algorithm
impl Proximity for Point {
  type Output = f64;

  fn distance(&self, other: &Point) -> f64 {
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

impl PartialEq for Point {
  fn eq(&self, other: &Point) -> bool {
    self.id == other.id
  }
}

impl Eq for Point {}

// Not required by the algorithm
impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

fn main() {
  // Use a tuple vector to define some points
  let point_tuples = vec![
    (0f64, 0f64), // Cluster
    (1., 0.),     //
    (0., -1.),    // -------
    (1., 2.),     // Noise
    (3., 5.),     // Cluster
    (4., 5.),     //
    (5., 5.),     // -------
    (3., -2.),    // Cluster
    (3., 0.),     // -------
    (-1., 4.),    // Noise
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

  // Create a new instance of the algorithm
  //
  // This instance will consider all points within a radius of 2 units as
  // 'neighours' and any point with more than 1 neighbour forms a cluster.
  let alg = DBSCAN::new(2f64, 1);

  // Print out clusters
  //
  // Clusters are returned as a `HashMap` whose keys are `Option<u32>` values
  // and whose values are a list of `Point` structs in this implementation.
  // The keys that have some value (`Some(value)`) represent clusters. Noise
  // elements have a key of `None`
  let cluster_results = alg.cluster(&points);
  for cluster in cluster_results.clusters() {
    print!("\nCluster: [");
    for cluster_point in cluster {
      print!(" {}", cluster_point)
    }
    print!(" ]\n");
  }

  print!("\nNoise: [");
  for noise_point in cluster_results.noise() {
    print!(" {}", noise_point);
  }
  print!(" ]\n");
}
