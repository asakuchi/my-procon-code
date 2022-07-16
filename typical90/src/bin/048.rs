use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        mut problems: [(usize, usize); n],
    }

    let mut list: Vec<_> = problems.iter().map(|ab| ab.0 - ab.1).collect();
    let mut part_scores: Vec<_> = problems.iter().map(|ab| ab.1).collect();
    list.append(&mut part_scores);

    list.sort_by_key(|&x| std::cmp::Reverse(x));

    let score: usize = list.iter().take(k).sum();

    println!("{}", score);
}
