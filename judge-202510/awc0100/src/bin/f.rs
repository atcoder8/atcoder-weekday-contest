use proconio::input;

fn main() {
    input! {
        (n, k): (usize, u64),
        vv: [u64; n],
    }

    let mut num_combs = 0;
    let mut sum = 0;
    let mut right = 0;
    for left in 0..n {
        while right < n && sum < k {
            sum += vv[right];
            right += 1;
        }

        if sum < k {
            break;
        }

        num_combs += n - right + 1;

        sum -= vv[left];
    }

    println!("{num_combs}");
}
