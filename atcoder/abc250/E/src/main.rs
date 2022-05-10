use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashMap;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        q: usize,
        xy: [(Usize1, Usize1); q],
    }

    let mut map = HashMap::new();

    for &num in a.iter() {
        let order = map.len() + 1;
        map.entry(num).or_insert(order);
    }

    for &num in b.iter() {
        let order = map.len() + 1_000_001;
        map.entry(num).or_insert(order);
    }

    let a: Vec<_> = a.iter().map(|num| *map.get(num).unwrap()).collect();
    let b: Vec<_> = b.iter().map(|num| *map.get(num).unwrap()).collect();

    let mut a_kinds = vec![0; n];
    let mut set = HashSet::new();

    for i in 0..n {
        set.insert(a[i]);
        a_kinds[i] = set.len();
    }

    let mut b_kinds_and_max = vec![(0, 0); n];
    let mut set = HashSet::new();
    let mut max_value = 0;

    for i in 0..n {
        let value = b[i];

        set.insert(value);
        max_value = max_value.max(value);

        b_kinds_and_max[i] = (set.len(), max_value);
    }

    for &(x, y) in xy.iter() {
        if a_kinds[x] == b_kinds_and_max[y].0 && a_kinds[x] == b_kinds_and_max[y].1 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
