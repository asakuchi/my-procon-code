use itertools::Itertools;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        x_y: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];
    let mut indegree = vec![0; n];

    for &(x, y) in &x_y {
        list[x].push(y);
        indegree[y] += 1;
    }

    let sorted = topological_sort(&list, &indegree, n);

    if sorted.len() != n {
        // これは入力に矛盾するAが存在するということで、
        // それは制約で入力されないということになっている
        println!("No");
        return;
    }

    // validate
    for i in 1..n {
        // ソートした時に隣あった要素同士にパスがなければならない
        if !list[sorted[i - 1]].contains(&sorted[i]) {
            println!("No");
            return;
        }
    }

    println!("Yes");

    let mut result = vec![0; n];
    let mut num = 1;

    for i in sorted {
        result[i] = num;
        num += 1;
    }

    let text = result.iter().join(" ");

    println!("{}", text);
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
