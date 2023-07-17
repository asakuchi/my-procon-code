fn main() {
    let t = input_value();

    'loop_testcase: for _ in 0..t {
        let (_n, _m, k) = input_tuple3();
        let (x, y) = input_tuple::<usize>();
        let list = input_tuple_vec::<usize>(k);

        for i in 0..k {
            let (f_x, f_y) = list[i];

            if (x + y) % 2 == (f_x + f_y) % 2 {
                println!("NO");
                continue 'loop_testcase;
            }
        }

        println!("YES");
    }
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

fn input_tuple3<T>() -> (T, T, T)
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
    let k = iter.next().unwrap().parse().unwrap();

    (n, m, k)
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

fn input_tuple_vec<T>(n: usize) -> Vec<(T, T)>
where
    T: std::str::FromStr,
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    // タプルのベクタ

    let stdin = std::io::stdin();

    let mut s_t_d = Vec::with_capacity(n);

    for _ in 0..n {
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        buf = buf.trim_end().to_owned();

        let mut iter = buf.split_whitespace();

        let s = iter.next().unwrap().parse().unwrap();
        let t = iter.next().unwrap().parse().unwrap();
        // let d = iter.next().unwrap().parse().unwrap();

        s_t_d.push((s, t));
    }

    s_t_d
}
