use proconio::input;

const MAX: u64 = 1000;

fn main() {
    input! {
        (n, k): (usize, usize),
        aa: [u64; n],
    }

    // dp[i][j]: i番目の友人まで、j番目の花まで
    let mut dp = vec![vec![MAX * k as u64; n + 1]; k + 1];
    dp[0][0] = 0;
    for i in 0..k {
        for left in 0..n {
            let mut min = MAX + 1;
            let mut max = 0;
            for right in left..n {
                let a = aa[right];
                min = min.min(a);
                max = max.max(a);
                let cand_effort = dp[i][left] + max - min;
                let effort = &mut dp[i + 1][right + 1];
                *effort = (*effort).min(cand_effort);
            }
        }
    }

    println!("{}", dp[k][n]);
}
