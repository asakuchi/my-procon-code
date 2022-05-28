use num_integer::*;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: usize,
        mut b: usize,
    }

    let mut result = 0;

    result += sum_n(n);
    result -= sum_n(n / a) * a;
    result -= sum_n(n / b) * b;

    let l = a.lcm(&b);

    result += sum_n(n / l) * l;

    println!("{}", result);
}

fn sum_n(n: usize) -> usize {
    n * (n + 1) / 2
}
