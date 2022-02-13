use proconio::fastout;
use proconio::input;
use std::cmp::max;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[usize; m]; n],
    }

    let mut result = 0;

    for t1 in 0..m {
        for t2 in t1 + 1..m {
            let mut group_result = 0;

            for i in 0..n {
                group_result += max(a[i][t1], a[i][t2]);
            }

            result = max(result, group_result);
        }
    }

    println!("{}", result);
}
