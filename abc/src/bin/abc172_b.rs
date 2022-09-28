use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut result = 0;

    for (&c1, &c2) in s.iter().zip(t.iter()) {
        if c1 != c2 {
            result += 1;
        }
    }

    println!("{}", result);
}
