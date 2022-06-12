use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [isize; n - 1],
        x: [isize; m],
    }

    let mut b = vec![0; n];

    for i in 1..n {
        b[i] = s[i - 1] - b[i - 1];
    }

    let mut map = std::collections::HashMap::new();

    for i in 0..n {
        for j in 0..m {
            let mut c = x[j] - b[i];
            if i % 2 == 0 {
                c *= -1;
            }

            *map.entry(c).or_insert(0) += 1;
        }
    }

    let mut result = 0;

    for (&_c, &count) in &map {
        result = result.max(count);
    }

    println!("{}", result);
}
