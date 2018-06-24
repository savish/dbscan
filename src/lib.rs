use std::collections::HashMap;

/// A distance function for types
///
/// This is implemented for any types that have the concept of a distance
/// between them. For instance, the distance between two numbers can be defined
/// as the difference between them.
pub trait HasDistance<Other = Self> {
    type Output: PartialOrd + Copy;

    fn distance(&self, other: Other) -> Self::Output;
    fn is_near(&self, other: Other, epsilon: Self::Output) -> bool {
        self.distance(other) <= epsilon
    }
}

pub struct DBSCAN<T>(HashMap<T, Option<i32>>);

impl<T> DBSCAN<T>
where
    T: HasDistance + Eq + Copy + std::hash::Hash,
{
    pub fn new(
        clusterables: &mut Vec<T>,
        epsilon: <T as HasDistance>::Output,
        min_pts: usize,
    ) -> DBSCAN<T> {
        DBSCAN(cluster(clusterables, epsilon, min_pts))
    }

    // clusters
    pub fn clusters(&self) -> HashMap<&Option<i32>, Vec<&T>> {
        let mut clusters = HashMap::new();

        for (clusterable, cluster) in self.inner() {
            if *cluster != Some(-1) {
                let current_cluster = clusters.entry(cluster).or_insert(Vec::new());
                current_cluster.push(clusterable);
            }
        }

        clusters
    }

    // noise
    pub fn noise(&self) -> Vec<&T> {
        let mut noise = Vec::new();

        for (clusterable, cluster) in self.inner() {
            if *cluster == Some(-1) {
                noise.push(clusterable);
            }
        }

        noise
    }

    pub fn inner(&self) -> &HashMap<T, Option<i32>> {
        let DBSCAN(inner) = self;
        inner
    }
}

fn neighbours<T>(
    clusterable: T,
    clusterables: &Vec<T>,
    epsilon: <T as HasDistance>::Output,
) -> Vec<T>
where
    T: HasDistance + Eq + Copy,
{
    clusterables
        .iter()
        .filter(|q| clusterable != **q && clusterable.is_near(**q, epsilon))
        .map(|q| *q)
        .collect::<Vec<_>>()
}

pub fn cluster<T>(
    clusterables: &mut Vec<T>,
    epsilon: <T as HasDistance>::Output,
    min_pts: usize,
) -> HashMap<T, Option<i32>>
where
    T: HasDistance + Eq + Copy + std::hash::Hash,
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
