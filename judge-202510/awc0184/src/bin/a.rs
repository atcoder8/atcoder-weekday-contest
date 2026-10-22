use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, c): (usize, u64),
        wa: [(u64, u64); n],
    }

    let output = wa
        .iter()
        .map(|&(w, a)| (w * a).saturating_sub(c))
        .join("\n");
    println!("{output}");
}
