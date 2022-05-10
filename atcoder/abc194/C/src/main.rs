use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let mut list = vec![0; 401];

    for i in 0..n {
        list[(a[i] + 200) as usize] += 1;
    }

    let mut result = 0;

    for i in -200isize..=200 {
        for j in i + 1..=200 {
            let num = list[(i + 200) as usize] * list[(j + 200) as usize] * (i - j).pow(2);
            result += num;
        }
    }

    println!("{}", result);
}
