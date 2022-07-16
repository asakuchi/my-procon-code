use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        a: (Usize1, Usize1),
        b: (Usize1, Usize1),
        s: [Chars; n],
    }

    if (a.0 + a.1) % 2 != (b.0 + b.1) % 2 {
        println!("-1");
        return;
    }

    let mut visited = vec![vec![false; n]; n];

    let mut queue = std::collections::VecDeque::new();
    queue.push_back((a, 0));
    visited[a.0][a.1] = true;

    while let Some(((i, j), count)) = queue.pop_front() {
        if b == (i, j) {
            println!("{}", count);
            return;
        }

        visited[i][j] = true;

        for (direction_x, direction_y) in vec![(1, 1), (1, -1), (-1, 1), (-1, -1)] {
            for d in 1..=n as isize {
                let (next_x, next_y) = (i as isize + d * direction_x, j as isize + d * direction_y);

                if next_x < 0 || next_y < 0 || next_x >= n as isize || next_y >= n as isize {
                    break;
                }

                let (next_x, next_y) = (next_x as usize, next_y as usize);

                if visited[next_x][next_y] {
                    continue;
                }

                if s[next_x][next_y] == '#' {
                    break;
                }

                visited[next_x][next_y] = true;

                queue.push_back(((next_x, next_y), count + 1));
            }
        }
    }

    println!("-1");
}
