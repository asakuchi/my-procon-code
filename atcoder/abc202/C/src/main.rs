use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
    }

    let mut map = std::collections::HashMap::new();

    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let mut result: usize = 0;

    for j in 0..n {
        if let Some(count) = map.get(&b[c[j] - 1]) {
            result += count;
        }
    }

    println!("{}", result);
}
