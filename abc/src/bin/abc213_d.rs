use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a_b: [(Usize1, Usize1); n - 1],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in &a_b {
        list[a].push(b);
        list[b].push(a);
    }

    for i in 0..n {
        list[i].sort();
    }

    let mut visited = vec![false; n];
    visited[0] = true;

    let mut result = Vec::new();

    rec(n, &list, &mut visited, &mut result, 0);

    let text = result.iter().format(" ");

    println!("{}", text);
}

fn rec(
    n: usize,
    list: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    result: &mut Vec<usize>,
    current: usize,
) {
    result.push(current + 1);

    for &next in list[current].iter() {
        if visited[next] {
            continue;
        }

        visited[next] = true;

        rec(n, list, visited, result, next);

        result.push(current + 1);
    }
}
