use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut has_omoji = false;
    let mut has_komoji = false;

    let mut set = std::collections::HashSet::new();

    for &c in s.iter() {
        if c >= 'a' && c <= 'z' {
            has_komoji = true;
        } else {
            has_omoji = true;
        }

        set.insert(c);
    }

    if has_komoji && has_omoji && s.len() == set.len() {
        println!("Yes");
    } else {
        println!("No");
    }
}
