use std::collections::HashSet;

use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x_y: [(isize, isize); n],
    }

    let mut uf = UnionFind::new(n);

    let direction = vec![(-1, -1), (-1, 0), (0, -1), (0, 1), (1, 0), (1, 1)];

    for i in 0..n {
        let (x_i, y_i) = x_y[i];

        for j in i + 1..n {
            let (x_j, y_j) = x_y[j];

            for direction in direction.iter() {
                if x_i + direction.0 == x_j && y_i + direction.1 == y_j {
                    if !uf.equiv(i, j) {
                        uf.union(i, j);
                    }
                }
            }
        }
    }

    let mut set = HashSet::new();

    for i in 0..n {
        set.insert(uf.find(i));
    }

    println!("{}", set.len());
}
