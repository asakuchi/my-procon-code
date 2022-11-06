use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut dp = vec![vec![None; 10]; n];

    let result = rec(n, &a, 1, a[0], &mut dp);

    for i in 0..10 {
        println!("{}", result[i]);
    }
}

fn rec(
    n: usize,
    a: &Vec<usize>,
    index: usize,
    current_value: usize,
    dp: &mut Vec<Vec<Option<Vec<usize>>>>,
) -> Vec<usize> {
    if n == index {
        let mut list = vec![0; 10];
        list[current_value] = 1;
        return list;
    }

    if let Some(value) = dp[index][current_value].clone() {
        return value;
    }

    // F
    let result_1 = rec(n, a, index + 1, (current_value + a[index]) % 10, dp);

    // G
    let result_2 = rec(n, a, index + 1, (current_value * a[index]) % 10, dp);

    let mut result = vec![0; 10];

    for i in 0..10 {
        result[i] += result_1[i] + result_2[i];
        result[i] %= 998244353;
    }

    dp[index][current_value] = Some(result.clone());

    result
}
