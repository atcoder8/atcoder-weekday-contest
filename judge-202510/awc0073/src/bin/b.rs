use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        mut fb: [(i32, i32); n],
    }

    fb.sort_unstable_by_key(|&(f, b)| b - f);

    let score =
        fb[..n - k].iter().map(|v| v.0).sum::<i32>() + fb[n - k..].iter().map(|v| v.1).sum::<i32>();
    println!("{score}");
}
