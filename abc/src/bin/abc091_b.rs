use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [String; n],
        m: usize,
        t: [String; m],
    }

    let mut map = std::collections::HashMap::new();

    for i in 0..n {
        let text = s[i].clone();

        let count = map.entry(text).or_insert(0);
        *count += 1;
    }

    for i in 0..m {
        let text = t[i].clone();
        let count = map.entry(text).or_insert(0);

        if *count > 0 {
            *count -= 1;
        }
    }

    let mut result = 0;

    for (text, yen) in map.iter() {
        result = result.max(*yen);
    }

    println!("{}", result);
}
