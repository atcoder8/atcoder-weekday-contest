use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (m, s): (usize, usize),
        bb: [usize; m],
        n: usize,
        lr: [(Usize1, usize); n],
    }

    let mut prefix_sum = vec![0; m + 1];
    for i in 0..m {
        prefix_sum[i + 1] = prefix_sum[i] + bb[i] + s / m + (i < s % m) as usize;
    }

    let output = lr
        .iter()
        .map(|&(l, r)| prefix_sum[r] - prefix_sum[l])
        .join("\n");
    println!("{output}");
}
