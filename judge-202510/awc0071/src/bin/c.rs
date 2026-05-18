use proconio::input;

fn main() {
    input! {
        n: usize,
        ww: [u64; n],
    }

    let is_ok = |d: u64| {
        let mut curr = 0;
        for &w in &ww {
            curr += w;

            if curr > d {
                return false;
            }

            if curr == d {
                curr = 0;
            }
        }

        curr == 0
    };

    let sum_w = ww.iter().sum::<u64>();
    let divisors = find_divisors(sum_w);

    let ans = divisors
        .iter()
        .find_map(|&d| if is_ok(d) { Some(sum_w / d) } else { None })
        .unwrap();
    println!("{ans}");
}

/// Creates a sequence consisting of the divisors of `n`.
pub fn find_divisors(n: u64) -> Vec<u64> {
    assert_ne!(n, 0, "`n` must be at least 1.");

    let mut divisors = vec![];

    for i in (1..).take_while(|&i| i <= n / i) {
        if n % i == 0 {
            divisors.push(i);

            if n / i != i {
                divisors.push(n / i);
            }
        }
    }

    divisors.sort_unstable();

    divisors
}
