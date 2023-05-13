use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    }

    let mut map_s = HashMap::new();
    let mut map_t = HashMap::new();

    for i in 0..s.len() {
        *map_s.entry(s[i]).or_insert(0) += 1;
        *map_t.entry(t[i]).or_insert(0) += 1;
    }

    let atcoder = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];

    for x in 'a' as u8..='z' as u8 {
        let c = x as char;

        let count_s = map_s.entry(c).or_insert(0).clone();
        let count_t = map_t.entry(c).or_insert(0).clone();

        if count_s == count_t {
            // OK
            continue;
        }

        if !atcoder.contains(&c) {
            println!("No");
            return;
        }

        let count_s_at = map_s.entry('@').or_insert(0);
        let count_t_at = map_t.entry('@').or_insert(0);

        if count_s > count_t {
            if *count_t_at >= count_s - count_t {
                // OK
                *count_t_at -= count_s - count_t;
            } else {
                // NG
                println!("No");
                return;
            }
        } else {
            if *count_s_at >= count_t - count_s {
                // OK
                *count_s_at -= count_t - count_s;
            } else {
                // NG
                println!("No");
                return;
            }
        }
    }

    println!("Yes");
}
