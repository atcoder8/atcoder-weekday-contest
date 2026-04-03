use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, s): (usize, usize, u64),
        pp: [u64; n],
        tq: [(Usize1, u64); m],
    }

    let mut money = s;
    for &(t, q) in &tq {
        let x = pp[t] * q;
        money += (x + 1) / 2;
    }
    println!("{money}");
}
