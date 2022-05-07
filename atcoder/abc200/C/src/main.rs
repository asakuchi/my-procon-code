use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let a: Vec<_> = a.iter().map(|&num| num % 200).collect();

    let mut map = std::collections::HashMap::new();

    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let mut result: usize = 0;

    for i in 0..n {
        *map.entry(a[i]).or_insert(0) -= 1;
        result += *map.entry(a[i]).or_insert(0);
    }

    println!("{}", result);
}
