use proconio::input;

fn main() {
    input! {
        (n, m, k): (usize, u64, u64),
        aa: [u64; n],
    }

    let mut cost = 0;
    let mut rem = m;
    for &a in &aa {
        let required = a.div_ceil(k);
        let available = required.min(rem);
        rem -= available;
        cost += required - available;
    }

    println!("{cost}");
}
