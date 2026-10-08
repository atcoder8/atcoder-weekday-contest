use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(u32, u32); n],
    }

    let output = ab.iter().map(|&(a, b)| (a + b) % 24).join("\n");
    println!("{output}");
}
