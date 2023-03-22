use ac_library_rs::{Additive, Monoid, Segtree};

// struct DepthIndex(usize, usize);

pub struct DepthIndexMin;

impl Monoid for DepthIndexMin {
    // type S = S;
    type S = (usize, usize);

    fn identity() -> Self::S {
        // S::max_value()
        (usize::MAX, 0)
    }
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        std::cmp::min(*a, *b)
    }
}

fn main() {
    let n = 6;
    let m = 5;

    let u_v_w = vec![(1, 2, 1), (2, 3, 2), (3, 4, 4), (2, 5, 8), (1, 6, 16)];

    let mut list = vec![Vec::new(); n + 1];

    for &(u, v, w) in &u_v_w {
        list[u].push((v, w));
        list[v].push((u, w));
    }

    println!("{:?}", list);

    let mut visited = vec![false; n + 1];
    visited[1] = true;

    let mut edge = Vec::new();

    edge.push((1, 0, 0));

    dfs(n, m, &list, 1, &mut visited, &mut edge, 0, 0);

    edge.push((-1, 0, 0));

    println!("edge {:?}", edge);

    let mut in_list = vec![0; n + 1];
    let mut out_list = vec![0; n + 1];

    for i in 0..edge.len() {
        if edge[i].0 > 0 {
            in_list[edge[i].0 as usize] = i;
        } else {
            out_list[(-edge[i].0) as usize] = i;
        }
    }

    println!("in :{:?}", in_list);
    println!("out: {:?}", out_list);

    let mut path_query_tree = Segtree::<Additive<isize>>::new(edge.len());

    let mut lca_tree = Segtree::<DepthIndexMin>::new(edge.len());

    for i in 0..edge.len() {
        path_query_tree.set(i, edge[i].1);
        lca_tree.set(i, (edge[i].2, i));
    }

    println!(
        "根から頂点5までのパスの重み {}",
        path_query_tree.prod(0, in_list[5] + 1)
    );

    println!(
        "根から頂点3までのパスの重み {}",
        path_query_tree.prod(0, in_list[3] + 1)
    );

    let index = lca_tree.prod(in_list[3], out_list[5] + 1).1;

    println!("頂点3と頂点5のLCA {:?}", edge[index].0.abs());
    // これの始点がLCA
    // TODO: 辺の始点を求める必要あり

    println!(
        "頂点3から頂点5までのパスの重み {}",
        path_query_tree.prod(out_list[3], in_list[5] + 1)
    );
}

fn dfs(
    n: usize,
    m: usize,
    list: &Vec<Vec<(usize, usize)>>,
    current: usize,
    visited: &mut Vec<bool>,
    edge: &mut Vec<(isize, isize, usize)>,
    total_cost: usize,
    depth: usize,
) {
    // edge.push((current as isize, total_cost));

    for &(next, cost) in list[current].iter() {
        if visited[next] {
            continue;
        }

        visited[next] = true;

        edge.push((next as isize, cost as isize, depth + 1));

        dfs(
            n,
            m,
            list,
            next,
            visited,
            edge,
            total_cost + cost,
            depth + 1,
        );

        edge.push((-(next as isize), -(cost as isize), depth));
    }

    // edge.push((-(current as isize), total_cost));
}
