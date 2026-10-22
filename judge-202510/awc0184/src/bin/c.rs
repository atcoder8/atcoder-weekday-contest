use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        (n, m): (usize, usize),
        mut dd: [u32; m],
        ss: [u32; n],
    }

    dd.sort_unstable();

    let output = ss.iter().map(|s| dd.upper_bound(s)).join("\n");
    println!("{output}");
}
