use proconio::input;

fn main() {
    input! {
        n: usize,
        h: usize,
        x: usize,
        a: [usize; n],
    }

    for i in 0..n {
        if h + a[i] >= x {
            println!("{}", i + 1);
            return;
        }
    }
}
