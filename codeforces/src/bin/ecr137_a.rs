fn main() {
    let t = input_usize();

    for _ in 0..t {
        let _n = input_usize();

        let a = input_vec();

        let mut nums = vec![];

        'search: for i in 0..=9 {
            for &value in &a {
                if value == i {
                    continue 'search;
                }
            }

            nums.push(i);
        }

        let result = nums.len() * (nums.len() - 1) * 6 / 2;

        println!("{}", result);
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

fn input_vec() -> Vec<usize> {
    let stdin = std::io::stdin();

    let mut buf = String::new();
    stdin.read_line(&mut buf).unwrap();
    buf = buf.trim_end().to_owned();

    let iter = buf.split_whitespace();

    let line: Vec<_> = iter.map(|x| x.parse().unwrap()).collect();

    line
}
