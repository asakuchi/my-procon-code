use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        _n: usize,
        m: usize,
    }

    let mut a = Vec::with_capacity(m);

    for _ in 0..m {
        input! {
            line: [usize],
        }

        a.push(line);
    }

    // -----------------------------------------------

    let mut tail_list = Vec::new();
    let mut num_map = HashMap::new();

    for i in 0..m {
        let tail = a[i].pop().unwrap();

        tail_list.push((tail, i));

        a[i].push(tail);
    }

    while let Some((tail, cylinder)) = tail_list.pop() {
        if let Some(&other_cylinder) = num_map.get(&tail) {
            a[cylinder].pop();
            a[other_cylinder as usize].pop();

            if let Some(next) = a[cylinder].pop() {
                tail_list.push((next, cylinder));
                a[cylinder].push(next);
            }

            if let Some(next) = a[other_cylinder as usize].pop() {
                tail_list.push((next, other_cylinder));
                a[other_cylinder].push(next);
            }

            num_map.remove(&tail);
        } else {
            num_map.insert(tail, cylinder);
        }
    }

    if num_map.len() == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
