use proconio::fastout;
use proconio::input;
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

fn rec(h: usize, memo: &mut HashMap<usize, usize>) -> usize {
    // println!("rec {}", h);

    if h == 1 {
        return 1;
    }

    if let Some(&value) = memo.get(&h) {
        return value;
    }

    let mut count = 1;

    count += rec((h as f64 / 2.).floor() as usize, memo);
    count += rec((h as f64 / 2.).floor() as usize, memo);

    memo.insert(h, count);

    count
}
