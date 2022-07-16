use itertools::Itertools;
use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;
use std::collections::HashMap;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        s: [Chars; n],
    }

    let mut sets = vec![HashSet::new(); n];

    for i in 0..n {
        for &c in s[i].iter() {
            sets[i].insert(c);
        }
    }

    let mut result = 0;

    for combi in (1..=n).rev() {
        for combination in (0..n).combinations(combi) {
            let mut sum = HashMap::new();

            for i in combination {
                for &c in sets[i].iter() {
                    *sum.entry(c).or_insert(0) += 1;
                }
            }

            let mut kinds = 0;

            for (&_c, &num) in sum.iter() {
                if num == k {
                    kinds += 1;
                }
            }

            result = result.max(kinds);
        }
    }

    println!("{}", result);
}
