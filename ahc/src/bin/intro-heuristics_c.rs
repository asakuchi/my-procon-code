use proconio::{input, marker::Usize1};

struct Input {
    d: usize,
    c: Vec<isize>,
    s: Vec<Vec<isize>>,
}

fn main() {
    input! {
        d: usize,
        c: [isize; 26],
        s: [[isize; 26]; d],
        t: [Usize1; d],
        m: usize,
        d_q: [(Usize1, Usize1); m],
    }

    let input = Input { d, c, s };

    let (mut score, mut last) = calc_score(&input, &t);

    // println!("{:?}", last);

    let mut t_mod = t.clone();

    for (day, q) in d_q {
        score += calc_score_diff(&input, &t_mod, &mut last, day, q);
        t_mod[day] = q;

        println!("{}", score);
        // println!("diff {}", score);
        // println!("{:?}", last);

        // let (refresh_score, refresh_last) = calc_score(&input, &t_mod);

        // println!("refresh {}", refresh_score);
        // println!("{:?}", refresh_last);

        // assert_eq!(last, refresh_last);
    }
}

fn calc_score(input: &Input, t: &Vec<usize>) -> (isize, Vec<Vec<isize>>) {
    let mut score = 0;

    let mut last = vec![vec![0; 26]; input.d];

    for day in 0..input.d {
        let contest_type = t[day];

        score += input.s[day][contest_type];

        if day > 0 {
            last[day] = last[day - 1].clone();
        }

        last[day][contest_type] = day as isize + 1;

        for i in 0..26 {
            score -= input.c[i] * (day as isize + 1 - last[day][i]);
        }

        // println!("{}", score);
    }

    (score, last)
}

fn calc_score_diff(
    input: &Input,
    t: &Vec<usize>,
    last: &mut Vec<Vec<isize>>,
    day: usize,
    contest_type: usize,
) -> isize {
    let mut diff_score = 0;

    let old_contest_type = t[day];

    diff_score -= input.s[day][old_contest_type];
    diff_score += input.s[day][contest_type];

    // 新しいコンテスト
    for next_day in day..input.d {
        if last[next_day][contest_type] == next_day as isize + 1 {
            break;
        }

        // 古いデータを戻す
        diff_score +=
            input.c[contest_type] * (next_day as isize + 1 - last[next_day][contest_type]);

        // contest_type を最後に実施したのは day
        last[next_day][contest_type] = day as isize + 1;

        // 新しいデータで更新
        diff_score -=
            input.c[contest_type] * (next_day as isize + 1 - last[next_day][contest_type]);
    }

    // 古いコンテストをdayにやらないなら、最後に実施したのはいつか -> 前日に記録がある
    let old_contest_exec_day = if day > 0 {
        last[day - 1][old_contest_type]
    } else {
        0
    };

    // eprintln!("old_contest_exec_day {}", old_contest_exec_day);

    // 古いコンテスト

    // 古いデータを戻す
    diff_score += input.c[old_contest_type] * (day as isize + 1 - last[day][old_contest_type]);

    // old_contest_type を最後に実施したのは old_contest_exec_day(+1済み)
    last[day][old_contest_type] = old_contest_exec_day as isize;

    // 新しいデータで更新
    diff_score -= input.c[old_contest_type] * (day as isize + 1 - last[day][old_contest_type]);

    for next_day in day + 1..input.d {
        if last[next_day][old_contest_type] == next_day as isize + 1 {
            break;
        }

        // 古いデータを戻す
        diff_score +=
            input.c[old_contest_type] * (next_day as isize + 1 - last[next_day][old_contest_type]);

        // old_contest_type を最後に実施したのは old_contest_exec_day(+1済み)
        last[next_day][old_contest_type] = old_contest_exec_day as isize;

        // 新しいデータで更新
        diff_score -=
            input.c[old_contest_type] * (next_day as isize + 1 - last[next_day][old_contest_type]);
    }

    diff_score
}
