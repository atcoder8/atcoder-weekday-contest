use proconio::input;

fn main() {
    input! {
        (n, m, _k): (usize, usize, usize),
        aa: [u32; n],
        bb: [u32; m],
    }

    let max_a = *aa.iter().max().unwrap();
    let ans = bb.iter().take_while(|&&b| max_a >= b).count();
    println!("{ans}");
}
