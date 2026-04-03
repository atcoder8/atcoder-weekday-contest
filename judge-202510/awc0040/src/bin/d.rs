use std::collections::BinaryHeap;

use proconio::input;

fn main() {
    let answer = match solve() {
        Some(value) => format!("{}", value),
        None => "-1".to_string(),
    };
    println!("{}", answer);
}

fn solve() -> Option<usize> {
    input! {
        (n, g, f): (usize, u64, u64),
        mut pr: [(u64, u64); n],
    }

    pr.sort_unstable();
    pr.push((g, 0));

    let mut heap = BinaryHeap::<u64>::new();
    let mut rem = f;
    let mut prev = 0;
    let mut cnt = 0;
    for &(p, r) in &pr {
        let dist = p - prev;
        while rem < dist {
            if let Some(r) = heap.pop() {
                rem += r;
                cnt += 1;
            } else {
                return None;
            }
        }

        rem -= dist;
        heap.push(r);
        prev = p;
    }

    Some(cnt)
}
