fn main() {
    let (n, m) = input_tuple();

    let mut visited = vec![false; n];

    visited[0] = true;

    rec(n, m, &mut visited, 0, 0);
}

fn rec(n: usize, m: usize, visited: &mut Vec<bool>, current: usize, parrent: usize) -> bool {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    if buf == "-1" || buf == "OK" {
        return true;
    }

    let mut iter = buf.split_whitespace();
    iter.next();

    let mut v = Vec::new();

    for x in iter {
        v.push(x.to_string().parse::<usize>().unwrap() - 1);
    }

    for &next in &v {
        if visited[next] {
            continue;
        }

        visited[next] = true;

        println!("{}", next + 1);

        if rec(n, m, visited, next, current) {
            return true;
        }
    }

    println!("{}", parrent + 1);

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();

    false
}

fn input_tuple<T>() -> (T, T)
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let n = iter.next().unwrap().parse().unwrap();
    let m = iter.next().unwrap().parse().unwrap();

    (n, m)
}
