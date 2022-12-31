use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut result = 0;

    let mut renban = 1;

    for i in 0..n {
        if a[i] == renban {
            renban += 1;
        } else {
            result += 1;
        }
    }

    if renban == 1 {
        println!("-1");
    } else {
        println!("{}", result);
    }
}
