use proconio::fastout;
use proconio::input;
use std::cmp::min;

#[fastout]
fn main() {
    input! {
        n: i128,
        mut ab: [(i128, i128); n],
    }

    let mut a_to_b = 0;

    for (a, b) in &ab {
        a_to_b += b - a;
    }

    let mut entrance_result = std::i128::MAX;
    let mut exit_result = std::i128::MAX;

    for (a, b) in &ab {
        let mut sum_a = 0;
        let mut sum_b = 0;

        for (t_a, t_b) in &ab {
            sum_a += (a - t_a).abs();
            sum_b += (b - t_b).abs();
        }

        entrance_result = min(entrance_result, sum_a);
        exit_result = min(exit_result, sum_b);
    }

    println!("{}", a_to_b + entrance_result + exit_result);
}
