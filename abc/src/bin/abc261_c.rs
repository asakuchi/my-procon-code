use std::collections::HashMap;

use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut map = HashMap::new();

    for i in 0..n {
        let count = map.entry(s[i].clone()).or_insert(0);

        if *count == 0 {
            println!("{}", s[i]);
        } else {
            println!("{}({})", s[i], *count);
        }

        *count += 1;
    }
}
