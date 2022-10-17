use std::cmp::Reverse;

use itertools::Itertools;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        a_b: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    // 入次数
    let mut indegree = vec![0; n];

    for &(a, b) in &a_b {
        list[a].push(b);
        indegree[b] += 1;
    }

    let sorted = topological_sort(&list, &indegree, n);

    if sorted.len() == n {
        let text = sorted.iter().map(|x| x + 1).format(" ");
        println!("{}", text);
    } else {
        println!("-1");
    }
}

fn topological_sort(list: &Vec<Vec<usize>>, indegree: &Vec<usize>, v: usize) -> Vec<usize> {
    let mut sorted_vertices = Vec::new();

    // let mut queue = std::collections::VecDeque::new();
    let mut queue = std::collections::BinaryHeap::new();

    let mut indegree = indegree.clone();

    for i in 0..v {
        if indegree[i] == 0 {
            // queue.push_back(i);
            queue.push(Reverse(i));
        }
    }

    while let Some(Reverse(vertex)) = queue.pop() {
        for &next in &list[vertex] {
            indegree[next] -= 1;

            if indegree[next] == 0 {
                // queue.push_back(next);
                queue.push(Reverse(next));
            }
        }

        sorted_vertices.push(vertex);
    }

    sorted_vertices
}
