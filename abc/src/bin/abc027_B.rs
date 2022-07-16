use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let total: isize = a.iter().sum();

    if total % n as isize != 0 {
        println!("-1");
        return;
    }

    let average = total / n as isize;

    let mut result = 0;
    let mut moving = 0;

    for &num in a.iter() {
        moving += num - average;

        if moving != 0 {
            result += 1;
        }
    }

    println!("{}", result);
}
