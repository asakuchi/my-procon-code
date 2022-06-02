use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = 0;

    for i in 0..n {
        let mut num = a[i];

        while num % 2 == 0 {
            result += 1;
            num /= 2;
        }
    }

    println!("{}", result);
}
