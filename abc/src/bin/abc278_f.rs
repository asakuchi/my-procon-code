use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut list = vec![Vec::new(); n];

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }

            if s[i][s[i].len() - 1] == s[j][0] {
                list[i].push(j);
            }
        }
    }

    let mut has_first_win = false;

    for i in 0..n {
        let mut dp = vec![vec![None; 1 << n]; n];

        if rec(n, &list, i, 1 << i, &mut dp) == 0 {
            has_first_win = true;
            break;
        }
    }

    if has_first_win {
        println!("First");
    } else {
        println!("Second");
    }
}

fn rec(
    n: usize,
    list: &Vec<Vec<usize>>,
    current: usize,
    visited: usize,
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if let Some(value) = dp[current][visited] {
        return value;
    }

    let mut has_first_win = false;
    let mut has_second_win = false;

    let mut current_player = 0;

    for i in 0..n {
        if visited & 1 << i > 0 {
            current_player += 1;
            current_player %= 2;
        }
    }

    for &next in list[current].iter() {
        if visited & 1 << next > 0 {
            continue;
        }

        let win_player = rec(n, list, next, visited | 1 << next, dp);

        if win_player == 0 {
            has_first_win = true;
        } else {
            has_second_win = true;
        }
    }

    let result;

    if current_player == 0 {
        if has_first_win {
            result = 0;
        } else {
            result = 1;
        }
    } else {
        if has_second_win {
            result = 1;
        } else {
            result = 0;
        }
    }

    dp[current][visited] = Some(result);

    result
}
