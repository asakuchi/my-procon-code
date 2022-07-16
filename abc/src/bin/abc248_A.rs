use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        s: Chars,
    }

    let mut list = vec![true; 10];

    for c in s {
        let i: usize = c.to_string().parse().unwrap();
        list[i] = false;
    }

    for i in 0..=9 {
        if list[i] {
            println!("{}", i);
            return;
        }
    }
}
