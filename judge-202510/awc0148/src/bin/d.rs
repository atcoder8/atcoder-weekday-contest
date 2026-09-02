use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, p): (usize, u64),
        hg: [(u64, u64); n],
    }

    let possible = hg.iter().all(|&(h, _)| p >= h)
        && hg
            .iter()
            .map(|&(h, g)| (p - h) / g)
            .sorted_unstable()
            .enumerate()
            .all(|(i, margin)| i as u64 <= margin);
    println!("{}", if possible { "Yes" } else { "No" });
}
