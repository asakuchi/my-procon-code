use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
        t: [Chars; n],
    }

    let mut t_points = Vec::new();

    for i in 0..n {
        for j in 0..n {
            if t[i][j] == '#' {
                t_points.push((i as isize, j as isize));
            }
        }
    }

    t_points.sort();

    let mut s_points = Vec::new();

    for i in 0..n {
        for j in 0..n {
            if s[i][j] == '#' {
                s_points.push((i as isize, j as isize));
            }
        }
    }

    for _ in 0..4 {
        s_points.sort();

        let diff = (s_points[0].0 - t_points[0].0, s_points[0].1 - t_points[0].1);

        let compare_points: Vec<_> = s_points
            .iter()
            .map(|&(x, y)| (x - diff.0, y - diff.1))
            .collect();

        if t_points == compare_points {
            println!("Yes");
            return;
        }

        s_points = s_points.iter().map(|&(x, y)| (y, -x)).collect();
    }

    println!("No");
}
