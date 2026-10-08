use std::borrow::Cow;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, q): (usize, usize),
        mut xx: [u64; n],
        lrab: [(usize, usize, u64, u64); n],
        cc: [u64; q],
    }

    xx.insert(0, 0);

    // dp[i][j]: 起用する配達員が i 人、最後の届け先が j であるときの最小コスト
    let mut dp = vec![vec![None::<u64>; n + 1]; n + 1];
    dp[0][0] = Some(0);
    for num_couriers in 0..n {
        for dest in 0..n {
            let Some(cost) = dp[num_couriers][dest] else {
                continue;
            };
            let (l, r, a, b) = lrab[dest];
            for next_dest in dest + l..=(dest + r).min(n) {
                chmin_for_option(
                    &mut dp[num_couriers + 1][next_dest],
                    cost + a * (xx[next_dest] - xx[dest]) + b,
                );
            }
        }
    }

    let solve = |c: u64| {
        (1..=n)
            .filter_map(|num_couriers| Some(dp[num_couriers][n]? + c * num_couriers as u64))
            .min()
    };

    let output = cc
        .iter()
        .map(|&c| match solve(c) {
            Some(min_cost) => Cow::Owned(min_cost.to_string()),
            None => Cow::Borrowed("-1"),
        })
        .join("\n");
    println!("{output}");
}

/// If `value` is `None` or contains a value greater than `cand_value`, update it to `Some(cand_value)`.
///
/// Returns whether `value` has been updated or not as a bool value.
///
/// # Arguments
///
/// * `value` - Reference variable to be updated.
/// * `cand_value` - Candidate value for update.
pub fn chmin_for_option<T>(value: &mut Option<T>, cand_value: T) -> bool
where
    T: PartialOrd,
{
    if value.as_ref().is_some_and(|cost| cost <= &cand_value) {
        return false;
    }

    *value = Some(cand_value);

    true
}
