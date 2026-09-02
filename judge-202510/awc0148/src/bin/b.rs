use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(i32, i32); n],
    }

    let mut horizontal = BTreeMap::<i32, usize>::new();
    let mut vertical = BTreeMap::<i32, usize>::new();
    for &(x, y) in &xy {
        *horizontal.entry(x).or_default() += 1;
        *vertical.entry(y).or_default() += 1;
    }

    let grid_area = horizontal.len() * vertical.len();

    let solve = |x: i32, y: i32| {
        let remove_x = *horizontal.get(&x).unwrap() == 1;
        let remove_y = *vertical.get(&y).unwrap() == 1;

        let new_grid_area = match (remove_x, remove_y) {
            (true, true) => grid_area - (vertical.len() + horizontal.len() - 1),
            (true, false) => grid_area - vertical.len(),
            (false, true) => grid_area - horizontal.len(),
            (false, false) => grid_area,
        };

        new_grid_area - (n - 1)
    };

    let output = xy.iter().map(|&(x, y)| solve(x, y)).join("\n");
    println!("{output}");
}
