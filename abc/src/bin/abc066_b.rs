use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    for i in (0..s.len()).rev() {
        if even_string(&s[0..i]) {
            println!("{}", i);
            return;
        }
    }
}

fn even_string(s: &[char]) -> bool {
    if s.len() % 2 != 0 {
        return false;
    }

    for i in 0..s.len() / 2 {
        if s[i] != s[s.len() / 2 + i] {
            return false;
        }
    }

    true
}
