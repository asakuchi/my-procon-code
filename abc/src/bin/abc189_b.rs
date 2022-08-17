use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        v_p: [(usize, usize); n],
    }

    let mut total = 0;

    for i in 0..n {
        let (v, p) = v_p[i];

        total += v * p;

        if total > x * 100 {
            println!("{}", i + 1);
            return;
        }
    }

    println!("-1");
}
