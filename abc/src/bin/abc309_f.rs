use std::collections::{BTreeMap, BTreeSet, HashMap};

use ac_library_rs::{Min, Segtree};

use proconio::input;

fn main() {
    input! {
        n: usize,
        h_w_d: [(usize, usize, usize); n],
    }

    // 座標圧縮
    let mut numbers = BTreeSet::new();

    let mut map = BTreeMap::new();

    for &(h, w, d) in &h_w_d {
        let mut line = vec![h, w, d];
        line.sort();

        map.entry(line[0])
            .or_insert(Vec::new())
            .push((line[1], line[2]));

        // 座標圧縮
        numbers.insert(line[1]);
    }

    // 座標圧縮
    let mut coordinate = HashMap::new();

    for (i, &x) in numbers.iter().enumerate() {
        coordinate.insert(x, i + 1);
    }

    let mut tree = Segtree::<Min<usize>>::new(n * 3 + 1);

    for (_a, bc) in map.iter() {
        for &(b, c) in bc.iter() {
            let cc_b = *coordinate.get(&b).unwrap();

            if tree.prod(0, cc_b) < c {
                println!("Yes");
                return;
            }
        }

        for &(b, c) in bc.iter() {
            let cc_b = *coordinate.get(&b).unwrap();

            tree.set(cc_b, c.min(tree.get(cc_b)));
        }
    }

    println!("No");
}
