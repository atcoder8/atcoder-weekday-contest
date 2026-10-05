use std::collections::VecDeque;

use proconio::input;

const DIFFS: [(usize, usize); 4] = [(!0, 0), (1, 0), (0, !0), (0, 1)];

fn main() {
    input! {
        (h, w): (usize, usize),
        aaa: [[u8; w]; h],
    }

    let mut dp = vec![vec![[None::<usize>; 2]; w]; h];
    let mut queue = VecDeque::from_iter([(0, 0, 0, 0), (0, 0, 0, 1)]);
    while let Some((row, col, dist, dir)) = queue.pop_front() {
        if dp[row][col][dir].is_some() || aaa[row][col] == 1 {
            continue;
        }

        dp[row][col][dir] = Some(dist);

        for &(dr, dc) in &DIFFS[2 * dir..2 * dir + 2] {
            let adjacent_row = row.wrapping_add(dr);
            let adjacent_col = col.wrapping_add(dc);
            if adjacent_row < h && adjacent_col < w {
                queue.push_back((adjacent_row, adjacent_col, dist + 1, 1 - dir));
            }
        }
    }

    let min_dist = dp[h - 1][w - 1][0]
        .into_iter()
        .chain(dp[h - 1][w - 1][1])
        .min();
    match min_dist {
        Some(min_dist) => println!("{min_dist}"),
        None => println!("NONE"),
    }
}
