fn main() {
    let t = input_usize();

    for _ in 0..t {
        let n = input_usize();

        let mut result = Vec::new();

        result.push(1);

        for i in (2..=n).rev() {
            result.push(i);
        }

        let text: String = result
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        println!("{}", text);
    }
}

fn input_usize() -> usize {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let n: usize = buf.parse().unwrap();

    n
}
