fn main() {
    let mut n: usize = input_value();
    let (a, b, c) = input_tuple::<usize>();

    // 30 枚使う
    let mut list: Vec<usize> = vec![10 * a, 6 * b, 3 * c];

    list.sort();

    let mut result = 0_usize;

    if n > 100 {
        result += ((n / 30) - 2) * list[2];

        n %= 30;
        n += 60;
    }

    let mut rest = 0;

    // println!("rest n {}", n);

    for i in 0..=30 {
        for j in 0..=30 {
            for k in 0..=30 {
                // println!("{} {} {} {}", i, j, k, i * a + j * b + k * c);

                if i * 3 + j * 5 + k * 10 <= n {
                    rest = rest.max(i * a + j * b + k * c);
                }
            }
        }
    }

    println!("{}", result + rest);
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

fn input_tuple<T>() -> (T, T, T)
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
    let l = iter.next().unwrap().parse().unwrap();

    (n, m, l)
}
