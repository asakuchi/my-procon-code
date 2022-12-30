use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    if s.len() == 1 {
        if s[0] == '8' {
            println!("Yes");
        } else {
            println!("No");
        }
        return;
    }

    let mut map = HashMap::new();

    for &c in s.iter() {
        *map.entry(c).or_insert(0) += 1;
    }

    for mul_8 in (16..1000).step_by(8) {
        if mul_8 < 100 && s.len() > 2 {
            continue;
        }

        let mut map_2 = HashMap::new();

        for c in mul_8.to_string().chars() {
            *map_2.entry(c).or_insert(0) += 1;
        }

        let mut ok = true;

        for (c, count) in map_2 {
            if let Some(&s_count) = map.get(&c) {
                if s_count < count {
                    ok = false;
                }
            } else {
                ok = false;
            }
        }

        if ok {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
