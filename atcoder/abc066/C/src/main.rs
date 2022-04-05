use proconio::fastout;
use proconio::input;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut b = VecDeque::with_capacity(n);

    for i in 0..n {
        if i % 2 == 0 {
            b.push_back(a[i]);
        } else {
            b.push_front(a[i]);
        }
    }

    let mut b: Vec<_> = b.into();

    if n % 2 != 0 {
        b.reverse();
    }

    let result: String = b
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join(" ");

    println!("{}", result);
}
