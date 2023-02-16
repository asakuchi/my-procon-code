use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize; n],
        y: [usize; n],
        a_b_z: [(Usize1, Usize1, usize); m],
    }

    // n 個の島と仮想的に「空港が使用可能」、「港が使用可能」の頂点を考える
    let mut airport_edge = Vec::new();
    let mut port_edge = Vec::new();

    for i in 0..n {
        let airport_cost = x[i];
        airport_edge.push((i, n, airport_cost));

        let port_cost = y[i];
        port_edge.push((i, n + 1, port_cost));
    }

    let mut result = 1_000_000_000_000_000;

    for (has_airport, has_port) in vec![(false, false), (true, false), (false, true), (true, true)]
    {
        // 最小全域木
        let mut edge = a_b_z.clone();

        if has_airport {
            for &e in &airport_edge {
                edge.push(e);
            }
        }

        if has_port {
            for &e in &port_edge {
                edge.push(e);
            }
        }

        edge.sort_by_key(|x| x.2);

        let mut uf = UnionFind::new(n + 2);

        let mut total_cost = 0;

        for (u, v, cost) in edge {
            if !uf.equiv(u, v) {
                uf.union(u, v);
                total_cost += cost;
            }
        }

        // n個の頂点が連結でないならNG
        for i in 0..n {
            if !uf.equiv(0, i) {
                total_cost = 1_000_000_000_000_000;
                break;
            }
        }

        result = result.min(total_cost);
    }

    println!("{}", result);
}
