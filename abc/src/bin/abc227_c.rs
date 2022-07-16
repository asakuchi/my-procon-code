use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: isize,
    }

    let mut result: isize = 0;

    let mut a = 1;

    while a * a * a <= n {
        let mut b = a;

        while a * b * b <= n {
            let count = n / (a * b) as isize - b as isize + 1;
            result += count;

            b += 1;
        }

        a += 1;
    }

    println!("{}", result);
}
