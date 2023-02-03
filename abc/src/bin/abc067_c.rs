use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [isize; n],
    }

    let total: isize = a.iter().sum();

    // すぬけくんが0番目のカードだけをとる
    let mut x = a[0];
    let mut y = total - x;
    let mut result = (x - y).abs();

    for i in 1..n - 1 {
        x += a[i];
        y -= a[i];

        result = result.min((x - y).abs());
    }

    println!("{}", result);
}
