use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    if !s.contains(&'o') || !s.contains(&'-') {
        println!("-1");
        return;
    }

    let list = run_length_encoding(&s);

    let mut result = 0;

    for (c, size) in list {
        if c == 'o' {
            result = result.max(size);
        }
    }

    println!("{}", result);
}

///
/// ランレングス圧縮
///
pub fn run_length_encoding<T>(list: &Vec<T>) -> Vec<(T, usize)>
where
    T: PartialEq + Copy,
{
    let mut result = Vec::new();

    for &value in list.iter() {
        if let Some((next, size)) = result.pop() {
            if next == value {
                result.push((next, size + 1));
            } else {
                result.push((next, size));
                result.push((value, 1));
            }
        } else {
            result.push((value, 1));
        }
    }

    result
}
