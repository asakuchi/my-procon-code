use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Usize1; m],
    }

    let mut list = vec![Vec::new(); n];

    for x in a {
        list[x].push(x + 1);
        list[x + 1].push(x);
    }

    let mut result = Vec::new();

    let mut visited = vec![false; n];

    for i in 0..n {
        if visited[i] {
            continue;
        }

        rec(n, m, &mut result, &mut visited, &list, i);
    }

    let text = result.iter().map(|x| x + 1).format(" ");

    println!("{}", text);
}

fn rec(
    n: usize,
    m: usize,
    result: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    list: &Vec<Vec<usize>>,
    current: usize,
) {
    visited[current] = true;

    for &next in list[current].iter() {
        if visited[next] {
            continue;
        }

        rec(n, m, result, visited, list, next);
    }

    result.push(current);
}
