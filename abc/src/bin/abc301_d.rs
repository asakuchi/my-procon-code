use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        mut n: usize,
    }

    let keta = s.len();

    let mut target = 1 << (keta - 1);

    let mut result = 0_usize;

    for i in 0..keta {
        if s[i] == '1' {
            if n >= target {
                result += target;
                n -= target;
            } else {
                println!("-1");
                return;
            }
        }

        target /= 2;
    }

    let mut target = 1 << (keta - 1);

    for i in 0..keta {
        if s[i] == '?' {
            if n >= target {
                result += target;
                n -= target;
            }
        }

        target /= 2;
    }

    println!("{}", result);
}
