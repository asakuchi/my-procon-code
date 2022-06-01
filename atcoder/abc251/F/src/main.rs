use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut vu: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    for &(v, u) in &vu {
        list[v].push(u);
        list[u].push(v);
    }

    let mut t1 = Vec::new();

    let mut visited = vec![false; n];
    visited[0] = true;

    dfs(n, &list, 0, &mut visited, &mut t1);

    let mut t2 = Vec::new();

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0);

    let mut visited = vec![false; n];
    visited[0] = true;

    while let Some(current) = queue.pop_front() {
        for &next in &list[current] {
            if visited[next] {
                continue;
            }

            visited[next] = true;
            t2.push((current, next));

            queue.push_back(next);
        }
    }

    for &(u, v) in &t1 {
        println!("{} {}", u + 1, v + 1);
    }

    for &(u, v) in &t2 {
        println!("{} {}", u + 1, v + 1);
    }
}

fn dfs(
    n: usize,
    list: &Vec<Vec<usize>>,
    current: usize,
    visited: &mut Vec<bool>,
    t1: &mut Vec<(usize, usize)>,
) {
    for &next in &list[current] {
        if visited[next] {
            continue;
        }

        visited[next] = true;
        t1.push((current, next));

        dfs(n, list, next, visited, t1);
    }
}
