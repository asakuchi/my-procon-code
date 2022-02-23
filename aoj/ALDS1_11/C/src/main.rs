use std::io;

fn main() {
    // ------------------------------------
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n: usize = buf.parse().unwrap();

    // ------------------------------------
    // 二次元ベクタ

    let mut a: Vec<Vec<usize>> = vec![Vec::with_capacity(n); n];

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let mut iter = buf.split_whitespace();

        let u: usize = iter.next().unwrap().parse().unwrap();
        let _k: usize = iter.next().unwrap().parse().unwrap();

        let mut line: Vec<usize> = iter
            .map(|x| x.parse().unwrap())
            .map(|x: usize| x - 1)
            .collect();

        a[u - 1].append(&mut line);
    }

    // ------------------------------------

    let mut lengths = vec![-1; n];

    let mut visited = vec![false; n];

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0);

    lengths[0] = 0;
    visited[0] = true;

    while let Some(target) = queue.pop_front() {
        for &next in a[target].iter() {
            if visited[next] {
                continue;
            }

            visited[next] = true;
            lengths[next] = lengths[target] + 1;
            queue.push_back(next);
        }
    }

    for i in 0..n {
        println!("{} {}", i + 1, lengths[i]);
    }
}
