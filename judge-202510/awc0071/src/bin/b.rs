use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        _n: usize,
        s: String,
    }

    let ans = s
        .chars()
        .dedup_with_count()
        .tuple_windows()
        .map(|((left_cnt, left_ch), _, (right_cnt, right_ch))| {
            if left_ch == right_ch {
                left_cnt * right_cnt
            } else {
                0
            }
        })
        .sum::<usize>();
    println!("{ans}");
}
