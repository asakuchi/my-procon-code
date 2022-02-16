use std::io;

fn main() {
    // ------------------------------------
    let stdin = io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n: usize = buf.parse().unwrap();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let a: Vec<_> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let _q: usize = buf.parse().unwrap();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let m: Vec<_> = buf.split_whitespace().map(|x| x.parse().unwrap()).collect();

    // ------------------------------------

    for value in m {
        let result = rec(&a, n, 0, value);
        println!("{}", if result { "yes" } else { "no" });
    }
}

fn rec(a: &Vec<isize>, n: usize, i: usize, m: isize) -> bool {
    if m == 0 {
        return true;
    }

    if i == n {
        return false;
    }

    rec(a, n, i + 1, m) || rec(a, n, i + 1, m - a[i])
}
