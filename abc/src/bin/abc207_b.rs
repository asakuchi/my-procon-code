use num_integer;
use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    }

    if c * d <= b {
        println!("-1");
        return;
    }

    let x = num_integer::div_ceil(a, c * d - b);
    println!("{}", x);
}
