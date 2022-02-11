use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
    }

    let mut result = 0;

    for i in (1..=n).step_by(2) {
        let mut yakusu = 0;

        for j in 1..=i {
            if i % j == 0 {
                yakusu += 1;
            }
        }

        if yakusu == 8 {
            result += 1;
        }
    }

    println!("{}", result);
}
