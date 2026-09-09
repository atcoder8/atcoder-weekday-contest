use itertools::{Itertools, chain};
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        xx: [u32; n],
        pp: [u32; m],
    }

    let mut distances = xx.into_iter().map(|x| {
        let pivot = pp.partition_point(|&y| y < x);
        let p1 = pivot.checked_sub(1).map(|i| pp[i]);
        let p2 = pp.get(pivot).copied();
        chain(p1, p2).map(|p| p.abs_diff(x)).min().unwrap()
    });

    let output = distances.join("\n");
    println!("{output}");
}
