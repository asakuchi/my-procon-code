use ac_library_rs::SccGraph;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        u_v: [(Usize1, Usize1); m],
    }

    let mut cycle = vec![false; n];

    let mut reverse_list = vec![Vec::new(); n];

    let mut g = SccGraph::new(n);

    for &(u, v) in &u_v {
        g.add_edge(u, v);

        reverse_list[v].push(u);
    }

    let scc = g.scc();

    for com in scc.iter() {
        if com.len() >= 2 {
            for &v in com {
                cycle[v] = true;
            }
        }
    }

    let mut visited = vec![false; n];

    let mut result = 0;

    for i in 0..n {
        if !cycle[i] {
            continue;
        }

        if visited[i] {
            continue;
        }

        visited[i] = true;

        rec(&reverse_list, i, &mut visited, &mut result);
    }

    println!("{}", result);
}

fn rec(
    reverse_list: &Vec<Vec<usize>>,
    current: usize,
    visited: &mut Vec<bool>,
    result: &mut usize,
) {
    *result += 1;

    for &next in reverse_list[current].iter() {
        if visited[next] {
            continue;
        }

        visited[next] = true;

        rec(reverse_list, next, visited, result);
    }
}
