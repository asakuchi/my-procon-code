use num_rational::Ratio;
use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut xy: [(isize, isize); n],
    }

    for i in 0..n {
        for j in i + 1..n {
            let (x1, y1) = xy[i];
            let (x2, y2) = xy[j];

            if x1 != x2 {
                let a = Ratio::new(y1 - y2, x1 - x2);
                let b = Ratio::new(y2, 1) - a * x2;

                for k in j + 1..n {
                    let (x3, y3) = xy[k];

                    if Ratio::new(y3, 1) == a * x3 + b {
                        println!("Yes");
                        return;
                    }
                }
            } else {
                for k in j + 1..n {
                    let (x3, _y3) = xy[k];

                    if x1 == x3 {
                        println!("Yes");
                        return;
                    }
                }
            }
        }
    }

    println!("No");
}
