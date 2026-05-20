use proconio::input;

fn main() {
    input! {
        (k, m): (usize, usize),
        mut aa: [u32; k],
        mut bb: [u32; m],
    }

    aa.sort_unstable();
    bb.sort_unstable();

    let mut cnt = 0;
    let mut idx = 0;
    for &b in &bb {
        while idx < k && aa[idx] < b {
            idx += 1;
        }

        if idx == k {
            break;
        }

        cnt += (aa[idx] == b) as usize;
    }

    println!("{cnt}");
}
