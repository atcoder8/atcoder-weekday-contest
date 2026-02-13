use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        (n, m): (usize, usize),
        ww: [u64; n],
        mut cc: [u64; m],
    }

    // トラックを積載重量の上限に関して降順にソート
    cc.sort_unstable_by_key(|&c| Reverse(c));

    // 既に積み込まれた荷物の集合に対する(現在のトラックの番号, 現在のトラックの荷重)の最小値
    // 積み込み方法が見つかっていない場合は`None`
    let mut dp: Vec<Option<(usize, u64)>> = vec![None; 1 << n];
    dp[0] = Some((0, 0));
    for from in 0..1 << n {
        let Some((curr_track, curr_load)) = dp[from] else {
            continue;
        };

        for add_bit in 0..n {
            if from >> add_bit & 1 == 1 {
                continue;
            }

            // 現在のトラック、または次のトラックに積み込む
            // トラックを積載重量の上限に関して降順にソートしているためその次のトラックを参照する必要はない
            let to = from | (1 << add_bit);
            let weight = ww[add_bit];
            if curr_load + weight <= cc[curr_track] {
                chmin_for_option(&mut dp[to], (curr_track, curr_load + weight));
            } else if curr_track + 1 < m && weight <= cc[curr_track + 1] {
                chmin_for_option(&mut dp[to], (curr_track + 1, weight));
            }
        }
    }

    // 全ての荷物を積み込む方法が見つかったかどうかを判定
    let loadable = dp[(1 << n) - 1].is_some();
    println!("{}", if loadable { "Yes" } else { "No" });
}

/// `value`の値よりも`cand_value`が小さい場合は値の更新を行います。
/// ただし、`value`が`None`である場合は常に更新します。
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
