use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        d: usize,
        k: usize,
        mut u_v_w: [(Usize1, Usize1, usize); m],
        _x_y: [(usize, usize); n],
    }

    // 最小全域木
    let mut min_spanning_tree = Vec::new();
    // その他
    let mut others = Vec::new();

    // 重みの昇順にソート
    u_v_w.sort_by_key(|x| x.2);

    let mut set = UnionFind::new(n);

    for i in 0..m {
        let (u, v, _w) = u_v_w[i];

        if !set.equiv(u, v) {
            min_spanning_tree.push(i);
            set.union(u, v);
        } else {
            others.push(i);
        }
    }

    // eprintln!("mst {:?}", min_spanning_tree);
    eprintln!("mst {}", min_spanning_tree.len());

    // 各日に工事する道路
    let mut plan_day = vec![Vec::new(); d];

    let mut days = (0..d).cycle();

    // 最小全域木を均等に割り振る
    for &load in min_spanning_tree.iter() {
        let day = days.next().unwrap();

        plan_day[day].push(load);
    }

    // その他も均等に割り振る
    for &load in others.iter() {
        let day = days.next().unwrap();

        plan_day[day].push(load);
    }

    let mut result = vec![0; m];

    for day in 0..d {
        for &load in &plan_day[day] {
            result[load] = day + 1;
        }
    }

    let text = &result[..m].iter().format(" ");

    println!("{}", text);
}
