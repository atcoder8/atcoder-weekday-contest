use proconio::input;

fn main() {
    input! {
        (n, k): (usize, u64),
        aa: [u64; n],
    }

    let sum_a = aa.iter().sum::<u64>();

    let mut cnt = 0;
    let mut sum = 0;
    let mut right = 0;
    for left in 0..n {
        while right <= left || (right < n && sum + k < sum_a) {
            sum += aa[right];
            right += 1;
        }

        cnt += (sum + k == sum_a) as usize;
        sum -= aa[left];
    }
    cnt += (sum + k == sum_a) as usize;

    println!("{cnt}");
}
