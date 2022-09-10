use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [isize; n],
    }

    let mut map = HashMap::new();

    for i in 0..n {
        let sabun = (i as isize + n as isize - p[i]) % n as isize;
        let sabun = sabun as usize;

        let sabun_minus_1 = (sabun + n - 1) % n;

        let sabun_plus_1 = (sabun + n + 1) % n;

        *map.entry(sabun).or_insert(0) += 1;
        *map.entry(sabun_minus_1).or_insert(0) += 1;
        *map.entry(sabun_plus_1).or_insert(0) += 1;
    }

    let mut diff = 0;
    let mut max_value = 0;

    for (&key, &value) in map.iter() {
        if value > max_value {
            max_value = value;
            diff = key;
        }
    }

    let mut score = 0;

    for i in 0..n {
        if p[(i + n + diff as usize - 1) % n] == i as isize
            || p[(i + n + diff as usize) % n] == i as isize
            || p[(i + n + diff as usize + 1) % n] == i as isize
        {
            score += 1;
        }
    }

    let result = score;

    println!("{}", result);
}
