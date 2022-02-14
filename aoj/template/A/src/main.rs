use std::io;

fn main() {
    // ------------------------------------
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n: usize = buf.parse().unwrap();

    // ------------------------------------
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // ------------------------------------
    // 二次元ベクタ

    let mut a: Vec<Vec<isize>> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let iter = buf.split_whitespace();

        let line: Vec<isize> = iter.map(|x| x.parse().unwrap()).collect();

        a.push(line);
    }

    // ------------------------------------
    // タプルのベクタ

    let mut s_t_d: Vec<(usize, usize, usize)> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let mut iter = buf.split_whitespace();

        let s: usize = iter.next().unwrap().parse().unwrap();
        let t: usize = iter.next().unwrap().parse().unwrap();
        let d: usize = iter.next().unwrap().parse().unwrap();

        s_t_d.push((s, t, d));
    }

    // ------------------------------------
    // 件数不明

    use std::io::prelude::*;

    let stdin = io::stdin();

    let mut vw = Vec::with_capacity(n);

    for line_result in stdin.lock().lines() {
        let line = line_result.unwrap();

        let mut iter = line.split_whitespace();

        let vi: usize = iter.next().unwrap().parse().unwrap();
        let wi: usize = iter.next().unwrap().parse().unwrap();

        vw.push((vi, wi));
    }

    // ------------------------------------

    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let x: Vec<char> = buf.chars().collect();

    // ------------------------------------
}
