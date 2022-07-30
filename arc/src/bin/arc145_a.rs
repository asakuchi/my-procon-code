use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    if s.len() == 2 {
        if (s[0] == 'A' && s[1] == 'A') || (s[0] == 'B' && s[1] == 'B') {
            println!("Yes");
            return;
        } else {
            println!("No");
            return;
        }
    }

    if s[0] == 'B' || s[s.len() - 1] == 'A' {
        println!("Yes");
    } else {
        println!("No");
    }
}
