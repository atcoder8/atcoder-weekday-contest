use proconio::input;

fn main() {
    input! {
        (a, b, k): (u64, u64, u64),
    }

    println!("{}", (a - b) * k);
}
