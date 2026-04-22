use std::collections::{BTreeSet, BinaryHeap};

use itertools::{Itertools, enumerate, izip};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        (n, m): (usize, usize),
        lrc: [(Usize1, usize, u32); m],
    }

    let mut in_events = vec![vec![]; n + 1];
    let mut out_events = vec![vec![]; n + 1];
    for (id, &(l, r, c)) in enumerate(&lrc) {
        in_events[l].push((id, c));
        out_events[r].push(id);
    }

    let mut color_heap = BinaryHeap::new();
    let mut out_ids = BTreeSet::new();

    let mut colors = vec![0; n + 1];
    for (i, (in_event, out_event)) in enumerate(izip!(in_events, out_events)) {
        color_heap.extend(in_event);
        out_ids.extend(out_event);
        while let Some(&(id, _)) = color_heap.peek()
            && out_ids.contains(&id)
        {
            color_heap.pop();
            out_ids.remove(&id);
        }

        if let Some(&(_, color)) = color_heap.peek() {
            colors[i] = color;
        }
    }

    println!("{}", colors[..n].iter().join(" "));
}
