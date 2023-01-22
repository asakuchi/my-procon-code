use proconio::{input, marker::Chars};

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize,
        a: Chars,
        b: Chars,
    }

    let mut bigger = Vec::new();
    let mut smaller = Vec::new();

    for i in 0..n {
        let a_num: usize = a[i].to_string().parse().unwrap();
        let b_num: usize = b[i].to_string().parse().unwrap();

        if a_num > b_num {
            bigger.push(a_num);
            smaller.push(b_num);
        } else {
            bigger.push(b_num);
            smaller.push(a_num);
        }
    }

    bigger.reverse();
    smaller.reverse();

    let mut bigger_sum = 0;
    let mut smaller_sum = 0;
    let mut ten_pow = 1;

    for i in 0..n {
        let bigger_num = bigger[i] * ten_pow;
        bigger_sum += bigger_num;
        bigger_sum %= MOD;

        let smaller_num = smaller[i] * ten_pow;
        smaller_sum += smaller_num;
        smaller_sum %= MOD;

        ten_pow *= 10;
        ten_pow %= MOD;
    }

    println!("{}", smaller_sum * bigger_sum % MOD);
}
