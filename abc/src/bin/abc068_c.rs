use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        a_b: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    for (a, b) in a_b {
        list[a].push(b);
        list[b].push(a);
    }

    if rec(n, &list, 0, 0, 0) {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}

fn rec(n: usize, list: &Vec<Vec<usize>>, current: usize, step: usize, prev: usize) -> bool {
    if current == n - 1 {
        return true;
    }

    if step >= 2 {
        return false;
    }

    for &next in list[current].iter() {
        if next == prev {
            continue;
        }

        if rec(n, list, next, step + 1, current) {
            return true;
        }
    }

    false
}
