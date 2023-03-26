//!
//! LCA (Lowest Common Ancestor)
//!
//! 蟻本
//!
//! オイラーツアー
//!

use ac_library_rs::{Monoid, Segtree};

fn main() {
    let n = 8;
    let m = 7;

    let u_v = vec![(1, 2), (1, 3), (2, 4), (2, 5), (5, 7), (5, 8), (3, 6)];

    let mut list = vec![Vec::new(); n];

    for &(u, v) in &u_v {
        list[u - 1].push(v - 1);
        list[v - 1].push(u - 1);
    }

    let lca = Lca::new(n, m, &list);

    println!("{} と {} の LCA {}", 4, 7, lca.lca(4 - 1, 7 - 1) + 1);
    println!("{} と {} の LCA {}", 8, 6, lca.lca(8 - 1, 6 - 1) + 1);
    println!("{} と {} の LCA {}", 6, 8, lca.lca(6 - 1, 8 - 1) + 1);
    println!("{} と {} の LCA {}", 5, 8, lca.lca(5 - 1, 8 - 1) + 1);
}

pub struct DepthIndexMin;

impl Monoid for DepthIndexMin {
    /// (depth, index)
    type S = (usize, usize);

    fn identity() -> Self::S {
        (usize::MAX, 0)
    }

    ///
    /// depthの小さい方を返す
    ///
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        std::cmp::min(*a, *b)
    }
}

struct Lca {
    // n: usize,
    // m: usize,
    // list: Vec<Vec<usize>>,
    vs: Vec<usize>,
    // depth: Vec<usize>,
    id: Vec<usize>,
    lca_tree: Segtree<DepthIndexMin>,
}

impl Lca {
    pub fn new(n: usize, m: usize, list: &Vec<Vec<usize>>) -> Lca {
        let mut visited = vec![false; n];
        // 根からのDFSでの訪問順に頂点を並べたもの
        let mut vs = Vec::new();
        let mut depth = Vec::new();

        visited[0] = true;

        Self::pre_lca(n, m, &list, 0, 0, &mut visited, &mut vs, &mut depth);

        // println!("vs   : {:?}", vs);
        // println!("depth: {:?}", depth);

        // 各頂点vに対し、最初にvsに現れるindex
        let mut id = vec![0; n];

        let mut visited = vec![false; n];

        for i in 0..vs.len() {
            let v = vs[i];

            if visited[v] {
                continue;
            }

            visited[v] = true;

            id[v] = i;
        }

        // println!("id   : {:?}", id);

        let mut lca_tree = Segtree::<DepthIndexMin>::new(vs.len());

        for i in 0..vs.len() {
            lca_tree.set(i, (depth[i], i));
        }

        Lca {
            // n,
            // m,
            // list: list.clone(),
            vs,
            // depth,
            id,
            lca_tree,
        }
    }

    fn pre_lca(
        n: usize,
        m: usize,
        list: &Vec<Vec<usize>>,
        current_vertex: usize,
        current_depth: usize,
        visited: &mut Vec<bool>,
        vs: &mut Vec<usize>,
        depth: &mut Vec<usize>,
    ) {
        vs.push(current_vertex);
        depth.push(current_depth);

        // println!("in current: {} depth: {}", current_vertex, current_depth);

        for &next in list[current_vertex].iter() {
            if visited[next] {
                continue;
            }

            visited[next] = true;

            Self::pre_lca(n, m, list, next, current_depth + 1, visited, vs, depth);

            vs.push(current_vertex);
            depth.push(current_depth);

            // println!("back current: {} depth: {}", current_vertex, current_depth);
        }
    }

    pub fn lca(
        &self,
        // vs: &Vec<usize>,
        // id: &Vec<usize>,
        // lca_tree: &Segtree<DepthIndexMin>,
        u: usize,
        v: usize,
    ) -> usize {
        // swap
        let (u, v) = if self.id[u] < self.id[v] {
            (u, v)
        } else {
            (v, u)
        };

        let index = self.lca_tree.prod(self.id[u], self.id[v] + 1).1;

        self.vs[index]
    }
}
