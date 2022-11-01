use proconio::input;

const MOD: usize = 1_000_000_007;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut result = 0;

    for i in k..=n + 1 {
        // 初項 0 末項 i-1 項数 i
        let min = (i - 1) * i / 2;

        // 初項 n-i 末項 n 項数 i
        let max = (2 * n - i + 1) * i / 2;

        result += max - min + 1;
        result %= MOD;
    }

    println!("{}", result);
}
