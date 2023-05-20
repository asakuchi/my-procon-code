use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut s: [Chars; n],
    }

    let mut list = vec![Vec::new(); n];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            let mut diff = 0;

            for k in 0..m {
                if s[i][k] != s[j][k] {
                    diff += 1;
                }
            }

            if diff == 1 {
                list[i].push(j);
                list[j].push(i);
            }
        }
    }

    for i in 0..n {
        let mut visited = vec![false; n];

        visited[i] = true;

        rec(n, &list, i, &mut visited);

        let mut ok = true;

        for j in 0..n {
            if !visited[j] {
                ok = false;
                break;
            }
        }

        if ok {
            println!("Yes");
            return;
        }
    }

    println!("No");
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
