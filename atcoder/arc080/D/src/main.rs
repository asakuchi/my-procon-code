use proconio::fastout;
use proconio::input;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    pub fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

#[fastout]
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut a: [usize; n],
    }

    let mut result = vec![vec![0; w]; h];

    let mut i = 0;
    let mut j = 0;

    let mut direction = Direction::Right;

    if w == 1 {
        direction = Direction::Down;
    }

    for color in 0..n {
        let mut numbers = vec![color + 1; a[color]];

        while let Some(value) = numbers.pop() {
            result[i][j] = value;

            // eprintln!("{:?} {} {} {:?}", result, i, j, direction);

            match direction {
                Direction::Up => {
                    i -= 1;
                    if i == 0 || result[i - 1][j] != 0 {
                        direction = direction.next();
                    }
                }
                Direction::Right => {
                    j += 1;

                    if j >= w - 1 || result[i][j + 1] != 0 {
                        direction = direction.next();
                    }
                }
                Direction::Down => {
                    i += 1;
                    if i >= h - 1 || result[i + 1][j] != 0 {
                        direction = direction.next();
                    }
                }
                Direction::Left => {
                    if j > 0 {
                        j -= 1;
                    }
                    if j == 0 || result[i][j - 1] != 0 {
                        direction = direction.next();
                    }
                }
            }
        }
    }

    for i in 0..h {
        let line = result[i]
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(" ");

        println!("{}", line);
    }
}
