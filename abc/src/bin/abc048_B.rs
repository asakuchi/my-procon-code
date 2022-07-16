use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: isize,
        b: isize,
        x: isize,
    }

    let result = f(b, x) - f(a - 1, x);

    println!("{}", result);
}

fn f(n: isize, x: isize) -> isize {
    if n < 0 {
        return 0;
    }

    n / x + 1
}
