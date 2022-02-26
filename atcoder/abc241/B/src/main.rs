use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }

    let mut hash = std::collections::HashMap::new();

    for value in a {
        *hash.entry(value).or_insert(0) += 1;
    }

    for value in b {
        let count = hash.entry(value).or_insert(0);

        if *count > 0 {
            *count -= 1;
            continue;
        } else {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
