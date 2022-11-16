use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
    }

    let mut result = 0;

    a.sort_by_key(|&x| Reverse(x));

    // A3 - A2
    // A3      - A1
    // A3           - A0
    //      A2 - A1
    //      A2      - A0
    //           A1 - A0

    for i in 0..n {
        result += a[i] * (n as isize - 1 - i as isize);
        result -= a[i] * i as isize;
    }

    println!("{}", result);
}
