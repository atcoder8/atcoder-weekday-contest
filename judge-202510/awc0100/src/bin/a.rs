use proconio::input;

fn main() {
    input! {
        n: usize,
        at: [(i64, i64); n],
    }

    let ans = at.iter().map(|(a, t)| a * t).sum::<i64>();
    println!("{ans}");
}
