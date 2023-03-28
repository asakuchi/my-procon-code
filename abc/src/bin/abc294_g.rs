use ac_library_rs::{Additive, Monoid, Segtree};
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        u_v_w: [(Usize1, Usize1, isize); n - 1],
        q: usize,
    }

    let m = n - 1;

    let mut list = vec![Vec::new(); n];
    let mut list_lca = vec![Vec::new(); n];
    let mut list_conv = vec![Vec::new(); n];

    for i in 0..m {
        let (u, v, w) = u_v_w[i];

        list[u].push((v, w));
        list[v].push((u, w));

        list_lca[u].push(v);
        list_lca[v].push(u);

        list_conv[u].push((v, w, i));
        list_conv[v].push((u, w, i));
    }

    let mut tour = EulerTour::new(n, m, &list);
    let lca = Lca::new(n, m, &list_lca);

    // 与えられた辺の番号iをEulerTour換算する
    let mut edge_conv = vec![0; n];

    let mut visited = vec![false; n];
    visited[0] = true;

    dfs(n, m, &list_conv, 0, &mut visited, &mut edge_conv);

    for _ in 0..q {
        input! {
            query: usize,
        }

        if query == 1 {
            input! {
                i: Usize1,
                w: isize,
            }

            let edge_num = edge_conv[i];

            tour.set_weight(edge_num, w);
        } else {
            input! {
                u: Usize1,
                v: Usize1,
            }

            let lca_vertex = lca.lca(u, v);

            println!(
                "{}",
                tour.path_query(u) + tour.path_query(v) - tour.path_query(lca_vertex) * 2
            )
        }
    }
}

fn dfs(
    n: usize,
    m: usize,
    list: &Vec<Vec<(usize, isize, usize)>>,
    current: usize,
    visited: &mut Vec<bool>,
    edge_conv: &mut Vec<usize>,
) {
    for &(next, _cost, edge_index) in list[current].iter() {
        if visited[next] {
            continue;
        }

        // 辺の番号変換テーブル
        edge_conv[edge_index] = next;

        visited[next] = true;

        dfs(n, m, list, next, visited, edge_conv);
    }
}

pub struct DepthIndexMin;

impl Monoid for DepthIndexMin {
    /// (depth, index)
    type S = (usize, usize);

    fn identity() -> Self::S {
        (std::usize::MAX, 0)
    }

    ///
    /// depthの小さい方を返す
    ///
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        std::cmp::min(*a, *b)
    }
}

struct Lca {
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

        let mut lca_tree = Segtree::<DepthIndexMin>::new(vs.len());

        for i in 0..vs.len() {
            lca_tree.set(i, (depth[i], vs[i]));
        }

        Lca { id, lca_tree }
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

        for &next in list[current_vertex].iter() {
            if visited[next] {
                continue;
            }

            visited[next] = true;

            Self::pre_lca(n, m, list, next, current_depth + 1, visited, vs, depth);

            vs.push(current_vertex);
            depth.push(current_depth);
        }
    }

    pub fn lca(&self, u: usize, v: usize) -> usize {
        // swap
        let (u, v) = if self.id[u] < self.id[v] {
            (u, v)
        } else {
            (v, u)
        };

        let lca_vertex = self.lca_tree.prod(self.id[u], self.id[v] + 1).1;

        lca_vertex
    }
}

#[derive(Debug)]
enum EdgeDirection {
    Up(usize),
    Down(usize),
}

struct EulerTour {
    path_query_tree: Segtree<Additive<isize>>,
    in_list: Vec<usize>,
    out_list: Vec<usize>,
}

impl EulerTour {
    fn new(n: usize, m: usize, list: &Vec<Vec<(usize, isize)>>) -> EulerTour {
        let mut visited = vec![false; n];
        visited[0] = true;

        let mut edge = Vec::new();
        let mut weight = Vec::new();

        edge.push(EdgeDirection::Down(0));
        weight.push(0);

        EulerTour::dfs(n, m, list, 0, &mut visited, &mut edge, &mut weight);

        edge.push(EdgeDirection::Up(0));
        weight.push(0);

        // println!("edge {:?}", edge);

        let mut in_list = vec![0; n];
        let mut out_list = vec![0; n];

        let mut visited = vec![false; n];

        for i in 0..edge.len() {
            match edge[i] {
                EdgeDirection::Down(edge_num) => {
                    if visited[edge_num] {
                        continue;
                    }

                    visited[edge_num] = true;

                    in_list[edge_num] = i;
                }
                EdgeDirection::Up(edge_num) => {
                    out_list[edge_num] = i;
                }
            }
        }

        // println!("in :{:?}", in_list);
        // println!("out: {:?}", out_list);

        let mut path_query_tree = Segtree::<Additive<isize>>::new(edge.len());

        for i in 0..edge.len() {
            path_query_tree.set(i, weight[i]);
        }

        EulerTour {
            path_query_tree,
            in_list,
            out_list,
        }
    }

    fn dfs(
        n: usize,
        m: usize,
        list: &Vec<Vec<(usize, isize)>>,
        current: usize,
        visited: &mut Vec<bool>,
        edge: &mut Vec<EdgeDirection>,
        weight: &mut Vec<isize>,
    ) {
        for &(next, cost) in list[current].iter() {
            if visited[next] {
                continue;
            }

            visited[next] = true;

            edge.push(EdgeDirection::Down(next));
            weight.push(cost);

            EulerTour::dfs(n, m, list, next, visited, edge, weight);

            edge.push(EdgeDirection::Up(next));
            weight.push(-cost);
        }
    }

    ///
    /// 根から頂点vまでのパスの重み
    ///
    fn path_query(&self, v: usize) -> isize {
        self.path_query_tree.prod(0, self.in_list[v] + 1)
    }

    fn set_weight(&mut self, v: usize, w: isize) {
        let target_in = self.in_list[v];
        let target_out = self.out_list[v];

        self.path_query_tree.set(target_in, w);
        self.path_query_tree.set(target_out, -w);
    }
}
