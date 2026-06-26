use std::cmp::Reverse;

use itertools::{Itertools, enumerate};
use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [u32; n],
    }

    let ia = enumerate(aa).sorted_unstable_by_key(|v| Reverse(v.1));
    let mut groups: Vec<(u32, Vec<usize>)> = vec![];
    for (i, a) in ia {
        if let Some((value, indices)) = groups.last_mut()
            && *value == a
        {
            indices.push(i);
        } else {
            groups.push((a, vec![i]));
        }
    }

    let mut counts = vec![0_usize; n];
    let mut acc = 0;
    for (_, indices) in groups {
        indices.iter().for_each(|&idx| counts[idx] = acc);
        acc += indices.len();
    }

    println!("{}", counts.iter().join(" "));
}
