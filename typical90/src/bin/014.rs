// -*- coding:utf-8-unix -*-

use proconio::input;

///
/// 014 - We Used to Sing a Song Together（★3）
///
fn main() {
    input! {
        n: usize,
        mut a: [isize; n],
        mut b: [isize; n],
    }

    a.sort();
    b.sort();

    let sum = a
        .iter()
        .zip(b.iter())
        .fold(0, |sum, (x, y)| sum + (x - y).abs());

    println!("{}", sum);
}
