use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xx: [i64; n],
    }

    xx.sort_unstable();
    let half_x = xx[n / 2];
    let cost = xx.iter().map(|x| (x - half_x).abs()).sum::<i64>();
    println!("{cost}");
}
