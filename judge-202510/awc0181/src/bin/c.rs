use proconio::input;

fn main() {
    input! {
        (n, c): (usize, u64),
        mut ww: [u64; n],
    }

    ww.sort_unstable();

    let mut cnt = 0;
    let mut sum = 0;
    for &w in &ww {
        if sum + w <= c {
            cnt += 1;
            sum += w;
        }
    }

    println!("{}", (cnt + 1).min(n));
}
