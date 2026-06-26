use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, l, q): (usize, usize, usize),
        ss: [String; n],
        ccc: [[Usize1]; q],
    }

    let ss = ss
        .iter()
        .map(|s| {
            s.chars()
                .enumerate()
                .map(|(i, c)| {
                    if c == '0' {
                        0
                    } else {
                        2_u64.pow((l - 1 - i) as u32)
                    }
                })
                .sum::<u64>()
        })
        .collect_vec();

    let output = ccc
        .iter()
        .map(|cc| format!("{:0l$b}", cc.iter().fold(0_u64, |acc, &c| acc | ss[c])))
        .join("\n");
    println!("{output}");
}
