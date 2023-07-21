fn main() {
    let t = input_value();

    for _ in 0..t {
        solve();
    }
}

fn solve() {
    let (n, k) = input_tuple::<usize>();
    let mut a = input_vec::<usize>();

    a.sort();

    // 番兵
    a.push(3_000_000_000_000_000_000);

    let mut max_renzoku = 1_usize;

    let mut renzoku = 1_usize;

    for i in 1..n + 1 {
        if a[i] - a[i - 1] <= k {
            // 連続
            renzoku += 1;
        } else {
            max_renzoku = max_renzoku.max(renzoku);
            renzoku = 1;
        }
    }

    println!("{}", n - max_renzoku);
}

fn input_value<T>() -> T
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n = buf.parse().unwrap();

    n
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

fn input_vec<T>() -> Vec<T>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let iter = buf.split_whitespace();

    let line = iter.map(|x| x.parse().unwrap()).collect();

    line
}
