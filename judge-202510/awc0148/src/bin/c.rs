use proconio::input;

fn main() {
    input! {
        (n, t, s): (usize, usize, usize),
        cp: [(usize, u64); n],
    }

    let u = t - s;

    let mut dp = vec![0; u + 1];
    for &(c, p) in &cp {
        for from in (0..(u + 1).saturating_sub(c)).rev() {
            dp[from + c] = dp[from + c].max(dp[from] + p);
        }
    }

    let max_score = *dp.iter().max().unwrap();
    println!("{max_score}");
}
