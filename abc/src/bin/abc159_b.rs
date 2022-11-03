use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    if is_kaibun(&s) {
        if is_kaibun(&s[..(s.len() - 1) / 2]) && is_kaibun(&s[((s.len() + 3) / 2) - 1..]) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

fn is_kaibun(s: &[char]) -> bool {
    // println!("{:?}", s);

    for i in 0..s.len() {
        let other = s.len() - 1 - i;

        if s[i] != s[other] {
            return false;
        }
    }

    true
}
