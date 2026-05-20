use itertools::iproduct;
use ndarray::Array2;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, k): (usize, usize, usize),
        uvw: [(Usize1, Usize1, u64); k],
        st: [(Usize1, Usize1); n],
    }

    let mut dist_array = Array2::<Option<u64>>::default([m; 2]);
    for i in 0..m {
        dist_array[[i; 2]] = Some(0);
    }
    for &(u, v, w) in &uvw {
        dist_array[(u, v)] = Some(w);
        dist_array[(v, u)] = Some(w);
    }

    for (mid, from, to) in iproduct!(0..m, 0..m, 0..m) {
        if let (Some(dist1), Some(dist2)) = (dist_array[(from, mid)], dist_array[(mid, to)]) {
            chmin_for_option(&mut dist_array[(from, to)], dist1 + dist2);
        }
    }

    let sum_dist = st
        .iter()
        .map(|&(s, t)| dist_array[(s, t)].unwrap())
        .sum::<u64>();
    println!("{sum_dist}");
}

/// If `value` is `None` or contains a value greater than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmin_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost <= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}
