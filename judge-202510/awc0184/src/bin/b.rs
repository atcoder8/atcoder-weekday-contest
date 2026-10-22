use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let count = s.chars().dedup().filter(|&ch| ch == '#').count();
    println!("{count}");
}
