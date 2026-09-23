use proconio::input;

fn main() {
    input! {
        (mut l, w): (u64, u64),
    }

    let mut cnt = 0_u32;
    while l > w {
        l = l.div_ceil(2);
        cnt += 1;
    }
    println!("{cnt}");
}
