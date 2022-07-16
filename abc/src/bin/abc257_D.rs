use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xyp: [(isize, isize, usize); n],
    }

    let mut ok: isize = 1_000_000_000_000;
    let mut ng = 0;

    while (ok - ng).abs() > 1 {
        let mid = (ok + ng) / 2;

        let mut solve = false;

        'search_start: for start in 0..n {
            let mut visited = vec![false; n];

            rec(&xyp, n, start, &mut visited, mid as usize);

            for i in 0..n {
                if !visited[i] {
                    continue 'search_start;
                }
            }

            solve = true;
            break;
        }

        if solve {
            ok = mid;
        } else {
            ng = mid;
        }
    }

    println!("{}", ok);
}

fn rec(xyp: &Vec<(isize, isize, usize)>, n: usize, v: usize, visited: &mut Vec<bool>, s: usize) {
    visited[v] = true;

    for next in 0..n {
        if v == next {
            continue;
        }

        if visited[next] {
            continue;
        }

        let len = (xyp[v].0 - xyp[next].0).abs() + (xyp[v].1 - xyp[next].1).abs();

        if xyp[v].2 * s >= len as usize {
            rec(xyp, n, next, visited, s);
        }
    }
}
