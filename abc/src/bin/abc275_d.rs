use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut dp = HashMap::new();

    println!("{}", f(n, &mut dp));
}

fn f(x: usize, dp: &mut HashMap<usize, usize>) -> usize {
    if x == 0 {
        return 1;
    }

    if let Some(&value) = dp.get(&x) {
        return value;
    }

    let result = f(x / 2, dp) + f(x / 3, dp);

    dp.insert(x, result);

    result
}
