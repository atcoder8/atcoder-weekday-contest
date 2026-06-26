use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i32, i32); n],
    }

    let ans = ab.iter().position_max_by_key(|(a, b)| a + b).unwrap() + 1;
    println!("{ans}");
}
