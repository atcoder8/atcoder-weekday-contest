use proconio::input;

fn main() {
    input! {
        n: usize,
        _aa: [u32; n],
    }

    println!("{}", n - 2);
}
