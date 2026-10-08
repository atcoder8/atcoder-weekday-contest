use proconio::input;

fn main() {
    input! {
        (h, _w): (usize, usize),
        ss: [String; h],
    }

    let ans = ss
        .iter()
        .map(|s| s.chars().filter(|&ch| ch == 'x').count())
        .max()
        .unwrap();
    println!("{ans}");
}
