use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut a_index = s.len();
    let mut z_index = 0;

    for i in 0..s.len() {
        if s[i] == 'A' {
            a_index = a_index.min(i);
        } else if s[i] == 'Z' {
            z_index = z_index.max(i);
        }
    }

    println!("{}", z_index - a_index + 1);
}
