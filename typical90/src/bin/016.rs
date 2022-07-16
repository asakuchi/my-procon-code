use proconio::input;

fn main() {
    input! {
        n: i64,
        a: i64,
        b: i64,
        c: i64,
    }

    let mut answer = 1i64 << 30;

    for i in 0..=9999 {
        for j in 0..=9999 - i {
            let v = n - 1i64 * i * a - 1i64 * j * b;
            let r = 1i64 * i + 1i64 * j + v / c;

            if v % c != 0i64 || v < 0i64 || r > 9999i64 {
                continue;
            }

            answer = std::cmp::min(answer, r);
        }
    }

    println!("{}", answer);
}
