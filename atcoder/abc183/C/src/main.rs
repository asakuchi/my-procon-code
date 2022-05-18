use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        p: [[usize; n]; n],
    }

    let mut visited = vec![false; n];
    visited[0] = true;

    let result = rec(n, k, &p, 0, 0, &mut visited);

    println!("{}", result);
}

fn rec(
    n: usize,
    k: usize,
    p: &Vec<Vec<usize>>,
    current: usize,
    sum: usize,
    visited: &mut Vec<bool>,
) -> usize {
    if visited.iter().all(|&x| x) {
        return if sum + p[current][0] == k { 1 } else { 0 };
    }

    if sum > k {
        return 0;
    }

    let mut result = 0;

    for next in 0..n {
        if current == next {
            continue;
        }

        if visited[next] {
            continue;
        }

        visited[next] = true;

        result += rec(n, k, p, next, sum + p[current][next], visited);

        visited[next] = false;
    }

    result
}
