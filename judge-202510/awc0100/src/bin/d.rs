use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [u64; n],
    }

    let sum = aa.iter().sum::<u64>();
    let mut acc = 0;
    let ans = aa[..n - 1]
        .iter()
        .map(|a| {
            acc += a;
            acc.abs_diff(sum - acc)
        })
        .min()
        .unwrap();
    println!("{ans}");
}
