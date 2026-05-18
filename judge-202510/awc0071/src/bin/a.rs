use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut depth = 0_usize;
    let max_depth = s
        .chars()
        .map(|ch| {
            if ch == '(' {
                depth += 1;
            } else {
                depth -= 1;
            }
            depth
        })
        .max()
        .unwrap();
    println!("{max_depth}");
}
