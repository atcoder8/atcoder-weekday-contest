use std::cmp::Reverse;

use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        bb: [u32; n],
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
    }
    graph
        .iter_mut()
        .for_each(|edges| edges.sort_unstable_by_key(|&adjacent| Reverse(bb[adjacent])));

    let mut order = vec![];
    let mut visited = vec![false; n];
    let mut init_cities = (0..n).collect_vec();
    init_cities[1..].sort_unstable_by_key(|&city| Reverse(bb[city]));
    for init_city in init_cities {
        if visited[init_city] {
            continue;
        }

        let mut curr = Some(init_city);
        while let Some(v) = curr {
            order.push(v);
            visited[v] = true;
            curr = graph[v]
                .iter()
                .copied()
                .find(|&adjacent| !visited[adjacent]);
        }
    }

    println!("{}", order.iter().map(|v| v + 1).join(" "));
}
