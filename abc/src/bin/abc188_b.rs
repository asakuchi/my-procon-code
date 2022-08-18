use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
        b: [isize; n],
    }

    let mut result = 0;

    for (&a, &b) in a.iter().zip(b.iter()) {
        result += a * b;
    }

    if result == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
