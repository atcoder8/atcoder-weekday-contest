use proconio::input;

fn main() {
    input! {
        (a, b, c, d): (u32, u32, u32, u32),
    }

    let ans = a + b * d >= c;
    println!("{}", if ans { "Yes" } else { "No" });
}
