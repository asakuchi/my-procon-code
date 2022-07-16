// -*- coding:utf-8-unix -*-

use proconio::fastout;
use proconio::input;
// use proconio::derive_readable;
// use proconio::marker::Chars;
// use itertools::izip;
// use itertools::Itertools;

#[fastout]
fn main() {
    input! {
        n: u128,
    }

    // println!("{}", keta);

    let n_keta = n.to_string().len();

    let mut answer: u128 = 0;

    // println!("n_keta:{}", n_keta);

    for i in 0..(n_keta - 1) as u128 {
        // println!("i:{}", i);
        // answer += 9 * i * (9 * i + 1) / 2;
        let temp = 9 * (10u128.pow((i) as u32)) * (9 * (10u128.pow((i) as u32)) + 1) / 2;
        // println!("temp:{}", temp);
        answer += temp % 998244353;
        // answer += (9 * i * (9 * i + 1) / 2) % 998244353;
    }

    // println!("ans_other:{}", answer);

    // let kousu = n % 998244353 - (10u128.pow((n_keta - 1) as u32) % 998244353) + 1;
    let kousu = n - (10u128.pow((n_keta - 1) as u32)) + 1;
    // println!("kousu:{}", kousu);

    let temp = (kousu * (kousu + 1)) / 2;

    // println!("{}", temp);

    answer += temp;

    // println!("{}", kousu);

    println!("{}", answer % 998244353);
}
