use proconio::input;
use proconio::marker::Chars;
use proconio::marker::Usize1;

fn main() {
    input! {
        r: usize,
        c: usize,
        start: (Usize1,Usize1),
        goal: (Usize1,Usize1),
        mut maze: [Chars; r],
    }

    // println!("{:?}", maze);

    let mut visited = vec![vec![false; c]; r];
    let mut lengths = vec![vec![-1; c]; r];

    let patterns = vec![(1, 0), (0, -1), (-1, 0), (0, 1)];

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(start);

    lengths[start.0][start.1] = 0;

    while let Some(target) = queue.pop_front() {
        // println!("target:{:?} goal:{:?}", target, goal);

        // 終了条件
        if target.0 == goal.0 && target.1 == goal.1 {
            println!("{}", lengths[goal.0][goal.1]);
            return;
        }

        for pattern in patterns.iter() {
            let next = (
                (target.0 as isize + pattern.0) as usize,
                (target.1 as isize + pattern.1) as usize,
            );

            // println!("next:{:?}", next);

            if maze[next.0][next.1] == '#' || visited[next.0][next.1] {
                continue;
            }

            lengths[next.0][next.1] = lengths[target.0][target.1] + 1;
            visited[next.0][next.1] = true;

            queue.push_back(next);
        }
    }

    panic!("goal not found");
}
