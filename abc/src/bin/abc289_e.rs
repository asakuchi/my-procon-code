use std::collections::{HashSet, VecDeque};

use proconio::fastout;
use proconio::{input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        t: usize,
    }

    't_loop: for _ in 0..t {
        input! {
            n: usize,
            m: usize,
            c: [usize; n],
            u_v: [(Usize1, Usize1); m],
        }

        let mut list = vec![Vec::new(); n];

        for &(u, v) in &u_v {
            list[u].push(v);
            list[v].push(u);
        }

        let mut visited = HashSet::new();

        let mut queue = VecDeque::new();
        queue.push_back((0, 0, n - 1));
        visited.insert((0, n - 1));

        while let Some((step, taka, aoki)) = queue.pop_front() {
            // 終了条件
            if taka == n - 1 && aoki == 0 {
                println!("{}", step);
                continue 't_loop;
            }

            for &next_taka in list[taka].iter() {
                for &next_aoki in list[aoki].iter() {
                    if c[next_taka] == c[next_aoki] {
                        continue;
                    }

                    if visited.contains(&(next_taka, next_aoki)) {
                        continue;
                    }

                    // 次へ
                    visited.insert((next_taka, next_aoki));
                    queue.push_back((step + 1, next_taka, next_aoki));
                }
            }
        }

        println!("-1");
    }
}
