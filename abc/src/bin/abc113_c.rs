use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
        p_y: [(usize, usize); m],
    }

    let mut map = HashMap::new();

    for i in 0..m {
        let (p, y) = p_y[i];

        let list = map.entry(p).or_insert(vec![]);
        list.push((y, i));
    }

    let mut result = Vec::new();

    for (p, list) in map.iter_mut() {
        list.sort();

        let mut x = 0;

        for &(_year, i) in list.iter() {
            x += 1;
            result.push((i, format!("{:<06}{:<06}", p, x)));
        }
    }

    result.sort();

    for (_, id) in result {
        println!("{}", id);
    }
}
