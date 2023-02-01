use proconio::input;
// use rand::Rng;
use std::collections::BinaryHeap;
// use std::collections::VecDeque;
use rand::seq::SliceRandom;
use std::time::Instant;

struct Input {
    d: usize,
    c: Vec<isize>,
    s: Vec<Vec<isize>>,
}

fn main() {
    let start = Instant::now();

    let mut rng = rand::thread_rng();

    input! {
        d: usize,
        c: [isize; 26],
        s: [[isize; 26]; d],
        // t: [Usize1; d],
        // m: usize,
        // d_q: [(Usize1, Usize1); m],
    }

    let input = Input { d, c, s };

    let t = init(&input);

    let (score, last) = calc_score(&input, &t);

    let mut best_score = score;
    let mut best_t = t.clone();

    // 候補を変えていく
    let mut priority_queue = BinaryHeap::new();
    priority_queue.push((score, t, last));

    while let Some((score, t, last)) = priority_queue.pop() {
        let end = start.elapsed();
        if end.as_millis() >= 1_800 {
            // eprintln!("timeup");
            break;
        }

        // eprintln!("score {}", score);

        if score > best_score {
            // eprintln!("score update! {}", score);

            best_score = score;
            best_t = t.clone();
        }

        // ランダム
        let mut choice_day = (0..d).collect::<Vec<_>>();
        let mut choice_contest = (0..26).collect::<Vec<_>>();

        let mut found_count = 0;

        while found_count < 3 {
            let end = start.elapsed();
            if end.as_millis() >= 1_800 {
                // eprintln!("timeup");

                break;
            }

            choice_day.shuffle(&mut rng);
            choice_contest.shuffle(&mut rng);

            let day = choice_day[0];
            let q = choice_contest[0];

            let mut last_mod = last.clone();

            let score_diff = calc_score_diff(&input, &t, &mut last_mod, day, q);

            // eprintln!("searching... {}", score_diff);

            if score_diff > 0 {
                let mut t_mod = t.clone();
                t_mod[day] = q;

                priority_queue.push((score + score_diff, t_mod, last_mod));

                found_count += 1;
            }
        }

        // println!("{}", score);
    }

    for x in best_t {
        println!("{}", x + 1);
    }
}

fn init(input: &Input) -> Vec<usize> {
    let mut t = vec![0; input.d];

    let mut last = vec![vec![0; 26]; input.d];

    for day in 0..input.d {
        let mut max_score = -1_000_000_000_000_000_000;
        let mut best_contest = 0;

        for contest_type in 0..26 {
            // let contest_type = t[day];

            let mut score = input.s[day][contest_type];

            if day > 0 {
                last[day] = last[day - 1].clone();
            }

            for i in 0..26 {
                if i != contest_type {
                    score -= input.c[i] * (day as isize + 1 - last[day][i]);
                }
            }

            if score > max_score {
                max_score = score;
                best_contest = contest_type;
            }
        }

        last[day][best_contest] = day as isize + 1;

        t[day] = best_contest;
        // println!("{}", score);
    }

    t
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
