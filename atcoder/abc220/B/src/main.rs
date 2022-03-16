use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        k: usize,
        mut a: Chars,
        mut b: Chars,
    }

    let mut a_10 = 0;

    a.reverse();

    for i in 0..a.len() {
        let num: usize = a[i].to_string().parse().unwrap();
        a_10 += num * k.pow(i as u32);
    }

    let mut b_10 = 0;

    b.reverse();

    for i in 0..b.len() {
        let num: usize = b[i].to_string().parse().unwrap();
        b_10 += num * k.pow(i as u32);
    }

    println!("{}", a_10 * b_10);
}
