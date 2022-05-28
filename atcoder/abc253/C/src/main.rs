use proconio::fastout;
use proconio::input;
use std::collections::BTreeMap;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut map = BTreeMap::new();

    for _ in 0..q {
        input! {
            query: usize,
        }

        match query {
            1 => {
                input! {
                    x: usize,
                }

                *map.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    x: usize,
                    c: usize,
                }

                let count = map.entry(x).or_insert(0);

                if *count > c {
                    *count -= c;
                } else {
                    map.remove(&x);
                }
            }
            _ => {
                let min_value = *map.keys().next().unwrap();
                let max_value = *map.keys().next_back().unwrap();

                println!("{}", max_value - min_value);
            }
        }
    }
}
