use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        u_v: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    for &(u, v) in &u_v {
        list[u].push(v);
    }

    let mut pairs = 0;

    for i in 0..n {
        let mut visited = vec![false; n];

        visited[i] = true;
        pairs += rec(n, &mut list, i, &mut visited);
    }

    println!("{}", pairs - m);
}

fn rec(n: usize, list: &Vec<Vec<usize>>, current: usize, visited: &mut Vec<bool>) -> usize {
    let mut result = 0;

    for &next in list[current].iter() {
        if visited[next] {
            continue;
        }

        visited[next] = true;

        result += rec(n, list, next, visited);
        result += 1;
    }

    result
}
