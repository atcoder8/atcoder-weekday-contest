use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m, k): (usize, usize, i64),
        aa: [i64; n],
        lr: [(Usize1, usize); m],
    }

    let mut imos = vec![0; n + 1];
    for &(l, r) in &lr {
        imos[l] += 1;
        imos[r] -= 1;
    }
    for i in 0..n {
        imos[i + 1] += imos[i];
    }

    let cnt = (0..n).filter(|&i| aa[i] + imos[i] >= k).count();
    println!("{cnt}");
}
