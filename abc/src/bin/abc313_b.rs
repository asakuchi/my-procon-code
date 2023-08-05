use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a_b: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in &a_b {
        list[a].push(b);
    }

    for i in 0..n {
        let mut visited = vec![false; n];

        visited[i] = true;
        rec(n, m, &list, i, &mut visited);

        let mut ok = true;

        for j in 0..n {
            if !visited[j] {
                ok = false;
                break;
            }
        }

        if ok {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}

fn rec(n: usize, m: usize, list: &Vec<Vec<usize>>, current: usize, visited: &mut Vec<bool>) {
    for &next in list[current].iter() {
        if visited[next] {
            continue;
        }

        visited[next] = true;
        rec(n, m, list, next, visited);
    }
}
