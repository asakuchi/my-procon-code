use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [usize; n],
    }

    let mut result = 0;

    let mut current = 0;

    for i in 0..n - 1 {
        if h[i] >= h[i + 1] {
            current += 1;

            result = result.max(current);
        } else {
            current = 0;
        }
    }

    println!("{}", result);
}
