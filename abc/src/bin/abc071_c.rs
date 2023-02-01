use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut map = HashMap::new();

    for x in a {
        *map.entry(x).or_insert(0) += 1;
    }

    let mut list = Vec::new();

    for (x, count) in map {
        if count >= 4 {
            list.push(x);
            list.push(x);
        } else if count >= 2 {
            list.push(x);
        }
    }

    list.sort();

    // println!("{:?}", list);

    if list.len() < 2 {
        println!("0");
        return;
    }

    println!("{}", list.pop().unwrap() * list.pop().unwrap());
}
