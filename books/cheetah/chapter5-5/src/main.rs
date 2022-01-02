// -*- coding:utf-8-unix -*-

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String;n],
    }

    let mut list = vec![vec![false; n]; n];

    for i in 0..n {
        for j in 0..n {
            let line = &s[i];
            let value = line.chars().nth(j).unwrap();

            if value == 'Y' {
                list[i][j] = true;

                // 友人の友人 を探す
                let friends_friends = &s[j];

                for k in 0..n {
                    if i == k {
                        // 自分自身
                        continue;
                    }

                    let value = friends_friends.chars().nth(k).unwrap();

                    if value == 'Y' {
                        list[i][k] = true;
                    }
                }
            }
        }
    }

    let mut counter = std::collections::HashMap::new();

    for i in 0..n {
        for j in 0..n {
            counter.entry(i).or_insert(0);

            if list[i][j] {
                let count = counter.entry(i).or_insert(0);
                *count += 1;
            }
        }
    }

    let result = counter.values().max().unwrap();

    println!("{:?}", result);
}
