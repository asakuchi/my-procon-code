use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        p: [[usize; w]; h],
    }

    let mut result = 0;

    for mask in 0..1 << h {
        let mut map = HashMap::new();

        // 1列が全て同じか確認
        'search_column: for j in 0..w {
            let mut prev = -1;
            let mut counter = 0;

            for i in 0..h {
                if mask & 1 << i == 0 {
                    continue;
                }

                if prev == -1 {
                    prev = p[i][j] as isize;
                } else if p[i][j] as isize != prev {
                    continue 'search_column;
                }

                counter += 1;
            }

            if counter == 0 {
                continue;
            }

            // 1列が全て同じだった
            *map.entry(prev).or_insert(0) += counter;
        }

        for (_key, &value) in map.iter() {
            result = result.max(value);
        }
    }

    println!("{}", result);
}
