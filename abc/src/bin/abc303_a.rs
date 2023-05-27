use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }

    for i in 0..n {
        if s[i] == t[i] {
        } else if s[i] == '1' && t[i] == 'l' {
        } else if t[i] == '1' && s[i] == 'l' {
        } else if s[i] == '0' && t[i] == 'o' {
        } else if t[i] == '0' && s[i] == 'o' {
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
