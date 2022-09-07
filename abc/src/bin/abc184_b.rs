use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        mut x: usize,
        s: Chars,
    }

    for c in s {
        if c == 'o' {
            x += 1;
        } else {
            if x > 0 {
                x -= 1;
            }
        }
    }

    println!("{}", x);
}
