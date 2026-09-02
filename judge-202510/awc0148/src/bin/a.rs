use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, d, k): (usize, usize, usize),
        sss: [[Usize1]; d],
    }

    let mut counts = vec![0_usize; n];
    for ss in &sss {
        ss.iter().for_each(|&s| counts[s] += 1);
    }

    let regular_users = (0..n).filter(|&i| counts[i] >= k).collect_vec();

    let output = if regular_users.is_empty() {
        "-1".to_string()
    } else {
        regular_users.iter().map(|i| i + 1).join(" ")
    };
    println!("{output}");
}
