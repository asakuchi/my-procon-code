use std::collections::HashSet;

use im_rc::HashMap;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    let mut count_map = HashMap::new();

    for i in 0..n {
        *count_map.entry(a[i]).or_insert(0) += 1;
    }

    a.sort();
    a.dedup();

    let original_n = n;
    let n = a.len();

    // ----------------------------------

    // 全て同じ
    let mut all_same = true;

    for i in 0..n {
        if a[i] != a[0] {
            all_same = false;
            break;
        }
    }

    if all_same {
        println!("0");
        return;
    }

    // -----------------------------------

    let mut candi = HashSet::new();

    let mut result = 0usize;

    let mut min_count_2 = None;
    let mut min_count_3 = None;

    for i in 0..n {
        let mut current = a[i];

        let mut count = 0;

        let mut count_2 = 0;
        let mut count_3 = 0;

        for j in vec![2, 3] {
            while current % j == 0 {
                // list.push(j);
                count += 1;

                if j == 2 {
                    count_2 += 1;
                } else {
                    count_3 += 1;
                }

                // println!("{} / {}", current, j);

                current /= j;
            }
        }

        candi.insert(current);

        if candi.len() > 1 {
            println!("-1");
            return;
        }

        // 全項を割れる2の数
        if let Some(prev_min_count_2) = min_count_2 {
            if count_2 < prev_min_count_2 {
                min_count_2 = Some(count_2);
            }
        } else {
            min_count_2 = Some(count_2);
        }

        // 3
        if let Some(prev_min_count_3) = min_count_3 {
            if count_3 < prev_min_count_3 {
                min_count_3 = Some(count_3);
            }
        } else {
            min_count_3 = Some(count_3);
        }

        result += count * *count_map.entry(a[i]).or_insert(0);
    }

    // 全項を割れる2の数
    if let Some(min_count_2) = min_count_2 {
        result -= original_n * min_count_2;
    }

    if let Some(min_count_3) = min_count_3 {
        result -= original_n * min_count_3;
    }

    println!("{}", result);
}
