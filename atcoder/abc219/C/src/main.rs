use proconio::input;
use proconio::marker::Chars;

// #[fastout]
fn main() {
    input! {
        x: Chars,
        n: usize,
        mut s: [String; n],
    }

    let mut rank = std::collections::HashMap::new();

    for (i, &c) in x.iter().enumerate() {
        rank.insert(c, i);
    }

    s.sort_by_key(|name| name.chars().map(|c| rank[&c]).collect::<Vec<_>>());

    for name in s.iter() {
        println!("{}", name);
    }
}
