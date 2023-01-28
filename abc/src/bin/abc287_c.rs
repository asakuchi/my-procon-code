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
        list[v].push(u);
    }

    let mut start_goal = Vec::new();

    for i in 0..n {
        if list[i].len() == 1 {
            start_goal.push(i);
        } else if list[i].len() != 2 {
            println!("No");
            return;
        }
    }

    if start_goal.len() != 2 {
        println!("No");
        return;
    }

    let mut visited = vec![false; n];

    rec(n, &list, start_goal[0], &mut visited);

    for i in 0..n {
        if !visited[i] {
            println!("No");
            return;
        }
    }

    println!("Yes");
}

fn rec(n: usize, list: &Vec<Vec<usize>>, current: usize, visited: &mut Vec<bool>) {
    for &next in list[current].iter() {
        if visited[next] {
            continue;
        }

        visited[next] = true;

        rec(n, list, next, visited);
    }
}
