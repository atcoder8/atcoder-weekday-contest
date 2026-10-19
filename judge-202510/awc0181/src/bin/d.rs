use itertools::Itertools;
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        (n, m): (usize, usize),
        lr: [(Usize1, usize); m],
    }

    let mut imos = vec![0; n + 1];
    for &(l, r) in &lr {
        imos[l] += 1;
        imos[r] -= 1;
    }
    for i in 0..n {
        imos[i + 1] += imos[i];
    }
    imos.pop();

    let num_zeros = imos.iter().filter(|&&v| v == 0).count();
    let one_indices = imos.iter().positions(|&v| v == 1).collect_vec();

    let mut max_score = 0;
    for &(l, r) in &lr {
        let lower = one_indices.lower_bound(&l);
        let upper = lower + one_indices[lower..].lower_bound(&r);
        max_score = max_score.max(n - (num_zeros + upper - lower));
    }

    println!("{max_score}");
}
