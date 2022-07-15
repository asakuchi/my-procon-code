use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let mut sum = 0;

    for i in 0..n {
        if i % 2 == 1 {
            sum += a[i] - 1;
        } else {
            sum += a[i];
        }
    }

    if sum <= x {
        println!("Yes");
    } else {
        println!("No");
    }
}
