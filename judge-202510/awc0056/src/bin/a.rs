use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        aa: [i64; n],
        bs: [(Usize1, i64); m],
    }

    let score = bs.iter().map(|&(b, s)| aa[b] + s).sum::<i64>();
    println!("{score}");
}
