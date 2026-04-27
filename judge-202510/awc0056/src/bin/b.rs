use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut aa: [u64; n],
    }

    aa.sort_unstable();

    let ans = aa[..n - k].iter().sum::<u64>() + aa[n - k..].iter().map(|a| a / 2).sum::<u64>();
    println!("{ans}");
}
