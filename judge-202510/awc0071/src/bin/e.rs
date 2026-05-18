// unfinished

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        uv: [(Usize1, Usize1); m],
    }

    let mut graph = vec![vec![]; n];
    for &(u, v) in &uv {
        graph[u].push(v);
        graph[v].push(u);
    }

    let mut cost = 0;
    let mut visited = vec![false; n];
    for start in 0..n {
        if visited[start] {
            continue;
        }

        let mut counts = [0_usize; 2];
        let mut stack = vec![(start, 0)];
        while let Some((curr, parity)) = stack.pop() {
            if visited[curr] {
                continue;
            }

            visited[curr] = true;

            counts[parity] += 1;

            stack.extend(graph[curr].iter().map(|&next| (next, 1 - parity)));
        }

        cost += counts[0].min(counts[1]);
    }

    println!("{cost}");
}
