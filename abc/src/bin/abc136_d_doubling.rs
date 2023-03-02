use itertools::Itertools;
use procon_library_rs::doubling::Doubling;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let n = s.len();

    // 移動する回数
    let k = 1_000_000;

    let mut one_step = vec![0; n];

    for i in 0..n {
        if s[i] == 'R' {
            one_step[i] = i + 1;
        } else {
            one_step[i] = i - 1;
        }
    }

    let mut doubling = Doubling::new(k, &one_step);

    doubling.preprocess();

    let mut result = vec![0; n];

    for i in 0..n {
        let k_step_position = doubling.get(k, i);
        result[k_step_position] += 1;
    }

    let text = result.iter().format(" ");

    println!("{}", text);
}
