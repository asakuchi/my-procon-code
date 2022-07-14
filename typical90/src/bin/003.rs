use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in ab.iter() {
        list[a].push(b);
        list[b].push(a);
    }

    let s = 0;

    let mut visited = vec![false; n];

    let (_, u) = dfs(n, &list, &mut visited, s);

    let mut visited = vec![false; n];

    let (cost, _) = dfs(n, &list, &mut visited, u);

    println!("{}", cost + 1);
}

fn dfs(
    n: usize,
    list: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    current: usize,
) -> (usize, usize) {
    let mut result = (0, current);

    visited[current] = true;

    for &next in list[current].iter() {
        if visited[next] {
            continue;
        }

        let (cost, v) = dfs(n, list, visited, next);

        result = result.max((cost + 1, v));
    }

    result
}
