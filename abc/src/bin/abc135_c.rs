use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n + 1],
        mut b: [usize; n],
    }

    // 街 n+1 は勇者 n しか守れない

    let mut result = 0;

    for i in 0..n {
        for j in 0..=1 {
            if a[i + j] > 0 {
                if b[i] > a[i + j] {
                    b[i] -= a[i + j];

                    result += a[i + j];

                    a[i + j] = 0;
                } else {
                    a[i + j] -= b[i];

                    result += b[i];

                    b[i] = 0;
                }
            }
        }
    }

    println!("{}", result);
}
