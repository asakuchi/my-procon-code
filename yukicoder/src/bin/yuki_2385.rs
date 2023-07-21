fn main() {
    let q = input_value();

    for _ in 0..q {
        let s = input_string();

        if s.starts_with("0b") {
            let num = usize::from_str_radix(&s[2..], 2);
            println!("{}", num.unwrap());
        } else if s.starts_with("0o") {
            let num = usize::from_str_radix(&s[2..], 8);
            println!("{}", num.unwrap());
        } else if s.starts_with("0x") {
            let num = usize::from_str_radix(&s[2..], 16);
            println!("{}", num.unwrap());
        } else {
            println!("{}", s);
        }
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

fn input_string() -> String {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    buf
}
