use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut tlr: [(usize, f64, f64); n],
    }

    for i in 0..n {
        let (t, _, _) = tlr[i];

        match t {
            2 => {
                tlr[i].2 -= 0.1;
            }
            3 => {
                tlr[i].1 += 0.1;
            }
            4 => {
                tlr[i].1 += 0.1;
                tlr[i].2 -= 0.1;
            }
            _ => {}
        }
    }

    let mut result = 0;

    for i in 0..n {
        for j in i + 1..n {
            let (_, li, ri) = tlr[i];
            let (_, lj, rj) = tlr[j];

            if li <= rj && lj <= ri {
                result += 1;
            }
        }
    }

    println!("{}", result);
}
