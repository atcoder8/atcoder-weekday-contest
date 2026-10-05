use proconio::{input, marker::Chars};

fn main() {
    input! {
        (h, _w): (usize, usize),
        ss: [Chars; h],
    }

    let ans = (0..h)
        .rev()
        .max_by_key(|&row| ss[row].iter().filter(|&&s| s == '.').count())
        .unwrap()
        + 1;
    println!("{ans}");
}
