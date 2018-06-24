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

/// Labelled types
///
/// Implemented for any types that *may* have a label
pub trait HasLabel {
    fn set_label(&mut self, label: i32) -> ();
    fn clear_label(&mut self) -> ();
    fn label(&self) -> Option<i32>;
}

fn neighbours<T>(point: T, points: &Vec<T>, epsilon: <T as HasDistance>::Output) -> Vec<T>
where
    T: HasDistance + HasLabel + Eq + Copy,
{
    points
        .iter()
        .filter(|q| point != **q && point.is_near(**q, epsilon))
        .map(|q| *q)
        .collect::<Vec<_>>()
}

pub fn cluster<T>(points: &mut Vec<T>, epsilon: <T as HasDistance>::Output, min_pts: usize) -> ()
where
    T: HasDistance + HasLabel + Eq + Copy,
{
    let mut cluster_count = 0;

    for p_ix in 0..points.len() {
        let mut p = points[p_ix];
        if p.label().is_some() {
            continue;
        }

        let mut nbrs = neighbours(p, points, epsilon);

        if nbrs.len() <= min_pts {
            p.set_label(-1);
            continue;
        }

        p.set_label(cluster_count);
        points[p_ix] = p;

        for q_ix in 0..nbrs.len() {
            let mut q = nbrs[q_ix];
            if q.label() == Some(-1) {
                q.set_label(cluster_count)
            }

            if q.label().is_some() {
                continue;
            }

            q.set_label(cluster_count);
            nbrs[q_ix] = q;
            let q_nbrs = neighbours(q, points, epsilon);
            if q_nbrs.len() > min_pts {
                nbrs.extend(q_nbrs)
            }
        }

        cluster_count = cluster_count + 1;
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
