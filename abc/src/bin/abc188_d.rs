use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        prime_cost: isize,
        a_b_c: [(usize, usize, isize); n],
    }

    let mut map = BTreeMap::new();

    for &(a, b, c) in &a_b_c {
        *map.entry(a).or_insert(0) += c;
        *map.entry(b + 1).or_insert(0) -= c;
    }

    // println!("{:?}", map);

    let mut total_cost = 0;
    let mut sum_diff = 0;

    let mut prev_day = 0;

    for (day, diff) in map {
        total_cost += (day - prev_day) as isize
            * if sum_diff > prime_cost {
                prime_cost
            } else {
                sum_diff
            };

        // println!(
        //     "day {} prev_day {} sum_diff {} total_cost {}",
        //     day, prev_day, sum_diff, total_cost
        // );

        sum_diff += diff;
        prev_day = day;
    }

    println!("{}", total_cost);
}
