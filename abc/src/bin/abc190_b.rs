use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        d: usize,
        x_y: [(usize, usize); n],
    }

    if x_y.iter().any(|&(x, y)| x < s && y > d) {
        println!("Yes");
    } else {
        println!("No");
    }
}
