use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a_b: [(usize, usize); n],
    }

    a_b.sort();

    let mut kind = 0;

    for &(_a, b) in &a_b {
        kind += b;
    }

    let mut days = 1;

    if kind <= k {
        println!("{}", 1);
        return;
    }

    let mut prev = 0;

    for &(a, b) in &a_b {
        kind -= b;
        days += a - prev;

        if kind <= k {
            println!("{}", days);
            return;
        }

        prev = a;
    }
}
