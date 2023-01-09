use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    // (n,1) に移動した場合
    let mut result = n - 1;

    let mut i = 1;

    while i * i <= n {
        if n % i == 0 {
            let j = n / i;
            result = result.min(i + j - 2);
        }

        i += 1;
    }

    println!("{}", result);
}
