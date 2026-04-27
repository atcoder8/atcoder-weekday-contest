use proconio::input;

fn main() {
    input! {
        (n, k): (usize, i64),
        aa: [i64; n],
    }

    let mut max_score = 0;
    let mut sum = 0;
    let mut right = 0;
    for left in 0..n {
        while right < n && sum + aa[right] <= k {
            sum += aa[right];
            right += 1;
        }
        max_score = max_score.max(right - left);
        sum -= aa[left];
    }
    println!("{max_score}");
}
