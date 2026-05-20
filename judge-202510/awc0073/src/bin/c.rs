use proconio::input;

fn main() {
    input! {
        (n, d): (usize, u32),
        mut xs: [(u32, u64); n],
    }

    xs.sort_unstable_by_key(|v| v.0);

    let mut score = 0;
    let mut sum = 0;
    let mut right = 0;
    for left in 0..n {
        while right < n && xs[right].0 - xs[left].0 <= d {
            score += sum * xs[right].1;
            sum += xs[right].1;
            right += 1;
        }

        sum -= xs[left].1;
    }

    println!("{score}");
}
