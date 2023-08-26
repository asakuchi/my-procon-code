use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a_b_c: [(Usize1, Usize1, usize); m],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b, c) in &a_b_c {
        list[a].push((b, c));
        list[b].push((a, c));
    }

    let mut result = 0;

    for i in 0..n {
        let mut visited = vec![false; n];
        visited[i] = true;

        let score_1 = rec(n, &list, i, &mut visited);

        result = result.max(score_1);
    }

    println!("{}", result);
}

fn rec(
    n: usize,
    list: &Vec<Vec<(usize, usize)>>,
    current: usize,
    visited: &mut Vec<bool>,
) -> usize {
    let mut result = 0;

    for &(next, c) in list[current].iter() {
        if visited[next] {
            continue;
        }

        visited[next] = true;

        let score_1 = rec(n, list, next, visited) + c;

        visited[next] = false;

        result = result.max(score_1);
    }

    result
}
