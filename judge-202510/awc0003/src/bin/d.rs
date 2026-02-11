use proconio::input;

fn main() {
    input! {
        (n, k, m): (usize, usize, u64),
        aa: [u64; n],
    }

    let mut exclusive = 0;
    let mut sum = 0;
    let mut right = 0;
    for left in 0..n {
        while right < n && (right + 1 - left < k || sum + aa[right] < m) {
            sum += aa[right];
            right += 1;
        }
        exclusive += right - left;
        sum -= aa[left];
    }

    let ans = n * (n + 1) / 2 - exclusive;
    println!("{ans}");
}
