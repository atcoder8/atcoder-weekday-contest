use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, s): (usize, usize, Usize1),
        uvw: [(Usize1, Usize1, u64); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v, weight) in &uvw {
        graph[u].push((v, weight));
        graph[v].push((u, weight));
    }

    let mut distances = vec![None::<u64>; n];
    let mut heap = BinaryHeap::from_iter([(Reverse(0), s)]);
    while let Some((Reverse(cost), curr)) = heap.pop() {
        if distances[curr].is_some() {
            continue;
        }

        distances[curr] = Some(cost);

        heap.extend(
            graph[curr]
                .iter()
                .map(|&(adjacent, weight)| (Reverse(cost + weight), adjacent)),
        );
    }

    let sum_cost = distances.iter().flat_map(|&dist| dist).sum::<u64>();
    println!("{sum_cost}");
}
