use proconio::input;

fn main() {
    input! {
        (h, _w): (usize, usize),
        ss: [String; h],
    }

    let ans = ss.iter().position(|s| s.contains("X"));
    match ans {
        Some(ans) => println!("{}", ans + 1),
        None => println!("-1"),
    }
}
