use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        mut ab: [(usize, usize); n-1],
        mut px: [(usize, u128); q],
    }

    let ab: Vec<(usize, usize)> = ab.iter().map(|(a, b)| (a - 1, b - 1)).collect();

    let mut edges = vec![Vec::with_capacity(n); n];

    for &(a, b) in ab.iter() {
        edges[a].push(b);
        edges[b].push(a);
    }

    let mut counter = vec![0 as u128; n];

    for &(p, x) in px.iter() {
        counter[p - 1] += x;
    }

    let mut visited = vec![false; n];

    let mut stack = Vec::new();
    stack.push(0);
    visited[0] = true;

    while let Some(current) = stack.pop() {
        for &child in edges[current].iter() {
            if visited[child] {
                continue;
            }

            visited[child] = true;
            counter[child] += counter[current];
            stack.push(child);
        }
    }

    for i in 0..n {
        if i != 0 {
            print!(" ");
        }
        print!("{}", counter[i]);
    }
    println!();
}

// fn rec(edges: &Vec<Vec<usize>>, counter: &mut Vec<u128>, current: usize, parrent: usize) {
//     for &child in edges[current].iter() {
//         if child != parrent {
//             counter[child] += counter[current];
//             rec(edges, counter, child, current);
//         }
//     }
// }
