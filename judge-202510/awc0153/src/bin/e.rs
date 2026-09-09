// unfinished

use std::{cmp::Reverse, collections::BinaryHeap};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, d): (usize, u64),
        hh: [i64; n],
    }

    let mut hh = hh.into_iter().map(|h| h.max(0) as u64).collect_vec();
    // 何ターン後に感染するかを管理
    let mut heap = BinaryHeap::from_iter(
        hh.iter()
            .positions(|&h| h == 0)
            .map(|pos| (Reverse(0), pos)),
    );
    for turn in 0.. {
        let mut infected = false;

        while let Some(&(Reverse(infected_turn), pos)) = heap.peek()
            && infected_turn == turn
        {
            heap.pop();
            if hh[pos] == 0 {
                continue;
            }
            infected = true;
            hh[pos] = 0;
            // -1
            if pos > 0 && hh[pos - 1] > 0 {
                let reduce = d + d * (pos > 2 && hh[pos - 2] == 0) as u64;
                heap.push((Reverse(turn + hh[pos - 1].div_ceil(reduce)), pos - 1));
            }
            // +1
            if pos + 1 < n && hh[pos + 1] > 0 {
                let reduce = d + d * (pos + 2 < n && hh[pos + 2] == 0) as u64;
                heap.push((Reverse(turn + hh[pos + 1].div_ceil(reduce)), pos + 1));
            }
        }

        if !infected {
            break;
        }
    }

    let cnt = hh.iter().filter(|&&h| h == 0).count();
    println!("{cnt}");
}
