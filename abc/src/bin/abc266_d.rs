use proconio::input;

fn main() {
    input! {
        n: usize,
        t_x_a: [(usize, usize,usize); n],
    }

    let mut dp = vec![vec![Option::None; 5]; n];

    let mut result = 0;

    let first_time = t_x_a[0].0;

    for next_hole in 0..5 {
        if next_hole > first_time {
            continue;
        }

        let tmp_score = rec(n, &t_x_a, 0, next_hole, &mut dp);

        result = result.max(tmp_score);
    }

    println!("{}", result);
}

fn rec(
    n: usize,
    t_x_a: &Vec<(usize, usize, usize)>,
    time_index: usize,
    current_hole: usize,
    dp: &mut Vec<Vec<Option<usize>>>,
) -> usize {
    if let Some(value) = dp[time_index][current_hole] {
        return value;
    }

    let (t, x, a) = t_x_a[time_index];

    let mut score = if current_hole == x { a } else { 0 };

    let mut next_score = 0;

    if time_index != n - 1 {
        let next_time = t_x_a[time_index + 1].0;
        let diff_time = next_time - t;

        for next_hole in 0..5 {
            if (current_hole as isize - next_hole as isize).abs() > diff_time as isize {
                continue;
            }

            let tmp_score = rec(n, t_x_a, time_index + 1, next_hole, dp);

            next_score = next_score.max(tmp_score);
        }
    }

    score += next_score;

    // dp
    dp[time_index][current_hole] = Some(score);

    score
}
