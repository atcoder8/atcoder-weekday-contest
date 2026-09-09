use itertools::{Itertools, izip};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [u64; n],
        bb: [Usize1; k],
        cc: [u64; k],
    }

    let mut values = aa.clone();
    for (b, c) in izip!(bb, cc) {
        values[b] = c;
    }

    let sum = values
        .into_iter()
        .tuple_windows()
        .map(|(v1, v2)| v1.abs_diff(v2))
        .sum::<u64>();
    println!("{sum}");
}
