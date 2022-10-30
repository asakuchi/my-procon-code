use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut max_i = 0;
    let mut max_value = h[0];

    for i in 0..n {
        if h[i] > max_value {
            max_value = h[i];
            max_i = i;
        }
    }

    println!("{}", max_i + 1);
}
