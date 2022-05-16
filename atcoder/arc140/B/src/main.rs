use proconio::fastout;
use proconio::input;
use proconio::marker::Chars;

#[fastout]
fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let mut list = Vec::new();

    let mut mode = 0;

    let mut a_count: usize = 0;
    let mut c_count = 0;

    for c in s {
        match mode {
            1 => {
                // a
                if c == 'A' {
                    a_count += 1;
                } else if c == 'R' {
                    mode = 2;
                } else {
                    mode = 0;
                    a_count = 0;
                    c_count = 0;
                }
            }
            2 => {
                // c
                if c == 'C' {
                    c_count += 1;
                } else {
                    if c_count > 0 {
                        list.push(a_count.min(c_count));
                    }

                    mode = 0;
                    a_count = 0;
                    c_count = 0;

                    if c == 'A' {
                        a_count = 1;
                        mode = 1;
                    }
                }
            }
            _ => {
                if c == 'A' {
                    a_count = 1;
                    mode = 1;
                }
            }
        }
    }

    if mode == 2 {
        if c_count > 0 {
            list.push(a_count.min(c_count));
        }
    }

    let mut result: usize = 0;

    let mut map = std::collections::BTreeMap::new();

    for num in list {
        *map.entry(num).or_insert(0) += 1;
    }

    loop {
        match map.keys().next_back() {
            Some(&max_value) => {
                result += 1;

                let count = map.entry(max_value).or_insert(0);
                *count -= 1;

                if *count == 0 {
                    map.remove(&max_value);
                }

                if max_value > 1 {
                    *map.entry(max_value - 1).or_insert(0) += 1;
                }
            }
            None => {
                break;
            }
        }

        match map.keys().next() {
            Some(&min_value) => {
                result += 1;

                let count = map.entry(min_value).or_insert(0);
                *count -= 1;

                if *count == 0 {
                    map.remove(&min_value);
                }
            }
            None => {
                break;
            }
        }
    }

    println!("{}", result);
}
