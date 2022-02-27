use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut set = std::collections::BTreeSet::new();

    for i in 0..q {
        input! {
            c:usize
        }

        match c {
            1 => {
                input! {
                    x :usize,
                }

                set.insert((x, i));
            }
            2 => {
                // 大きい方から
                input! {
                    x :usize,
                    mut k :usize,
                }

                match set.range(..=(x, q)).rev().nth(k - 1) {
                    Some(&value) => {
                        println!("{}", value.0);
                    }
                    None => {
                        println!("{}", -1);
                    }
                }
            }
            _ => {
                // 小さい方から
                input! {
                    x :usize,
                    mut k :usize,
                }

                match set.range((x, 0)..).nth(k - 1) {
                    Some(&value) => {
                        println!("{}", value.0);
                    }
                    None => {
                        println!("{}", -1);
                    }
                }
            }
        }
    }
}
