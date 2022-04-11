use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut a = 1;

    let mut result = a.to_string().len().max((n / a).to_string().len());

    while a * a <= n {
        if n % a == 0 {
            let f = a.to_string().len().max((n / a).to_string().len());

            result = result.min(f);
        }

        a += 1;
    }

    println!("{}", result);
}
