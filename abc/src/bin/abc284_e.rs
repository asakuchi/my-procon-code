use proconio::{input, marker::Usize1};

const MAX_PATH: usize = 1_000_000;

fn main() {
    input! {
        n: usize,
        m: usize,
        u_v: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    for &(u, v) in &u_v {
        list[u].push(v);
        list[v].push(u);
    }

    let mut visited = vec![false; n];

    let result = rec(n, m, &list, 0, &mut visited);

    if result > MAX_PATH {
        println!("{}", MAX_PATH);
    } else {
        println!("{}", result);
    }
}

fn rec(
    n: usize,
    m: usize,
    list: &Vec<Vec<usize>>,
    current: usize,
    visited: &mut Vec<bool>,
) -> usize {
    let mut result = 1;

    for &next in list[current].iter() {
        if visited[next] {
            continue;
        }

        visited[current] = true;

        let next_result = rec(n, m, list, next, visited);

        result += next_result;

        if result > MAX_PATH {
            return MAX_PATH;
        }

        visited[current] = false;
    }

    result
}
