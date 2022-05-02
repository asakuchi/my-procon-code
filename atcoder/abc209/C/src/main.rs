use proconio::fastout;
use proconio::input;

const MODULO: usize = 1_000_000_007;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut c: [usize; n],
    }

    c.sort();

    let mut result = 1;

    for i in 0..n {
        if c[i] > i {
            result *= c[i] - i;
            result %= MODULO;
        } else {
            println!("0");
            return;
        }
    }

    println!("{}", result);
}
