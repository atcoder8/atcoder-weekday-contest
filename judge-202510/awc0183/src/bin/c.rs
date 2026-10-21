use std::collections::HashSet;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ss: [String; n],
    }

    ss.sort_unstable_by_key(|s| s.len());

    let mut num_combs = 0_usize;
    let mut set = HashSet::<String>::new();
    for s in ss {
        num_combs += (0..=s.len())
            .tuple_combinations()
            .map(|(left, right)| &s[left..right])
            .unique()
            .filter(|&substring| set.contains(substring))
            .count();
        set.insert(s);
    }

    println!("{num_combs}");
}
