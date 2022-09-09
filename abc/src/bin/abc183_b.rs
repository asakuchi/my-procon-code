use proconio::input;

fn main() {
    input! {
        s: (f64, f64),
        g: (f64, f64),
    }

    // y = a * x + b

    let a = (g.1 + s.1) / (g.0 - s.0);
    let b = g.1 - g.0 * a;

    // y = 0 の場合
    let x = -b / a;

    println!("{}", x);
}
