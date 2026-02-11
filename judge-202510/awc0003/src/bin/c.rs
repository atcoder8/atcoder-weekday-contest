use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut ab: [(u64, u64); n],
    }

    ab.sort_unstable_by_key(|&(a, b)| a - b);

    let ans = ab[..n - k].iter().map(|&(a, _)| a).sum::<u64>()
        + ab[n - k..].iter().map(|&(_, b)| b).sum::<u64>();
    println!("{ans}");
}
