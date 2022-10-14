use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
    }

    let total = a.iter().sum();

    if n >= total {
        println!("{}", n - total);
    } else {
        println!("-1");
    }
}
