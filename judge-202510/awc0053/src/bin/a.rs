use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [u64; n],
    }

    let sum_a = aa.iter().sum::<u64>();
    let ans = if sum_a % 2 == 0 { "Aoki" } else { "Takahashi" };
    println!("{ans}");
}
