// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String;n],
    }

    let mut list = vec![vec![false; n]; n];

    for (i, line) in s.iter().enumerate() {
        for (j, value) in line.chars().enumerate() {
            if value == 'Y' {
                list[i][j] = true;

                // 友人の友人 を探す
                let friends_friends = &s[j];

                for (k, value) in friends_friends.chars().enumerate() {
                    if i == k {
                        // 自分自身
                        continue;
                    }

                    if value == 'Y' {
                        list[i][k] = true;
                    }
                }
            }
        }
    }

    let mut counter = std::collections::HashMap::new();

    for (i, line) in list.iter().enumerate() {
        for &value in line {
            counter.entry(i).or_insert(0);

            if value {
                let count = counter.entry(i).or_insert(0);
                *count += 1;
            }
        }
    }

    let result = counter.values().max().unwrap();

    println!("{:?}", result);
}
