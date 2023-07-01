use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a_b: [(usize, usize); n],
    }

    let text = a_b
        .iter()
        .enumerate()
        .sorted_by(|(_i, p_1), (_j, p_2)| (p_2.0 * (p_1.0 + p_1.1)).cmp(&(p_1.0 * (p_2.0 + p_2.1))))
        .map(|(i, _)| i + 1)
        .join(" ");

    println!("{}", text);
}
