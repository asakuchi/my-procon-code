fn main() {
    let n: isize = input_value();

    let mut left = 1;
    let mut right = n;

    // 必ず
    // left_value は 0
    // right_value は 1

    while (left - right).abs() > 1 {
        let mid = (left + right) / 2;

        println!("? {}", mid);

        let mid_value: isize = input_value();

        if mid_value == 1 {
            right = mid;
        } else {
            left = mid;
        }
    }

    println!("! {}", left);
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
