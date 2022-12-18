use std::collections::HashSet;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        u_v: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    let mut uf = UnionFind::new(n);
    let mut size = vec![1; n];

    for &(u, v) in &u_v {
        list[u].push(v);
        list[v].push(u);

        if !uf.equiv(u, v) {
            let size_u = size[uf.find(u)];
            let size_v = size[uf.find(v)];

            uf.union(u, v);

            size[uf.find(u)] = size_u + size_v;
        }
    }

    let mut components = HashSet::new();

    for i in 0..n {
        components.insert(uf.find(i));
    }

    let mut color_list = vec![0; n];

    let mut result = n * (n - 1) / 2 - m;

    for &v in components.iter() {
        let mut color_1_count = 0;
        let mut color_2_count = 0;

        let is_bipartite = dfs(
            n,
            &list,
            v,
            1,
            &mut color_list,
            &mut color_1_count,
            &mut color_2_count,
        );

        if !is_bipartite {
            println!("0");
            return;
        }

        result -= color_1_count * (color_1_count - 1) / 2;
        result -= color_2_count * (color_2_count - 1) / 2;
    }

    println!("{}", result);
}

fn dfs(
    n: usize,
    list: &Vec<Vec<usize>>,
    current: usize,
    color: usize,
    color_list: &mut Vec<usize>,
    color_1_count: &mut usize,
    color_2_count: &mut usize,
) -> bool {
    color_list[current] = color;

    if color == 1 {
        *color_1_count += 1;
    } else {
        *color_2_count += 1;
    }

    let opposit_color = if color == 1 { 2 } else { 1 };

    for &next in list[current].iter() {
        if color_list[next] == color {
            // NG
            return false;
        }

        if color_list[next] != 0 {
            continue;
        }

        let check = dfs(
            n,
            list,
            next,
            opposit_color,
            color_list,
            color_1_count,
            color_2_count,
        );

        if !check {
            return false;
        }
    }

    true
}
