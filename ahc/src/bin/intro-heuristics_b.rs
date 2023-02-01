use proconio::{input, marker::Usize1};

fn main() {
    input! {
        d: usize,
        c: [isize; 26],
        s: [[isize; 26]; d],
        t: [Usize1; d],
    }

    let _score = calc_score(d, &c, &s, &t);
}

fn calc_score(d: usize, c: &Vec<isize>, s: &Vec<Vec<isize>>, t: &Vec<usize>) -> isize {
    let mut score = 0;

    let mut last = vec![vec![0; 26]; d];

    for day in 0..d {
        let contest_type = t[day];

        score += s[day][contest_type];

        if day > 0 {
            last[day] = last[day - 1].clone();
        }

        last[day][contest_type] = day as isize + 1;

        for i in 0..26 {
            score -= c[i] * (day as isize + 1 - last[day][i]);
        }

        println!("{}", score);
    }

    score
}
