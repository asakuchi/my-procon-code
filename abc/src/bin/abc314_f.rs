//
// 解説AC
//

use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

use ac_library::ModInt998244353 as mint;

fn main() {
    input! {
        n: usize,
        p_q: [(Usize1, Usize1); n - 1],
    }

    let mut uf = UnionFind::new(n);

    let mut size = vec![1; n];
    let mut node: Vec<_> = (0..n).collect();

    let mut list = vec![Vec::new(); n];

    for &(p, q) in &p_q {
        let size_p = size[uf.find(p)];
        let size_q = size[uf.find(q)];

        let new_node = list.len();
        list.push(Vec::new());

        let p_cost = mint::from(size_p) / (mint::from(size_p) + mint::from(size_q));
        let q_cost = mint::from(size_q) / (mint::from(size_p) + mint::from(size_q));

        list[new_node].push((node[uf.find(p)], p_cost));
        list[new_node].push((node[uf.find(q)], q_cost));

        uf.union(p, q);

        size[uf.find(p)] = size_p + size_q;
        node[uf.find(p)] = new_node;
    }

    let mut result = vec![mint::from(0); n];

    rec(&list, list.len() - 1, mint::from(0), &mut result);

    let text = result.iter().format(" ");

    println!("{}", text);
}

fn rec(list: &Vec<Vec<(usize, mint)>>, current: usize, total: mint, result: &mut Vec<mint>) {
    if list[current].len() == 0 {
        // 末端
        result[current] = total;

        return;
    }

    for &(next, weight) in list[current].iter() {
        rec(list, next, total + weight, result);
    }
}
