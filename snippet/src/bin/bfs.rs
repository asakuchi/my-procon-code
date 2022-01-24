fn main() {
    let move_row = vec![1, -1, 1, -1];
    let move_col = vec![-1, 1, -1, 1];

    let mut queue = std::collections::VecDeque::new();
    queue.push_back(0);

    while let Some(target) = queue.pop_front() {
        for pattern in move_row.iter().zip(move_col.iter()) {
            let next = (target.0 as isize + pattern.0, target.1 as isize + pattern.1);

            // 終了条件
            if 0 > next.0 || next.0 >= height as isize || 0 > next.1 || next.1 >= width as isize {
                continue;
            }

            let next = (next.0 as usize, next.1 as usize);

            if maze[next.0].chars().nth(next.1).unwrap() == 'X' {
                steps[next.0][next.1] = Cell::Unavailable;
                continue;
            }

            // 次へ
            if let Cell::Unexplored = steps[next.0][next.1] {
                if let Cell::Step(step) = steps[target.0][target.1] {
                    steps[next.0][next.1] = Cell::Step(step + 1);
                    queue.push_back(next);
                }
            }
        }
    }
}
