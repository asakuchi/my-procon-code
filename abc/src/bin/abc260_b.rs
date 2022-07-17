use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
        z: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut result = Vec::new();

    let mut list = a
        .iter()
        .zip(b.iter())
        .enumerate()
        .map(|(i, (&a, &b))| (i, a, b, a + b))
        .collect::<Vec<_>>();

    list.sort_by_key(|v| (v.1, std::cmp::Reverse(v.0)));

    for _ in 0..x {
        if let Some(value) = list.pop() {
            result.push(value.0);
        }
    }

    list.sort_by_key(|v| (v.2, std::cmp::Reverse(v.0)));

    for _ in 0..y {
        if let Some(value) = list.pop() {
            result.push(value.0);
        }
    }

    list.sort_by_key(|v| (v.3, std::cmp::Reverse(v.0)));

    for _ in 0..z {
        if let Some(value) = list.pop() {
            result.push(value.0);
        }
    }

    result.sort();

    for i in 0..result.len() {
        println!("{}", result[i] + 1);
    }
}
