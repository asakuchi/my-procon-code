use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        q: usize,
    }

    let mut hash = std::collections::BTreeMap::new();

    for _ in 0..q {
        input! {
            c:usize
        }

        match c {
            1 => {
                input! {
                    x :usize,
                }

                *hash.entry(x).or_insert(0) += 1;
            }
            2 => {
                // 大きい方から
                input! {
                    x :usize,
                    mut k :usize,
                }

                let mut result = -1;

                'search: for (&key, &value) in hash.range(..=x).rev() {
                    if k <= value {
                        result = key as isize;
                        break 'search;
                    } else {
                        k -= value;
                    }
                }

                println!("{}", result);
            }
            _ => {
                // 小さい方から
                input! {
                    x :usize,
                    mut k :usize,
                }

                let mut result = -1;

                'search2: for (&key, &value) in hash.range(x..) {
                    if k <= value {
                        result = key as isize;
                        break 'search2;
                    } else {
                        k -= value;
                    }
                }

                println!("{}", result);
            }
        }
    }
}
