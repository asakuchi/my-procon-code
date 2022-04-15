// use proconio::fastout;
use proconio::input;
use std::collections::BTreeMap;

// #[fastout]
fn main() {
    input! {
        n: usize,
        mut ab: [(isize, isize); n],
    }

    let mut map = BTreeMap::new();

    for &(a, b) in ab.iter() {
        *map.entry(a).or_insert(0) += 1;
        *map.entry(a + b).or_insert(0) += -1;
    }

    let mut result = vec![0; n + 1];

    let mut current: isize = 0;
    let mut prev_date = 0;

    for (&date, &updown) in map.iter() {
        result[current as usize] += date - prev_date;
        prev_date = date;
        current += updown;
    }

    println!(
        "{}",
        &result[1..=n]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    );
}
