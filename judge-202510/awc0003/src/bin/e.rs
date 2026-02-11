use itertools::enumerate;
use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        ww: [u64; n],
        cc: [u64; m],
    }

    let mut cost_by_comb = vec![0; 1 << n];
    for (i, &w) in enumerate(&ww) {
        cost_by_comb[1 << i] = w;
    }
    for axis in 0..n {
        for bit in 0..1 << n {
            if bit >> axis & 1 == 1 {
                cost_by_comb[bit] += cost_by_comb[bit ^ (1 << axis)];
            }
        }
    }

    let mut dp = vec![false; 1 << n];
    dp[0] = true;
    let mut next_dp = vec![false; 1 << n];
    for &c in &cc {
        next_dp.copy_from_slice(&dp);

        for from in 0..1 << n {
            if !dp[from] {
                continue;
            }

            let rem_bits = !from & ((1 << n) - 1);
            let mut add_bits = rem_bits;
            while add_bits != 0 {
                if cost_by_comb[add_bits] <= c {
                    next_dp[from | add_bits] = true;
                }
                add_bits = (add_bits - 1) & rem_bits;
            }
        }

        std::mem::swap(&mut dp, &mut next_dp);
    }

    println!("{}", if dp[(1 << n) - 1] { "Yes" } else { "No" });
}
