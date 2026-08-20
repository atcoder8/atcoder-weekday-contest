use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        q: usize,
        nn: [usize; q],
    }

    let solve = |n: usize| {
        let divisors = find_divisors(n);
        divisors
            .iter()
            .map(|&divisor| divisor.min(n / divisor))
            .max()
            .unwrap()
    };

    let output = nn.into_iter().map(solve).join("\n");
    println!("{output}");
}

/// Creates a sequence consisting of the divisors of `n`.
pub fn find_divisors(n: usize) -> Vec<usize> {
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
