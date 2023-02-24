use num::Integer;
use num_integer::ExtendedGcd;
use proconio::input;

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    input! {
        n: isize,
        s: isize,
        k: isize,
    }

    let ExtendedGcd { gcd, x, .. } = k.extended_gcd(&n);

    if s % gcd != 0 {
        println!("-1");
        return;
    }

    let n = n / gcd;
    let s = s / gcd;

    let result = (((x * -s) % n) + n) % n;

    println!("{}", result);
}
