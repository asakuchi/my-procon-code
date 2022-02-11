fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let x: usize = iter.next().unwrap().parse().unwrap();

    println!("{}", x.pow(3));
}
