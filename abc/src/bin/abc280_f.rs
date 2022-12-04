use std::collections::{HashMap, HashSet};

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        a_b_c: [(Usize1, Usize1, isize); m],
        x_y: [(Usize1, Usize1); q],
    }

    // 隣接リスト
    let mut list = vec![Vec::new(); n];

    for &(a, b, c) in &a_b_c {
        list[a].push((b, c));
        list[b].push((a, -c));
    }

    let mut uf = UnionFind::new(n);

    for &(a, b, _c) in &a_b_c {
        uf.union(a, b);
    }

    // 連結成分の代表
    let mut base_nodes = HashSet::new();

    for i in 0..n {
        base_nodes.insert(uf.find(i));
    }

    // inf になる連結成分の代表
    let mut inf_set = HashSet::new();

    // 連結成分の代表に対するコスト/高さ
    let mut hight_map = HashMap::new();

    for &base_node in base_nodes.iter() {
        rec(&list, base_node, 0, &mut hight_map, &mut inf_set, &uf);
    }

    for &(x, y) in &x_y {
        // 連結でないなら到達できない
        if !uf.equiv(x, y) {
            println!("nan");
            continue;
        }

        if inf_set.contains(&uf.find(x)) {
            println!("inf");
            continue;
        }

        if let Some(&y_cost) = hight_map.get(&y) {
            if let Some(&x_cost) = hight_map.get(&x) {
                println!("{}", y_cost - x_cost);
            } else {
                // ありえないはず
                // println!("nan");
                panic!();
            }
        } else {
            // ありえないはず
            // println!("nan");
            panic!();
        }
    }
}

fn rec(
    list: &Vec<Vec<(usize, isize)>>,
    current_node: usize,
    current_hight: isize,
    hight_map: &mut HashMap<usize, isize>,
    inf_set: &mut HashSet<usize>,
    uf: &UnionFind<usize>,
) {
    if let Some(&value) = hight_map.get(&current_node) {
        // 違うコストの経路があるならinf
        if value != current_hight {
            inf_set.insert(uf.find(current_node));
            return;
        } else {
            // もう探索済みなので続ける必要なし
            return;
        }
    } else {
        // コストを覚えておく
        hight_map.insert(current_node, current_hight);
    }

    for &(next, cost) in list[current_node].iter() {
        rec(list, next, current_hight + cost, hight_map, inf_set, uf);
    }
}
