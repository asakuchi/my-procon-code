use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: (isize, isize),
        t: (isize, isize),
        xyr: [(isize, isize, isize); n],
    }

    let mut start = n + 1;
    let mut goal = n + 1;

    for i in 0..n {
        let (x, y, r) = xyr[i];

        if start == n + 1 {
            if (x - s.0).pow(2) + (y - s.1).pow(2) == r.pow(2) {
                start = i;
            }
        }

        if goal == n + 1 {
            if (x - t.0).pow(2) + (y - t.1).pow(2) == r.pow(2) {
                goal = i;
            }
        }

        if start != n + 1 && goal != n + 1 {
            break;
        }
    }

    let mut list = vec![Vec::new(); n];

    for i in 0..n {
        let p1 = xyr[i];

        for j in i + 1..n {
            let p2 = xyr[j];

            if (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) <= (p1.2 + p2.2).pow(2)
                && (p1.0 - p2.0).pow(2) + (p1.1 - p2.1).pow(2) >= (p1.2 - p2.2).pow(2)
            {
                list[i].push(j);
                list[j].push(i);
            }
        }
    }

    let mut visited = vec![false; n];

    let result = dfs(&list, start, goal, &mut visited);

    if result {
        println!("Yes");
    } else {
        println!("No");
    }
}

fn dfs(list: &Vec<Vec<usize>>, current: usize, goal: usize, visited: &mut Vec<bool>) -> bool {
    if current == goal {
        return true;
    }

    visited[current] = true;

    for &next in list[current].iter() {
        if visited[next] {
            continue;
        }

        if dfs(list, next, goal, visited) {
            return true;
        }
    }

    false
}
