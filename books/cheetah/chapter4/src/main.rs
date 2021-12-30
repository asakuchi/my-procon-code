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

        let delta = std::cmp::min(available, bottles[from]);

        bottles[to] += delta;
        bottles[from] -= delta;
    }

    println!("{:?}", bottles);
}
