use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        mut n: usize,
        c: [usize; 9],
    }

    let mut c_min = c[8];
    let mut min_i = 9;

    for i in (1..=9).rev() {
        if c[i - 1] < c_min {
            c_min = c[i - 1];
            min_i = i;
        }
    }

    let max_digit = n / c_min;

    n %= c_min;

    let mut x = vec![min_i; max_digit];

    for digit in 0..max_digit {
        for i in (1..=9).rev() {
            if i > x[digit] && n >= c[i - 1] - c[min_i - 1] {
                x[digit] = i;
                n -= c[i - 1] - c[min_i - 1];
                break;
            }
        }

        if n == 0 {
            break;
        }
    }

    let result = x
        .iter()
        .map(|num| num.to_string())
        .collect::<Vec<_>>()
        .join("");

    println!("{}", result);
}
