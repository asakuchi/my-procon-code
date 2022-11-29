use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        mut a: usize,
        mut b: usize,
        k: usize,
    }

    let mut nth = k;
    let n = a + b;

    let mut result = Vec::new();

    for _i in 1..=n {
        // println!("i {} a {} b {}", i, a, b);

        if a == 0 {
            result.push('b');
            continue;
        }

        if b == 0 {
            result.push('a');
            continue;
        }

        // i文字眼a の場合
        let a_pattern = combination(a + b - 1, a - 1);

        // // b の場合
        // let b_pattern = combination(a + b - 1, a);

        // println!("nth {}", nth);
        // println!("a_pat {}", a_pattern);
        // println!("b_pat {}", b_pattern);

        if nth > a_pattern {
            nth -= a_pattern;
            result.push('b');

            b -= 1;
        } else {
            result.push('a');

            a -= 1;
        }

        // println!("{:?}", result);
    }

    // println!("{:?}", result);

    let text = result.iter().join("");

    println!("{}", text);
}

fn combination(n: usize, k: usize) -> usize {
    let mut result = 1;

    for i in 0..k {
        result *= n - i;
        result /= i + 1;
    }

    result
}
