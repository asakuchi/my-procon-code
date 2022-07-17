use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::BTreeSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
    }

    let mut ba: BTreeSet<usize> = BTreeSet::new();

    let mut uf = UnionFind::new(n);

    let mut size = vec![0; n];
    let mut turn_list = vec![0; n];

    for i in 0..n {
        let number = p[i];

        let mut next_num = 3_000_000;

        if let Some(next) = ba.range(number..).next() {
            next_num = next.clone();
        }

        if next_num != 3_000_000 {
            ba.remove(&next_num);
            ba.insert(number);

            let tmp = size[uf.find(next_num)];

            uf.union(next_num, number);
            size[uf.find(next_num)] = tmp + 1;
        } else {
            ba.insert(number);
            size[uf.find(number)] = 1;
        }

        if size[uf.find(number)] == k {
            ba.remove(&number);
            turn_list[uf.find(number)] = i + 1;
        }
    }

    let mut eaten_turn = vec![-1; n];

    for i in 0..n {
        if turn_list[uf.find(i)] != 0 {
            eaten_turn[i] = turn_list[uf.find(i)] as isize;
        }
    }

    for i in 0..n {
        println!("{}", eaten_turn[i]);
    }
}
