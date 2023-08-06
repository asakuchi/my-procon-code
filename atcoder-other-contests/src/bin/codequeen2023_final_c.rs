use std::collections::HashSet;

use proconio::{input, marker::Usize1};

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        s: Usize1,
        t: Usize1,
        u_v: [(Usize1, Usize1); n - 1],
    }

    let mut list = vec![Vec::new(); n];

    for &(u, v) in &u_v {
        list[u].push(v);
        list[v].push(u);
    }

    let mut path = HashSet::new();

    // s と t の最短経路上の頂点を探す
    rec_1(n, s, t, &list, s, s, &mut path);

    let mut result = vec![0; n];

    rec_2(n, s, t, &list, s, s, &path, 1, &mut result);

    for i in 0..n {
        println!("{}", result[i]);
    }
}

///
/// 最短経路ならtrue
///
fn rec_1(
    n: usize,
    s: usize,
    t: usize,
    list: &Vec<Vec<usize>>,
    current: usize,
    parent: usize,
    path: &mut HashSet<usize>,
) -> bool {
    if current == t {
        path.insert(current);
        return true;
    }

    for &next in list[current].iter() {
        if next == parent {
            continue;
        }

        if rec_1(n, s, t, list, next, current, path) {
            path.insert(current);
            return true;
        }
    }

    false
}

fn rec_2(
    n: usize,
    s: usize,
    t: usize,
    list: &Vec<Vec<usize>>,
    current: usize,
    parent: usize,
    path: &HashSet<usize>,
    weight: usize,
    result: &mut Vec<usize>,
) {
    result[current] = weight;

    if path.contains(&current) {
        result[current] = 1;
    } else {
        result[current] = weight;
    }

    for &next in list[current].iter() {
        if next == parent {
            continue;
        }

        rec_2(
            n,
            s,
            t,
            list,
            next,
            current,
            path,
            result[current] + 1,
            result,
        );
    }
}
