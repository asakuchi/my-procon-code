use std::collections::HashMap;

use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a_b: [(String, String); n],
    }

    let mut uniq_num = HashMap::new();
    let mut max_num = 1usize;

    let mut uf = UnionFind::new(3 * n);

    for (a, b) in a_b {
        let a_num;

        if let Some(&value) = uniq_num.get(&a) {
            a_num = value;
        } else {
            a_num = max_num;
            uniq_num.insert(a, max_num);
            max_num += 1;
        }

        let b_num;

        if let Some(&value) = uniq_num.get(&b) {
            b_num = value;
        } else {
            b_num = max_num;
            uniq_num.insert(b, max_num);
            max_num += 1;
        }

        if uf.equiv(a_num, b_num) {
            println!("No");
            return;
        } else {
            uf.union(a_num, b_num);
        }
    }

    println!("Yes");
}
