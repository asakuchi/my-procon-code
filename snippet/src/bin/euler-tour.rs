use ac_library_rs::{Additive, Segtree};

#[derive(Debug)]
enum EdgeDirection {
    Up(usize),
    Down(usize),
}

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

    let mut visited = vec![false; n];
    visited[0] = true;

    let mut edge = Vec::new();
    let mut weight = Vec::new();

    edge.push(EdgeDirection::Down(0));
    weight.push(0);

    dfs(n, m, &list, 0, &mut visited, &mut edge, &mut weight);

    edge.push(EdgeDirection::Up(0));
    weight.push(0);

    println!("edge {:?}", edge);

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

    println!("in :{:?}", in_list);
    println!("out: {:?}", out_list);

    let mut path_query_tree = Segtree::<Additive<isize>>::new(edge.len());

    for i in 0..edge.len() {
        path_query_tree.set(i, weight[i]);
    }

    for i in 0..n {
        println!(
            "根から頂点{}までのパスの重み {}",
            i + 1,
            path_query_tree.prod(0, in_list[i] + 1)
        );
    }
}

fn dfs(
    n: usize,
    m: usize,
    list: &Vec<Vec<(usize, usize)>>,
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
        weight.push(cost as isize);

        dfs(n, m, list, next, visited, edge, weight);

        edge.push(EdgeDirection::Up(next));
        weight.push(-(cost as isize));
    }
}
