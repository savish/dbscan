//! # DBSCAN
//! 
//! > Density-Based Spatial Clustering of Applications with Noise
//! 
//! [Wikipedia link](1)
//! 
//! DBSCAN is a density-based clustering algorithm: given a set of points in some space, it groups together points that are closely packed together (points with many nearby neighbors), marking as outliers points that lie alone in low-density regions (whose nearest neighbors are too far away).
//! 
//! This is an implementation of the algorithm in rust-stable.
//! 
//! ## Usage
//! 
//! This project is written entirely in rust. It is recommended that you use the latest stable version with it. The _oldest_ supported version is `1.26.1`
//! 
//! To use, Add the project to your `Cargo.toml` file, under dependencies. At the moment, there are no optional features, so this will suffice:
//! 
//! **Cargo.toml**
//! 
//! ```toml
//! [dependencies]
//! dbscan = "0.1"
//! ```
//! 
//! Import the library into your project using:
//! 
//! ```rust
//! extern crate dbscan;
//! use dbscan::{DBSCAN, HasDistance};
//! ```
//! 
//! This will add the traits `HasDistance` and the struct `DBSCAN` to the current module/scope.
//! 
//! ## Examples
//! 
//! Implementation examples are provided in the `examples/` directory. One simple implementation is presented below.
//! 
//! **2D Point clustering**
//! 
//! This implementation uses the `dbscan` crate to add distance-based clustering capabilities to a field of 2D points. The full example can be viewed in the examples directory mentioned above. A few implementation details are presented below.
//! 
//! 1.  Define a 'clusterable' type
//! 
//! ```rust
//! /// Represents a 2 Dimensional point
//! #[derive(Clone, Copy, Debug)]
//! struct Point {
//!   id: u32,
//!   x: f64,
//!   y: f64,
//! }
//! ```
//! 
//! 2.  Implement required traits. The algorthm requires that the `HasDistance`, `Hash`, `Eq` and `Copy` traits be implemented for all potential clusterable types. Two such implementations are listed below.
//! 
//! ```rust
//! impl HasDistance for Point {
//!   type Output = f64;
//! 
//!   fn distance(&self, other: Point) -> f64 {
//!     ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
//!   }
//! }
//! 
//! // Need to implement our own hash function since you cannot derive the `Hash`
//! // trait for the `f64` primitive type
//! impl Hash for Point {
//!   fn hash<H: Hasher>(&self, state: &mut H) {
//!     self.id.hash(state);
//!   }
//! }
//! ```
//! 
//! 3.  Define your clusterables
//! 
//! ```rust
//! fn main() {
//!   // Use a tuple vector to define some points
//!   let point_tuples = vec![
//!     (0f64, 0f64),
//!     (1., 0.),
//!     (0., -1.),
//!     (1., 2.),
//!     (3., 5.),
//!   ];
//! 
//!   // Create a vector of point structs
//!   let points = point_tuples
//!     .into_iter()
//!     .enumerate()
//!     .map(|(id, pt)| Point {
//!       id: id as u32,
//!       x: pt.0,
//!       y: pt.1,
//!     })
//!     .collect::<Vec<_>>();
//! 
//!   ...
//! ```
//! 
//! 4.  Create a new instance of the algorithm from your clusterables
//! 
//! ```rust
//! fn main() {
//!   ...
//!   let alg = DBSCAN::new(&points, 2f64, 1);
//!   ...
//! }
//! ```
//! 
//! 5.  Use the `.clusters()` function to get your clustered results
//! 
//! ```rust
//! fn main() {
//!   ...
//!   // Print out clusters
//!   for (cluster, points) in alg.clusters() {
//!     match cluster {
//!       Some(cluster_name) => println!("Cluster {:?}: {:?}", cluster_name, points),
//!       None => println!("Noise: {:?}", points),
//!     }
//!   }
//!   ...
//! }
//! ```
//! 
//! ## Tests
//! 
//! **TODO**
//! 
//! ## Versioning
//! 
//! This project uses SemVer for versioning. For the versions available, see the tags on this repository.
//! 
//! ## Authors
//! 
//! _Primary:_ Alan K <mailto:afksavish@gmail.com> @savish
//! 
//! ## License
//! 
//! This project is licensed under the MIT License - see the LICENSE.md file for details
//! 
//! ## Contributing
//! 
//! Please read [CONTRIBUTING.md](2) for the process of submitting pull requests.
//! 
//! [1]: https://en.wikipedia.org/wiki/DBSCAN
//! [2]: ./CONTRIBUTING.md
use std::collections::HashMap;

/// Functionality to determine proximity between datapoints
pub trait Proximity<Other = Self> {
    type Output: PartialOrd + Copy;

    /// Returns the distance between two trait implementors
    ///
    /// The DBSCAN algorithm uses the distance between its datapoints to
    /// determine whether or not they are within a 'neighbourhood'. The distance
    /// function itself, however, is highly dependent on the implementing types.
    fn distance(&self, other: Other) -> Self::Output;

    /// Returns `true` if the two data points are 'close' to each other
    ///
    /// The concept of 'closeness' is determined by the `epsilon` parameter.
    /// Any datapoints with distances that are less than this parameter are
    /// considered 'close' as far as the algorithm is concerned
    fn is_near(&self, other: Other, epsilon: Self::Output) -> bool {
        self.distance(other) <= epsilon
    }
}

/// Represents the DBSCAN algorithm
pub struct DBSCAN<T>(HashMap<T, Option<i32>>);

impl<T> DBSCAN<T>
where
    T: Proximity + Eq + Copy + std::hash::Hash,
{
    /// Initialize the algorithm
    ///
    /// This is where the clustering happens. After initialization, the
    /// `cluster` function can be used to retrieve the clusters.
    ///
    /// - `clusterables` This is a list of the data points fed into the
    /// algorithm for clustering
    /// - `epsilon` This parameter is used to determine the proximity of
    /// datapoints
    /// - `min_pts` The algorithm has a concept of 'core points' which are
    /// data points with more than `min_pts` neighbours.
    pub fn new(clusterables: &[T], epsilon: <T as Proximity>::Output, min_pts: usize) -> DBSCAN<T> {
        DBSCAN(cluster(clusterables, epsilon, min_pts))
    }

    /// Retrieve clusters from the source data
    ///
    /// The clusters are returned as a map. The keys of the map are `Option`s.
    /// For instance, the key `Some(3)` indicates that it's value (an array of
    /// datapoints) are all in cluster number 3. If the key is `None` then its
    /// value represents 'noise' datapoints
    pub fn clusters(&self) -> HashMap<Option<i32>, Vec<T>> {
        let mut clusters = HashMap::new();

        for (clusterable, cluster) in self.inner() {
            if *cluster != Some(-1) {
                let current_cluster = clusters.entry(*cluster).or_insert(Vec::new());
                current_cluster.push(*clusterable);
            } else {
                let current_cluster = clusters.entry(None).or_insert(Vec::new());
                current_cluster.push(*clusterable);
            }
        }

        clusters
    }

    fn inner(&self) -> &HashMap<T, Option<i32>> {
        let DBSCAN(inner) = self;
        inner
    }
}

// Determine neighbours for a given datapoint
fn neighbours<T>(clusterable: T, clusterables: &[T], epsilon: <T as Proximity>::Output) -> Vec<T>
where
    T: Proximity + Eq + Copy,
{
    clusterables
        .iter()
        .filter(|q| clusterable != **q && clusterable.is_near(**q, epsilon))
        .map(|q| *q)
        .collect::<Vec<_>>()
}

// Cluster data points using the DBSCAN algorithm
//
// The result type is a map with each data point as a key, and the value is an
// option indicating which cluster the datapoint is in (or `Some(-1)` for
// datapoints that are considered 'noise')
fn cluster<T>(
    clusterables: &[T],
    epsilon: <T as Proximity>::Output,
    min_pts: usize,
) -> HashMap<T, Option<i32>>
where
    T: Proximity + Eq + Copy + std::hash::Hash,
{
    let mut clusters = clusterables
        .iter()
        .fold(HashMap::new(), |mut acc, clusterable| {
            acc.insert(*clusterable, None);
            acc
        });

    let mut cluster_count = 0i32;

    for clusterable in clusterables.iter() {
        if clusters[clusterable].is_some() {
            continue;
        }

        let mut nbrs = neighbours(*clusterable, clusterables, epsilon);

        if nbrs.len() <= min_pts {
            clusters.insert(*clusterable, Some(-1));
            continue;
        }

        clusters.insert(*clusterable, Some(cluster_count));

        for c_ix in 0..nbrs.len() {
            let neighbour = nbrs[c_ix];
            if clusters[&neighbour] == Some(-1) {
                clusters.insert(neighbour, Some(cluster_count));
            }

            if clusters[&neighbour].is_some() {
                continue;
            }

            clusters.insert(neighbour, Some(cluster_count));

            let new_nbrs = neighbours(neighbour, clusterables, epsilon);

            if new_nbrs.len() > min_pts {
                nbrs.extend(new_nbrs);
            }
        }

        cluster_count = cluster_count + 1;
    }

    clusters
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
