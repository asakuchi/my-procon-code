use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut dp = vec![vec![None; w]; h];

    let result = rec(h, w, &a, 0, 0, &mut dp);

    if result == 0 {
        println!("Draw");
    } else if result > 0 {
        println!("Takahashi");
    } else {
        println!("Aoki");
    }
}

fn rec(
    h: usize,
    w: usize,
    a: &Vec<Vec<char>>,
    i: usize,
    j: usize,
    dp: &mut Vec<Vec<Option<isize>>>,
) -> isize {
    if let Some(value) = dp[i][j] {
        return value;
    }

    let taka_turn = (i + j) % 2 == 0;

    // コマを下に動かす
    let score_1 = if i == h - 1 {
        None
    } else {
        let next = if a[i + 1][j] == '+' { 1 } else { -1 };

        let next = if taka_turn { next } else { -next };

        let value = rec(h, w, a, i + 1, j, dp) + next;

        Some(value)
    };

    // コマを右に動かす
    let score_2 = if j == w - 1 {
        None
    } else {
        let next = if a[i][j + 1] == '+' { 1 } else { -1 };

        let next = if taka_turn { next } else { -next };

        let value = rec(h, w, a, i, j + 1, dp) + next;

        Some(value)
    };

    let result = if taka_turn {
        if let Some(score) = score_1.max(score_2) {
            score
        } else {
            0
        }
    } else {
        // score_1.min(score_2)
        if let Some(&score) = score_1.iter().chain(score_2.iter()).min() {
            score
        } else {
            0
        }
    };

    // println!(
    //     "i {} j {} taka {} s_1 {:?} s_2 {:?}",
    //     i, j, taka_turn, score_1, score_2
    // );

    // println!("i {} j {} taka {} result {}", i, j, taka_turn, result);

    dp[i][j] = Some(result);

    result
}
