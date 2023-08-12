use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        q: usize,
        t_x_c: [(usize, usize, char); q],
    }

    let mut list: Vec<_> = s.iter().map(|&c| (0, c)).collect();

    let mut last_case = None;

    for i in 1..=q {
        let (t, x, c) = t_x_c[i - 1];

        if t == 1 {
            list[x - 1] = (i, c);
        } else if t == 2 {
            last_case = Some((i, 2));
        } else if t == 3 {
            last_case = Some((i, 3));
        }
    }

    let mut result = Vec::new();

    for i in 0..n {
        let (c_i, c) = list[i];

        if let Some((index, t)) = last_case {
            if c_i < index {
                if t == 2 {
                    result.push(c.to_lowercase().to_string().chars().nth(0).unwrap());
                } else {
                    result.push(c.to_uppercase().to_string().chars().nth(0).unwrap());
                }
            } else {
                result.push(c);
            }
        } else {
            result.push(c);
        }
    }

    let text: String = result.iter().collect();
    println!("{}", text);
}
