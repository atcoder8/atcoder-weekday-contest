use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let max = s.chars().max().unwrap();
    println!("{max}");
}
