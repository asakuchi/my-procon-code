use proconio::fastout;
use proconio::input;
// use proconio::derive_readable;
// use proconio::marker::Chars;
// use itertools::izip;
// use itertools::Itertools;
use std::collections::HashMap;
#[fastout]
fn main() {
    input! {
        mut h: usize,
    }

    let mut memo = HashMap::new();
    let count = rec(h, &mut memo);

    println!("{}", count);
}

fn rec(h: usize, memo: HashMap<usize, usize>) -> usize {
    if h == 1 {
        return 1;
    }

    if let Some(&value) = memo.get(&h) {
        return value;
    }

    let mut count = 0;

    count += rec((h as f64 / 2.).floor() as usize * 2, memo) * 2;

    memo.insert(h, count);

    count
}
