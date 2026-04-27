use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::{Itertools, enumerate};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uvw: [(Usize1, Usize1, u64); m],
        (s, k): (Usize1, usize),
        dd: [Usize1; k],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, weight) in &uvw {
        graph[u].push((v, weight));
        graph[v].push((u, weight));
    }

    let find_min_distances = |start: usize| {
        let mut distances = vec![None::<u64>; n];
        let mut heap = BinaryHeap::from_iter([(Reverse(0), start)]);
        while let Some((Reverse(cost), curr)) = heap.pop() {
            if distances[curr].is_some() {
                continue;
            }

            distances[curr] = Some(cost);

            heap.extend(
                graph[curr]
                    .iter()
                    .map(|&(next, weight)| (Reverse(cost + weight), next)),
            );
        }

        dd.iter().map(|&d| distances[d].unwrap()).collect_vec()
    };

    let from_start = find_min_distances(s);
    let from_destinations = dd.iter().cloned().map(find_min_distances).collect_vec();

    let mut dp = vec![vec![None::<u64>; k]; 1 << k];
    for (d, &cost) in enumerate(&from_start) {
        dp[1 << d][d] = Some(cost);
    }
    for bits in 1..1 << k {
        for from in 0..k {
            if bits >> from & 1 == 0 {
                continue;
            }

            let from_cost = dp[bits][from].unwrap();
            for to in 0..k {
                let to_cost = from_cost + from_destinations[from][to];
                chmin_for_option(&mut dp[bits | 1 << to][to], to_cost);
            }
        }
    }

    let min_sum_cost = enumerate(&dp[(1 << k) - 1])
        .map(|(i, cost)| cost.unwrap() + from_start[i])
        .min()
        .unwrap();
    println!("{min_sum_cost}");
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
