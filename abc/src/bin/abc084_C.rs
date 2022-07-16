use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut csf: [(usize, usize,usize); n - 1],
    }

    for i in 0..n - 1 {
        let mut result = 0;
        for j in i..n - 1 {
            let (c, s, f) = csf[j];

            result = result.max(s);

            if result % f != 0 {
                result -= result % f;
                result += f;
            }

            result += c;
        }
        println!("{}", result);
    }

    println!("0");
}
