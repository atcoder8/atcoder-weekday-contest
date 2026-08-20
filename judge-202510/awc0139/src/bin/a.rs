use proconio::input;

fn main() {
    input! {
        n: usize,
        aa: [usize; n],
    }

    let mut num_abstainers = 0_usize;
    let mut counts = vec![0_usize; n];
    for &a in &aa {
        if a == 0 {
            num_abstainers += 1;
        } else {
            counts[a - 1] += 1;
        }
    }

    let num_candidates = counts.iter().filter(|&&cnt| cnt > num_abstainers).count();
    println!("{num_candidates}");
}
