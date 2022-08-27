use proconio::input;

const MOD: isize = 998244353;

fn main() {
    input! {
        n: isize,
    }

    let mut result = n % MOD;

    if result < 0 {
        result += MOD;
    }

    println!("{}", result);
}
