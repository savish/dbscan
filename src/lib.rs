extern crate clusters;
pub use clusters::Algorithm;
pub use clusters::Clustered;
pub use clusters::Proximity;
use std::collections::HashMap;

/// Holds results from the DBSCAN clustering algorithm
pub struct Results<T>(HashMap<T, Option<i32>>);

impl<T> Results<T> {
    // Returns the data container in the struct. This type is pruposefully
    // opaque.
    fn inner(&self) -> &HashMap<T, Option<i32>> {
        let Results(inner) = self;
        inner
    }
}

impl<T> Clustered<T> for Results<T>
where
    T: Eq + Copy + std::hash::Hash,
{
    fn clusters(&self) -> Vec<Vec<T>> {
        let mut cluster_map = HashMap::new();
        let mut clusters = Vec::new();

        for (clusterable, cluster) in self.inner() {
            if *cluster != Some(-1) {
                let current_cluster = cluster_map.entry(*cluster).or_insert(Vec::new());
                current_cluster.push(*clusterable);
            }
        }

        for (_, clusterables) in cluster_map.drain() {
            clusters.push(clusterables)
        }

        clusters
    }

    fn noise(&self) -> Vec<T> {
        let mut noise = Vec::new();

        for (clusterable, cluster) in self.inner() {
            if *cluster == Some(-1) {
                noise.push(*clusterable)
            }
        }

        noise
    }
}

/// Represents the DBSCAN algorithm
pub struct DBSCAN<T>
where
    T: Proximity + Eq + Copy + std::hash::Hash,
{
    epsilon: <T as Proximity>::Output,
    min_pts: usize,
}

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
    pub fn new(epsilon: <T as Proximity>::Output, min_pts: usize) -> DBSCAN<T> {
        DBSCAN { epsilon, min_pts }
    }
}

impl<T> Algorithm<T> for DBSCAN<T>
where
    T: Proximity + Eq + Copy + std::hash::Hash + 'static,
{
    fn cluster(&self, clusterables: &[T]) -> Box<Clustered<T>> {
        Box::new(Results(cluster(clusterables, self.epsilon, self.min_pts)))
    }
}

// Determine neighbours for a given datapoint
fn neighbours<T>(clusterable: T, clusterables: &[T], epsilon: <T as Proximity>::Output) -> Vec<T>
where
    T: Proximity + Eq + Copy,
{
    clusterables
        .iter()
        .filter(|q| clusterable != **q && clusterable.is_near(*q, epsilon))
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
