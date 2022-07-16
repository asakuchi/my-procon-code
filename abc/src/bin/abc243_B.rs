use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    }

    let mut result_1 = 0;
    let mut result_2 = 0;

    for i in 0..n {
        if a[i] == b[i] {
            result_1 += 1;
        }

        for j in 0..n {
            if i != j {
                if a[i] == b[j] {
                    result_2 += 1;
                }
            }
        }
    }

    println!("{}", result_1);
    println!("{}", result_2);
}
