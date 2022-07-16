use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a.sort();

    let mut map = std::collections::HashMap::new();

    for &num in a.iter() {
        *map.entry(num).or_insert(0) += 1;
    }

    let mut result: usize = 0;

    let m = 200_001;

    for a in 1..m {
        for b in 1..=m / a {
            let c = a * b;

            result += *map.entry(a).or_insert(0)
                * *map.entry(b).or_insert(0)
                * *map.entry(c).or_insert(0);
        }
    }

    println!("{}", result);
}
