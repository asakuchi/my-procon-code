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

    let mut v: Vec<Vec<usize>> = vec![Vec::with_capacity(n); n];

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

        v[u - 1].append(&mut line);
    }

    // ------------------------------------

    // println!("{:?}", v);

    let mut d = vec![0; n];
    let mut f = vec![0; n];
    let mut visited = vec![false; n];

    let mut time = 1;

    for i in 0..n {
        if !visited[i] {
            visited[i] = true;
            rec(&v, &mut d, &mut f, &mut visited, i, &mut time);
        }
    }

    for i in 0..n {
        println!("{} {} {}", i + 1, d[i], f[i]);
    }
}

fn rec(
    v: &Vec<Vec<usize>>,
    d: &mut Vec<usize>,
    f: &mut Vec<usize>,
    visited: &mut Vec<bool>,
    current: usize,
    time: &mut usize,
) {
    // println!("find:{} time:{}", current + 1, *time);

    // d 頂点の発見時刻
    d[current] = *time;
    *time += 1;

    for &child in v[current].iter() {
        if !visited[child] {
            visited[child] = true;
            rec(v, d, f, visited, child, time);
        }
    }

    // println!("complete:{} time:{}", current + 1, *time);

    // f 頂点の完了時刻
    f[current] = *time;
    *time += 1;
}
