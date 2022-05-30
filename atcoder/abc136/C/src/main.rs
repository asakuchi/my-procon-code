use proconio::fastout;
use proconio::input;

#[fastout]
fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
    }

    a[0] -= 1;

    for i in 1..n {
        if a[i] < a[i - 1] {
            println!("No");
            return;
        }

        if a[i - 1] != a[i] {
            a[i] -= 1;
        }
    }

    println!("Yes");
}
