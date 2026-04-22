use itertools::Itertools;
use proconio::input;

#[derive(Debug, Clone, Copy)]
struct Event {
    event_in: bool,
    time: i64,
    c: i64,
}

fn main() {
    input! {
        n: usize,
        xlrc: [(i64, i64, i64, i64); n],
    }

    let events = xlrc
        .iter()
        .flat_map(|&(x, l, r, c)| {
            [
                Event {
                    event_in: true,
                    time: x - l,
                    c,
                },
                Event {
                    event_in: false,
                    time: x + r + 1,
                    c,
                },
            ]
        })
        .sorted_unstable_by_key(|event| (event.time, event.event_in));

    let mut max_strength = 0;
    let mut strength = 0;
    for event in events {
        if event.event_in {
            strength += event.c;
        } else {
            strength -= event.c;
        }
        max_strength = max_strength.max(strength);
    }
    println!("{max_strength}");
}
