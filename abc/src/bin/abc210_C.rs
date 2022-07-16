use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        k: usize,
        c: [usize; n],
    }

    let mut hash = std::collections::HashMap::new();

    for i in 0..k {
        *hash.entry(c[i]).or_insert(0) += 1;
    }

    let mut result = hash.len();

    for i in k..n {
        let num = hash.entry(c[i - k]).or_insert(0);
        *num -= 1;

        if *num == 0 {
            hash.remove_entry(&c[i - k]);
        }

        let num = hash.entry(c[i]).or_insert(0);
        *num += 1;

        result = result.max(hash.len());
    }

    println!("{}", result);
}
