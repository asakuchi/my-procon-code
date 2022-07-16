use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut ab: [(Usize1, Usize1); n-1],
    }

    let mut edges = vec![Vec::new(); n];

    for i in 0..n - 1 {
        let (a, b) = ab[i];
        edges[a].push((b, i));
        edges[b].push((a, i));
    }

    let mut answer = vec![0; n];

    dfs(0, -1, 0, &edges, &mut answer);

    let mut k = 0;

    for i in 0..n {
        k = k.max(edges[i].len());
    }

    println!("{}", k);

    for i in 0..n - 1 {
        println!("{}", answer[i]);
    }
}

fn dfs(
    current: usize,
    parent: isize,
    color: usize,
    edges: &Vec<Vec<(usize, usize)>>,
    answer: &mut Vec<usize>,
) {
    let mut used = HashSet::new();

    used.insert(color);

    let mut c = 1;

    for to in edges[current].iter() {
        if to.0 as isize != parent {
            while used.contains(&c) {
                c += 1;
            }

            answer[to.1] = c;
            dfs(to.0, current as isize, c, edges, answer);
            c += 1;
        }
    }
}
