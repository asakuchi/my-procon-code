use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        r: usize,
        c: usize,
    }

    let mut result = 4;

    if h == 1 {
        result -= 1;
    }

    if w == 1 {
        result -= 1;
    }

    if r == 1 || r == h {
        result -= 1;
    }

    if c == 1 || c == w {
        result -= 1;
    }

    println!("{}", result);
}
