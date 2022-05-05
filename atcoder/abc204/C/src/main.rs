use proconio::fastout;
use proconio::input;
use proconio::marker::Usize1;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(Usize1, Usize1); m],
    }

    let mut list = vec![Vec::new(); n];

    for &(a, b) in ab.iter() {
        list[a].push(b);
    }

    let mut result = 0;

    for i in 0..n {
        let mut visited = vec![false; n];
        visited[i] = true;

        result += rec(n, i, &list, &mut visited);
    }

    println!("{}", result);
}

fn rec(n: usize, current: usize, list: &Vec<Vec<usize>>, visited: &mut Vec<bool>) -> usize {
    let mut result = 1;

    for &next in list[current].iter() {
        if visited[next] {
            continue;
        }

        visited[next] = true;
        result += rec(n, next, list, visited);
    }

    result
}
