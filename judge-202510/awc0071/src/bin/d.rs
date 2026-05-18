use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    let answer = match solve() {
        Some(value) => format!("{}", value),
        None => "-1".to_string(),
    };
    println!("{}", answer);
}

fn solve() -> Option<u64> {
    input! {
        (n, m, s, g, t): (usize, usize, Usize1, Usize1, Usize1),
        uvc: [(Usize1, Usize1, u64); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, weight) in &uvc {
        graph[u].push((v, weight));
        graph[v].push((u, weight));
    }

    let mut distances = vec![None::<u64>; n];
    let mut heap = BinaryHeap::from([(Reverse(0), g)]);
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

    if let (Some(dist1), Some(dist2)) = (distances[s], distances[t]) {
        Some(dist1 + dist2)
    } else {
        None
    }
}
