use proconio::input;

fn main() {
    input! {
        (h, _w): (usize, usize),
        ss: [String; h],
    }

    let ans = ss.iter().filter(|s| s.chars().all(|ch| ch == 'o')).count();
    println!("{ans}");
}
