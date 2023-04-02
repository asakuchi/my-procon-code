use num::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut result = std::usize::MAX;

    for a in 1..=n {
        let b = m.div_ceil(&a);

        if a > b {
            break;
        }

        if b > n {
            continue;
        }

        result = result.min(a * b);

        // println!("a {} b {} a*b {}", a, b, a * b);
    }

    if result == std::usize::MAX {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
