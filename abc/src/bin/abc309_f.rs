use std::{
    collections::{BTreeMap, BTreeSet, HashMap},
    hash::Hash,
};

use ac_library_rs::{Min, Segtree};

use proconio::input;

struct CoordinateCompression<T> {
    coordinate: HashMap<T, usize>,
}

impl<T> CoordinateCompression<T>
where
    T: Eq + Hash + Ord + Copy,
{
    fn new(list: &Vec<T>) -> Self {
        let mut numbers = BTreeSet::new();
        let mut coordinate = HashMap::new();

        for &x in list.iter() {
            numbers.insert(x);
        }

        for (i, &x) in numbers.iter().enumerate() {
            coordinate.insert(x, i + 1);
        }

        CoordinateCompression { coordinate }
    }

    fn get(&self, x: &T) -> Option<&usize> {
        self.coordinate.get(x)
    }
}

fn main() {
    input! {
        n: usize,
        h_w_d: [(usize, usize, usize); n],
    }

    let mut map = BTreeMap::new();

    let mut numbers = Vec::new();

    for &(h, w, d) in &h_w_d {
        let mut line = vec![h, w, d];
        line.sort();

        map.entry(line[0])
            .or_insert(Vec::new())
            .push((line[1], line[2]));

        // 座標圧縮
        numbers.push(line[1]);
    }

    // 座標圧縮
    let cc = CoordinateCompression::new(&numbers);

    let mut tree = Segtree::<Min<usize>>::new(n * 3 + 1);

    for (_a, bc) in map.iter() {
        for &(b, c) in bc.iter() {
            let &cc_b = cc.get(&b).unwrap();

            if tree.prod(0, cc_b) < c {
                println!("Yes");
                return;
            }
        }

        for &(b, c) in bc.iter() {
            let &cc_b = cc.get(&b).unwrap();

            tree.set(cc_b, c.min(tree.get(cc_b)));
        }
    }

    println!("No");
}
