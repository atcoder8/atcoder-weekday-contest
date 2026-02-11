use proconio::input;

fn main() {
    input! {
        (n, k): (usize, u64),
        ab: [(u64, u64); n],
    }

    let ans = ab.iter().filter(|&&(a, b)| a * b >= k).count();
    println!("{ans}");
}
