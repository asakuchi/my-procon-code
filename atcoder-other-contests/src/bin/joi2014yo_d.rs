use proconio::{input, marker::Chars};

const MOD: usize = 10007;

fn main() {
    input! {
        n: usize,
        s: Chars
    }

    let person: Vec<_> = s
        .iter()
        .map(|c| match c {
            'J' => 0,
            'O' => 1,
            _ => 2,
        })
        .collect();

    let mut dp = vec![vec![None; 9]; n];

    // 最初はJ君が鍵を持っているため、-1にJ君が参加したことにする
    let prev_person = 1;

    let result = rec(n, &person, prev_person, 0, &mut dp);

    println!("{}", result);
}

fn rec(
    n: usize,
    person: &Vec<usize>,
    prev: usize,
    day: usize,
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if day == n {
        return 1;
    }

    if let Some(value) = dp[day][prev] {
        return value;
    }

    let mut result = 0;

    for mask in 0..1 << 3 {
        // 責任者がいること
        if mask & 1 << person[day] == 0 {
            continue;
        }

        // 前日の参加者の誰かがいること（鍵を持っている）
        if prev & mask == 0 {
            continue;
        }

        result += rec(n, person, mask, day + 1, dp);
        result %= MOD;
    }

    dp[day][prev] = Some(result);

    result
}
