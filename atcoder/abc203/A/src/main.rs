use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    if a == b {
        println!("{}", c);
        return;
    } else if b == c {
        println!("{}", a);
        return;
    } else if c == a {
        println!("{}", b);
        return;
    }

    println!("0");
}
