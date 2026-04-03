use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut xx: [usize; n],
    }

    xx.sort_unstable();

    let mut max = 0;
    let mut right = 0;
    for left in 0..n {
        while right < n && xx[right] - xx[left] <= k {
            right += 1;
        }
        max = max.max(right - left);
    }

    println!("{max}");
}
