use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        v: [usize; n],
    }

    let mut even_map = HashMap::new();

    for i in (0..n).step_by(2) {
        // println!("{}", i);

        *even_map.entry(v[i]).or_insert(0) += 1;
    }

    let mut odd_map = HashMap::new();

    for i in (1..n).step_by(2) {
        // println!("{}", i);

        *odd_map.entry(v[i]).or_insert(0) += 1;
    }

    let mut even_list: Vec<_> = even_map.into_iter().collect();
    let mut odd_list: Vec<_> = odd_map.into_iter().collect();

    even_list.sort_by_key(|x| x.1);
    odd_list.sort_by_key(|x| x.1);

    let odd_max = odd_list.pop().unwrap();
    let even_max = even_list.pop().unwrap();

    if odd_max.0 != even_max.0 {
        println!("{}", n - odd_max.1 - even_max.1);
        return;
    }

    if let Some(second_odd_max) = odd_list.pop() {
        if let Some(second_even_max) = even_list.pop() {
            let candi_1 = n - odd_max.1 - second_even_max.1;
            let candi_2 = n - even_max.1 - second_odd_max.1;

            println!("{}", candi_1.min(candi_2));
            return;
        } else {
            // evenは全て同じ
            println!("{}", n / 2 - second_odd_max.1);
            return;
        }
    } else {
        // oddは全て同じ
        if let Some(second_even_max) = even_list.pop() {
            println!("{}", n / 2 - second_even_max.1);
            return;
        } else {
            // odd, evenは全て同じ
            println!("{}", n / 2);
            return;
        }
    }
}
