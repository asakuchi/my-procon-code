use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(Usize1, Usize1); m],
        q: usize,
        mut xk: [(Usize1, usize); q],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in ab.iter() {
        list[a].push(b);
        list[b].push(a);
    }

    for &(x, k) in xk.iter() {
        let mut queue = VecDeque::new();
        queue.push_back((x, 0));

        let mut visited = vec![false; n];
        let mut result = 0;

        while let Some((current, length)) = queue.pop_front() {
            if length > k {
                continue;
            }

            if visited[current] {
                continue;
            }

            result += current + 1;
            visited[current] = true;

            for &next in list[current].iter() {
                queue.push_back((next, length + 1));
            }
        }

        println!("{}", result);
    }
}
