use itertools::Itertools;

fn main() {
    let (n, k): (usize, usize) = input_tuple();

    if k == 1 {
        let mut a = vec![0; n + 1];

        for i in 1..=n {
            println!("? {}", i);

            let t: isize = input_value();

            if t == -1 {
                return;
            }

            a[i] = t;
        }

        println!("! {}", a[1..=n].iter().format(" "));
        return;
    }

    let mut a = vec![0; n + 1];

    // 問い合わせの結果
    let mut list = vec![0; n + 1];

    let mut all_k = 0;

    for i in 1..=k + 1 {
        let mut query = Vec::new();

        for j in 1..=k + 1 {
            if i == j {
                continue;
            }

            query.push(j);
        }

        println!("? {}", query.iter().format(" "));

        let t: isize = input_value();

        if t == -1 {
            return;
        }

        list[i] = t;

        all_k ^= t;
    }

    for i in 1..=k + 1 {
        a[i] = all_k ^ list[i];
    }

    let mut prev = list[1];

    for i in k + 2..=n {
        let query = (i - k + 1..=i).collect::<Vec<_>>();

        println!("? {}", query.iter().format(" "));

        let t: isize = input_value();

        if t == -1 {
            return;
        }

        a[i] = prev ^ t ^ a[i - k];

        prev = t;
    }

    let text = a[1..=n].iter().format(" ");

    println!("! {}", text);
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
