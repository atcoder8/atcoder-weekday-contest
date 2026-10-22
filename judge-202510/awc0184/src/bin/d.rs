use proconio::input;

fn main() {
    input! {
        (n, w, k): (usize, usize, u32),
        hh: [u32; n],
    }

    let calc_rem = |bits: usize| {
        let mut rems = hh.clone();
        for i in 0..n {
            if bits >> i & 1 == 1 {
                let left = i.saturating_sub((w - 1) / 2);
                let right = (i + w / 2).min(n - 1);
                rems[left..=right].fill(0);
            }
        }

        rems.iter().sum::<u32>()
    };

    let min_rem = (0_usize..1 << n)
        .filter(|bits| bits.count_ones() <= k)
        .map(calc_rem)
        .min()
        .unwrap();
    println!("{min_rem}");
}
