fn main() {
    input_usize();
    input_tuple();
    input_vec_2d(10);
    input_tuple_vec(10);
    input_char_vec();
}

fn input_usize() -> usize {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n: usize = buf.parse().unwrap();

    n
}

fn input_tuple() -> (usize, usize) {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    (n, m)
}

fn input_vec_2d(n: usize) -> Vec<Vec<isize>> {
    // 二次元ベクタ

    let stdin = std::io::stdin();

    let mut a: Vec<Vec<isize>> = Vec::with_capacity(n);

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let iter = buf.split_whitespace();

        let line: Vec<isize> = iter.map(|x| x.parse().unwrap()).collect();

        a.push(line);
    }

    a
}

fn input_tuple_vec(n: usize) -> Vec<(usize, usize, usize)> {
    // タプルのベクタ

    let stdin = std::io::stdin();

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

    s_t_d
}

fn input_char_vec() -> Vec<char> {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let x: Vec<char> = buf.chars().collect();

    x
}
