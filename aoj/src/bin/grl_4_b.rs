//!
//! Topological sort
//! トポロジカルソート
//!

use std::io;

fn main() {
    // ------------------------------------
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let v: usize = iter.next().unwrap().parse().unwrap();
    let e: usize = iter.next().unwrap().parse().unwrap();

    // ------------------------------------
    // タプルのベクタ

    let mut st: Vec<(usize, usize)> = Vec::with_capacity(e);

    for _ in 0..e {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let mut iter = buf.split_whitespace();

        let s: usize = iter.next().unwrap().parse().unwrap();
        let t: usize = iter.next().unwrap().parse().unwrap();

        st.push((s, t));
    }

    // ------------------------------------

    let mut list = vec![Vec::new(); v];

    // 入次数
    let mut indegree = vec![0; v];

    for &(s, t) in &st {
        list[s].push(t);
        indegree[t] += 1;
    }

    let sorted = topological_sort(&list, &indegree, v);

    for i in sorted {
        println!("{}", i);
    }
}

fn topological_sort(list: &Vec<Vec<usize>>, indegree: &Vec<usize>, v: usize) -> Vec<usize> {
    let mut sorted_vertices = Vec::new();

    let mut queue = std::collections::VecDeque::new();

    let mut indegree = indegree.clone();

    for i in 0..v {
        if indegree[i] == 0 {
            queue.push_back(i);
        }
    }

    while let Some(vertex) = queue.pop_front() {
        for &next in &list[vertex] {
            indegree[next] -= 1;

            if indegree[next] == 0 {
                queue.push_back(next);
            }
        }

        sorted_vertices.push(vertex);
    }

    sorted_vertices
}
