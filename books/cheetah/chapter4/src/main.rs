// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        capacities: [usize;n],
        mut bottles: [usize;n],
        from_id: [usize; m],
        to_id: [usize; m],
    }

    for (&from, &to) in from_id.iter().zip(to_id.iter()) {
        let available = capacities[to] - bottles[to];

        // to が満杯になる場合
        if available < bottles[from] {
            bottles[to] += available; // full
            bottles[from] -= available;
        } else {
            // from が空になる場合
            bottles[to] += bottles[from];
            bottles[from] = 0; // empty;
        }
    }

    println!("{:?}", bottles);
}
