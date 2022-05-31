use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
    }

    let mut map = std::collections::HashMap::new();

    let mut max_count = 0;
    let mut max_name = s[0].clone();

    for name in s {
        let count = map.entry(name.clone()).or_insert(0);
        *count += 1;

        if *count > max_count {
            max_count = *count;
            max_name = name.clone();
        }
    }

    println!("{}", max_name);
}
