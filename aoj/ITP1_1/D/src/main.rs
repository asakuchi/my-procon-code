fn main() {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let mut iter = buf.split_whitespace();

    let s: usize = iter.next().unwrap().parse().unwrap();

    let temp_min = s / 60;
    let seconds = s % 60;

    let hours = temp_min / 60;
    let minutes = temp_min % 60;

    println!("{}:{}:{}", hours, minutes, seconds);
}
