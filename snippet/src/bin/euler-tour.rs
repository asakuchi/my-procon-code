//!
//! オイラーツアー
//!
//! https://maspypy.com/euler-tour-%E3%81%AE%E3%81%8A%E5%8B%89%E5%BC%B7
//!

use ac_library_rs::{Additive, Segtree};

fn main() {
    let n = 6;
    let m = 5;

    let u_v_w = vec![(1, 2, 1), (2, 3, 2), (3, 4, 4), (2, 5, 8), (1, 6, 16)];

    let mut list = vec![Vec::new(); n];

    for i in 0..m {
        let (u, v, w) = u_v_w[i];

        list[u - 1].push((v - 1, w));
        list[v - 1].push((u - 1, w));
    }

    println!("{:?}", list);

    let mut tour = EulerTour::new(n, m, &list);

    for i in 0..n {
        println!(
            "根から頂点{}までのパスの重み {}",
            i + 1,
            // path_query_tree.prod(0, in_list[i] + 1)
            tour.path_query(i),
        );
    }

    println!();

    tour.set_weight(3 - 1, 1000);

    for i in 0..n {
        println!(
            "根から頂点{}までのパスの重み {}",
            i + 1,
            // path_query_tree.prod(0, in_list[i] + 1)
            tour.path_query(i),
        );
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
