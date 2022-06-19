//!
//! 強連結成分分解（SCC）
//!

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];
    let mut reversed_list = vec![Vec::new(); n];

    for &(a, b) in &ab {
        list[a].push(b);
        reversed_list[b].push(a);
    }

    // 帰りがけ順の並び
    let mut vs = Vec::new();

    let mut visited = vec![false; n];
    // let mut topolo = vec![0; n];

    for v in 0..n {
        if !visited[v] {
            dfs(v, &list, &mut visited, &mut vs);
        }
    }

    visited = vec![false; n];

    let mut k = 0;
    let mut scc_list = Vec::new();

    // println!("vs:{:?}", vs);

    for &v in vs.iter().rev() {
        if !visited[v] {
            k += 1;

            let mut scc = Vec::new();

            rdfs(v, k, &reversed_list, &mut visited, &mut scc);

            scc_list.push(scc);
        }
    }

    // println!("{:?}", scc_list);
    // println!("{}", k);

    let mut result = 0;

    for scc in scc_list.iter() {
        let size = scc.len();

        result += size * (size - 1) / 2;
    }

    println!("{}", result);
}

fn dfs(v: usize, list: &Vec<Vec<usize>>, visited: &mut Vec<bool>, vs: &mut Vec<usize>) {
    visited[v] = true;

    for i in 0..list[v].len() {
        if !visited[list[v][i]] {
            dfs(list[v][i], list, visited, vs);
        }
    }
    vs.push(v);
}

fn rdfs(
    v: usize,
    k: usize,
    reversed_list: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    // topolo: &mut Vec<usize>,
    scc: &mut Vec<usize>,
) {
    visited[v] = true;
    // topolo[v] = k;
    scc.push(v);

    for i in 0..reversed_list[v].len() {
        if !visited[reversed_list[v][i]] {
            rdfs(reversed_list[v][i], k, reversed_list, visited, scc);
        }
    }
}
