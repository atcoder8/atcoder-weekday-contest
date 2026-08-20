use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        (n, k): (usize, usize),
        ww: [String; n],
    }

    let solve = |w: &str| {
        let s = w.chars().map(char_to_int).sum::<usize>() + w.len();

        if s % 2 == 1 {
            return if k % 2 == 0 {
                w.to_string()
            } else {
                w.chars().rev().collect()
            };
        }

        if w.len() % 2 == 0 {
            return w.chars().map(|ch| shift_char(ch, k)).collect();
        }

        let shifted = w.chars().map(|ch| shift_char(ch, 1));
        if k % 2 == 1 {
            shifted.collect()
        } else {
            shifted.rev().collect()
        }
    };

    let output = ww.iter().map(|w| solve(w)).join("\n");
    println!("{output}");
}

/// Converts a character to the corresponding integer.
pub fn char_to_int(c: char) -> usize {
    (c as u8 - b'a') as usize
}

fn shift_char(ch: char, num_shift: usize) -> char {
    (b'a' + ((char_to_int(ch) + num_shift) % 26) as u8) as char
}
