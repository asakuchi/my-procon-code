use ac_library_rs::crt;
use itertools::Itertools;

fn main() {
    let mut m = 0;

    let primes = vec![4, 9, 5, 7, 11, 13, 17, 19, 23];
    // let primes = vec![2i64, 3];

    for &prime in &primes {
        m += prime as usize;
    }

    println!("{}", m);

    let mut a = Vec::with_capacity(m);

    let mut i = 1;

    for &prime in &primes {
        let first_i = i;

        for _ in 0..prime - 1 {
            a.push(i + 1);
            i += 1;
        }

        a.push(first_i);
        i += 1;
    }

    println!("{}", a.iter().format(" "));

    let b: Vec<i64> = input_vec();

    let mut rem_list = Vec::new();

    let mut index = 0;

    for &prime in &primes {
        rem_list.push((b[index] - index as i64 - 1) % prime);

        index += prime as usize;
    }

    // println!("{:?}", rem_list);
    // println!("{:?}", primes);

    let result = crt(&rem_list, &primes);

    println!("{}", result.0);
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
