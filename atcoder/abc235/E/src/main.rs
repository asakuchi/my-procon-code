use petgraph::unionfind::UnionFind;
use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        mut abc: [(Usize1, Usize1, usize); m],
        mut uvw: [(Usize1, Usize1, usize); q],
    }

    // ------------------------------------

    let mut all_edges = Vec::new();

    for &(a, b, c) in &abc {
        all_edges.push((a, b, c, -1));
    }

    for i in 0..q {
        let (u, v, w) = uvw[i];
        all_edges.push((u, v, w, i as isize));
    }

    // 重みの昇順にソート
    all_edges.sort_by_key(|x| x.2);

    let mut set = UnionFind::new(n);

    let mut result = Vec::new();

    for &(a, b, _c, q_i) in &all_edges {
        if !set.equiv(a, b) {
            if q_i != -1 {
                result.push((q_i, String::from("Yes")));
            } else {
                // G の辺
                set.union(a, b);
            }
        } else {
            if q_i != -1 {
                result.push((q_i, String::from("No")));
            }
        }
    }

    result.sort();

    for (_i, res) in result.iter() {
        println!("{}", res);
    }
}
