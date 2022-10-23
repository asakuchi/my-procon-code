use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
    }

    let sum: usize = a.iter().sum();

    let mut count = 0;

    for value in a {
        if value * 4 * m >= sum {
            count += 1;
        }
    }

    if count >= m {
        println!("Yes");
    } else {
        println!("No");
    }
}
