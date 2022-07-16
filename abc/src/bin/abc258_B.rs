use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: isize,
        a: [Chars; n],
    }

    let mut result: usize = 0;

    for s_i in 0..n {
        for s_j in 0..n {
            let start = (s_i, s_j);

            let directions = vec![
                (0, 1),
                (0, -1),
                (1, 0),
                (-1, 0),
                (1, 1),
                (-1, -1),
                (1, -1),
                (-1, 1),
            ];

            for direction in directions {
                let mut points = Vec::new();

                for diff in 1..=n {
                    let mut point_1 = (start.0 + direction.0 * diff, start.1 + direction.1 * diff);
                    if point_1.0 >= n {
                        point_1.0 = point_1.0 - n;
                    }
                    if point_1.1 >= n {
                        point_1.1 = point_1.1 - n;
                    }

                    if point_1.1 < 0 {
                        point_1.1 = point_1.1 + n;
                    }
                    if point_1.0 < 0 {
                        point_1.0 = point_1.0 + n;
                    }

                    points.push(a[point_1.0 as usize][point_1.1 as usize]);
                }

                let text = points
                    .iter()
                    .map(|c| c.to_string())
                    .collect::<Vec<_>>()
                    .join("");
                let num = text.parse().unwrap();

                if num > result {
                    result = num;
                }
            }
        }
    }

    println!("{}", result);
}
