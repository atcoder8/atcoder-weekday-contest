use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        ww: [u32; n],
        uv: [(Usize1, Usize1); m],
    }

    let mut flags = vec![false; n];
    for &(u, v) in &uv {
        if ww[u] < ww[v] {
            flags[u] = true;
        } else if ww[u] > ww[v] {
            flags[v] = true;
        }
    }

    let ans = flags.iter().filter(|&&f| f).count();
    println!("{ans}");
}
