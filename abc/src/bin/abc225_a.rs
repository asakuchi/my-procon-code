use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut set = std::collections::HashSet::new();

    for c in s {
        set.insert(c);
    }

    match set.len() {
        1 => println!("1"),
        2 => println!("3"),
        _ => println!("6"),
    }
}
